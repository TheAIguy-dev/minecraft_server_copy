use std::collections::HashMap;
use std::{fs, time::Duration};

use bracket_noise::prelude::{FastNoise, NoiseType};
use eyre::{eyre, Context, Result};
use fastnbt::nbt;
use lazy_static::{initialize, lazy_static};
use log::{debug, info};
use sha256::digest;

use tokio::sync::mpsc::error::TryRecvError;
use tokio::time::{interval, Instant, Interval};
use tokio::{
    self,
    net::TcpListener,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_util::bytes::Buf;

use crate::config::{get_config, Config};
use crate::server::types::Position;
use crate::server::util::{get_chunk, normalize_yaw};

use net::{
    connection_manager, Connection, IncomingPacket, OutgoingPacket, WrappedConnectionState,
    WrappedIncomingPacket, WrappedOutgoingPacket,
};
use state::ConnectionState;
use types::{Chunk, Gamemode, PlayerInfoUpdateActions, Uuid};

pub mod net;
pub mod state;
#[allow(dead_code)]
pub mod types;
pub mod util;

pub const SEED: i64 = 0;
pub const VIEW_DISTANCE: i32 = 16;
pub const SIMULATION_DISTANCE: i32 = 0; // Technically the truth
pub const IS_HARDCORE: bool = false;
pub const REDUCED_DEBUG_INFO: bool = false;
pub const ENABLE_RESPAWN_SCREEN: bool = true;
pub const IS_DEBUG: bool = false;
pub const IS_FLAT: bool = true;
pub const DEFAULT_SPAWN_POSITION: (f64, f64, f64) = (0.5, 65.0, 0.5);

lazy_static! {
    pub static ref REGISTRY_CODEC: Vec<u8> = fs::read("registry_codec.nbt").unwrap();
    pub static ref NOISE: FastNoise = {
        let mut noise: FastNoise = FastNoise::seeded(SEED as u64);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_frequency(0.005);
        noise
    };
    pub static ref HASHED_SEED: i64 = digest(&SEED.to_be_bytes()).as_bytes().get_i64();
}

// #[allow(clippy::enum_variant_names)]
// #[derive(Clone, Copy, Debug)]
// enum Event {
//     PlayerJoin(i32),
//     PlayerMove(i32, i16, i16, i16),
//     PlayerQuit(i32),
// }

#[derive(Clone, Debug)]
struct Player {
    pub name: String,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            uuid: Uuid::offline(name),
            x: DEFAULT_SPAWN_POSITION.0,
            y: DEFAULT_SPAWN_POSITION.1,
            z: DEFAULT_SPAWN_POSITION.2,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,
        }
    }

    pub fn get_block(&self) -> (i32, i32, i32) {
        (
            self.x.floor() as i32,
            self.y.floor() as i32,
            self.z.floor() as i32,
        )
    }

    pub fn get_chunk(&self) -> (i32, i32) {
        (
            (self.x / 16.0).floor() as i32,
            (self.z / 16.0).floor() as i32,
        )
    }
}

// type PlayerData = HashMap<i32, Player>;
// type ChunkData = HashMap<(i32, i32), Chunk>;

#[allow(dead_code)]
pub async fn start() -> Result<()> {
    let start: Instant = Instant::now();

    initialize(&REGISTRY_CODEC);
    initialize(&NOISE);
    initialize(&HASHED_SEED);

    let config: Config = get_config();
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .wrap_err_with(|| format!("Could not start server on port {}", config.port))?;
    info!("Starting server on localhost:{}", config.port);

    // let mut chunk_data: ChunkData = HashMap::new();
    // for x in -16..=16 {
    //     for z in -16..=16 {
    //         chunk_data.insert((x, z), generate_chunk(x, z));
    //     }
    // }
    // let mut entity_ids: Vec<i32> = vec![];
    // let mut player_data: PlayerData = HashMap::new();

    let (state_sender, incoming_packet_receiver, outgoing_packet_sender) =
        connection_manager::init(listener);

    info!("Done ({:?})!", start.elapsed());
    run(
        config,
        state_sender,
        incoming_packet_receiver,
        outgoing_packet_sender,
    )
    .await?;

    Ok(())
}

async fn run(
    mut config: Config,
    state_sender: UnboundedSender<WrappedConnectionState>,
    mut packet_receiver: UnboundedReceiver<WrappedIncomingPacket>,
    packet_sender: UnboundedSender<WrappedOutgoingPacket>,
) -> Result<()> {
    let mut tick_count: usize = 0;
    let mut tick: Interval = interval(Duration::from_millis(50));

    let mut connections: HashMap<i32, Connection> = HashMap::new(); // Currently active connections
    let mut drop_connections: Vec<i32> = vec![]; // Connections to drop
    let mut players: HashMap<i32, Player> = HashMap::new(); // Connected players
    let mut chunks: HashMap<(i32, i32), Chunk> = HashMap::new(); // Chunk data

    loop {
        tick.tick().await;

        // Handle incoming packets
        handle_packets(
            &mut config,
            &state_sender,
            &mut packet_receiver,
            &packet_sender,
            &mut connections,
            &mut drop_connections,
            &mut players,
            &mut chunks,
        )?;

        // Send keep-alives
        if tick_count % 20 == 0 {
            for (&conn_id, connection) in &connections {
                if connection.state == ConnectionState::Play {
                    packet_sender.send(WrappedOutgoingPacket {
                        conn_id,
                        packet: OutgoingPacket::KeepAlive { keep_alive_id: 0 },
                    })?;
                }
            }
        }

        // Check timeouts
        for (&conn_id, connection) in &connections {
            if connection.last_timeout.elapsed() > Duration::from_secs(5) {
                drop_connections.push(conn_id);
            }
        }

        // Drop connections
        for &conn_id in &drop_connections {
            state_sender.send(WrappedConnectionState {
                conn_id,
                state: ConnectionState::Disconnect,
            })?;
            packet_sender.send(WrappedOutgoingPacket {
                conn_id,
                packet: OutgoingPacket::Disconnect,
            })?;
            connections.remove(&conn_id);
        }
        drop_connections.clear();

        tick_count += 1;
    }
}

fn handle_packets(
    config: &mut Config,
    state_sender: &UnboundedSender<WrappedConnectionState>,
    packet_receiver: &mut UnboundedReceiver<WrappedIncomingPacket>,
    packet_sender: &UnboundedSender<WrappedOutgoingPacket>,
    connections: &mut HashMap<i32, Connection>,
    drop_connections: &mut Vec<i32>,
    players: &mut HashMap<i32, Player>,
    chunks: &mut HashMap<(i32, i32), Chunk>,
) -> Result<()> {
    loop {
        match packet_receiver.try_recv() {
            // Receive incoming packet
            Ok(WrappedIncomingPacket { conn_id, packet }) => {
                let send_packet_to = |conn_id: i32, p: OutgoingPacket| {
                    packet_sender.send(WrappedOutgoingPacket { conn_id, packet: p })
                };
                let send_packet = |p: OutgoingPacket| send_packet_to(conn_id, p);

                if connections.get(&conn_id).is_none() {
                    connections.insert(conn_id, Connection::new());
                }
                let connection: &mut Connection = connections.get_mut(&conn_id).unwrap();

                // Verify state (probably unnecessary)
                if connection.state != packet.sent_state() {
                    drop_connections.push(conn_id);
                    continue;
                }

                use IncomingPacket::*;
                match packet {
                    Handshake {
                        protocol_version: _,
                        server_address: _,
                        server_port: _,
                        next_state,
                    } => {
                        // Transition to the next state
                        let state: ConnectionState = match next_state {
                            0x01 => ConnectionState::Status,
                            0x02 => ConnectionState::Login,
                            // Drop the connection if the state is invalid
                            _ => {
                                drop_connections.push(conn_id);
                                continue;
                            }
                        };

                        connection.last_timeout = Instant::now();
                        connection.state = state;

                        // Update state
                        state_sender.send(WrappedConnectionState { conn_id, state })?;
                    }
                    StatusRequest => {
                        connection.last_timeout = Instant::now();

                        // Send status
                        packet_sender.send(WrappedOutgoingPacket {
                            conn_id,
                            packet: OutgoingPacket::StatusResponse {
                                json_response: serde_json::to_string(&config.status).unwrap(),
                            },
                        })?;
                    }
                    PingRequest { payload } => {
                        // Send ping response
                        packet_sender.send(WrappedOutgoingPacket {
                            conn_id,
                            packet: OutgoingPacket::PingResponse { payload },
                        })?;

                        drop_connections.push(conn_id);
                    }
                    LoginStart { name, .. } => {
                        // Add player
                        config.status.players.online += 1; // Probably shouldn't use this.
                        let player: Player = Player::new(name);
                        players.insert(conn_id, player.clone());

                        connection.state = ConnectionState::Play;

                        // Update state
                        state_sender.send(WrappedConnectionState {
                            conn_id,
                            state: connection.state,
                        })?;

                        use OutgoingPacket::*;
                        use PlayerInfoUpdateActions::*;
                        send_packet(LoginSuccess {
                            uuid: player.uuid,
                            username: player.name.clone(),
                        })?;
                        send_packet(LoginPlay {
                            entity_id: conn_id as i32,
                            is_hardcore: IS_HARDCORE,
                            gamemode: Gamemode::Creative,
                            previous_gamemode: None,
                            dimension_names: vec!["minecraft:overworld".to_string()],
                            registry_codec: REGISTRY_CODEC.clone(),
                            dimension_type: "minecraft:overworld".to_string(),
                            dimension_name: "minecraft:overworld".to_string(),
                            hashed_seed: *HASHED_SEED,
                            max_players: config.status.players.max,
                            view_distance: VIEW_DISTANCE,
                            simulation_distance: SIMULATION_DISTANCE,
                            reduced_debug_info: REDUCED_DEBUG_INFO,
                            enable_respawn_screen: ENABLE_RESPAWN_SCREEN,
                            is_debug: IS_DEBUG,
                            is_flat: IS_FLAT,
                            death_location: None,
                            portal_cooldown: 0,
                        })?;
                        send_packet(SetCenterChunk {
                            chunk_x: 0,
                            chunk_z: 0,
                        })?;
                        send_packet(SetDefaultSpawnPosition {
                            location: Position {
                                x: DEFAULT_SPAWN_POSITION.0 as i32,
                                y: DEFAULT_SPAWN_POSITION.1 as i16,
                                z: DEFAULT_SPAWN_POSITION.2 as i32,
                            },
                            angle: 0.0,
                        })?;
                        for x in -VIEW_DISTANCE..=VIEW_DISTANCE {
                            for z in -VIEW_DISTANCE..=VIEW_DISTANCE {
                                send_packet(ChunkDataAndUpdateLight {
                                    chunk_x: x,
                                    chunk_z: z,
                                    heightmaps: nbt!({}),
                                    data: get_chunk(chunks, x, z),
                                    block_entities: vec![],
                                    sky_light_mask: vec![],
                                    block_light_mask: vec![],
                                    empty_sky_light_mask: vec![],
                                    empty_block_light_mask: vec![],
                                    sky_light_arrays: vec![],
                                    block_light_arrays: vec![],
                                })?;
                            }
                        }
                        send_packet(SynchronizePlayerPosition {
                            x: player.x,
                            y: player.y,
                            z: player.z,
                            yaw: player.yaw,
                            pitch: player.pitch,
                            flags: 0,
                            teleport_id: 0,
                        })?;
                        send_packet(PlayerInfoUpdate {
                            actions: 0x01 | 0x08,
                            players: players
                                .values()
                                .map(|p| {
                                    (
                                        p.uuid,
                                        vec![
                                            AddPlayer {
                                                name: p.name.clone(),
                                                properties: vec![],
                                            },
                                            UpdateListed { listed: true },
                                        ],
                                    )
                                })
                                .collect(),
                        })?;
                        for (&id, p) in &*players {
                            if id != conn_id {
                                // Send information about other players
                                send_packet(SpawnPlayer {
                                    entity_id: id,
                                    player_uuid: p.uuid,
                                    x: p.x,
                                    y: p.y,
                                    z: p.z,
                                    yaw: p.yaw,
                                    pitch: p.pitch,
                                })?;
                                send_packet(SetHeadRotation {
                                    entity_id: id,
                                    head_yaw: p.yaw,
                                })?;
                                // Send information about self to other players
                                send_packet_to(
                                    id,
                                    PlayerInfoUpdate {
                                        actions: 0x01 | 0x08,
                                        players: vec![(
                                            player.uuid.clone(),
                                            vec![
                                                AddPlayer {
                                                    name: player.name.clone(),
                                                    properties: vec![],
                                                },
                                                UpdateListed { listed: true },
                                            ],
                                        )],
                                    },
                                )?;
                                send_packet_to(
                                    id,
                                    SpawnPlayer {
                                        entity_id: conn_id,
                                        player_uuid: player.uuid,
                                        x: player.x,
                                        y: player.y,
                                        z: player.z,
                                        yaw: player.yaw,
                                        pitch: player.pitch,
                                    },
                                )?;
                                send_packet_to(
                                    id,
                                    SetHeadRotation {
                                        entity_id: conn_id,
                                        head_yaw: player.yaw,
                                    },
                                )?;
                            }
                        }
                        send_packet(EntityEffect {
                            entity_id: conn_id,
                            effect_id: 16,
                            amplifier: 0,
                            duration: -1,
                            flags: 0x02 | 0x04,
                            factor_codec: None,
                        })?;
                        send_packet(SetTabListHeaderAndFooter {
                            header: "{\"text\":\"\nUSSR 1.20.1\n\"}".to_string(),
                            footer: "{\"text\":\"\n\"}".to_string(),
                        })?;
                        send_packet(SetContainerSlot {
                            window_id: 0,
                            state_id: 0,
                            slot: 40,
                            slot_data: Some((
                                807,
                                1,
                                nbt!({"display": {"Name": "{\"text\":\"Magic Wand\",\"italic\":0,\"bold\":1,\"color\":\"gold\"}"}}),
                            )),
                        })?;
                        connection.last_timeout = Instant::now();
                    }
                    KeepAlive { .. } => {
                        connection.last_timeout = Instant::now();
                    }
                    SetPlayerPosition { x, y, z, on_ground } => {
                        let player: &Player = players.get(&conn_id).unwrap();
                        for &id in players.keys() {
                            if id != conn_id {
                                send_packet_to(
                                    id,
                                    OutgoingPacket::UpdateEntityPosition {
                                        entity_id: conn_id,
                                        dx: ((x * 32.0 - player.x * 32.0) * 128.0) as i16,
                                        dy: ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
                                        dz: ((z * 32.0 - player.z * 32.0) * 128.0) as i16,
                                        on_ground: player.on_ground,
                                    },
                                )?;
                            }
                        }

                        let player: &mut Player = players.get_mut(&conn_id).unwrap();
                        player.x = x;
                        player.y = y;
                        player.z = z;
                        player.on_ground = on_ground;
                    }
                    SetPlayerPositionAndRotation {
                        x,
                        y,
                        z,
                        mut yaw,
                        pitch,
                        on_ground,
                    } => {
                        yaw = normalize_yaw(yaw);

                        let player: &Player = players.get(&conn_id).unwrap();
                        for &id in players.keys() {
                            if id != conn_id {
                                send_packet_to(
                                    id,
                                    OutgoingPacket::UpdateEntityPositionAndRotation {
                                        entity_id: conn_id,
                                        dx: ((x * 32.0 - player.x * 32.0) * 128.0) as i16,
                                        dy: ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
                                        dz: ((z * 32.0 - player.z * 32.0) * 128.0) as i16,
                                        yaw,
                                        pitch,
                                        on_ground: player.on_ground,
                                    },
                                )?;
                                send_packet_to(
                                    id,
                                    OutgoingPacket::SetHeadRotation {
                                        entity_id: conn_id,
                                        head_yaw: yaw,
                                    },
                                )?;
                            }
                        }

                        let player: &mut Player = players.get_mut(&conn_id).unwrap();
                        player.x = x;
                        player.y = y;
                        player.z = z;
                        player.yaw = yaw;
                        player.pitch = pitch;
                        player.on_ground = on_ground;
                    }
                    SetPlayerRotation {
                        mut yaw,
                        pitch,
                        on_ground,
                    } => {
                        yaw = normalize_yaw(yaw);

                        let player: &Player = players.get(&conn_id).unwrap();
                        for &id in players.keys() {
                            if id != conn_id {
                                send_packet_to(
                                    id,
                                    OutgoingPacket::UpdateEntityRotation {
                                        entity_id: conn_id,
                                        yaw,
                                        pitch,
                                        on_ground: player.on_ground,
                                    },
                                )?;
                                send_packet_to(
                                    id,
                                    OutgoingPacket::SetHeadRotation {
                                        entity_id: conn_id,
                                        head_yaw: yaw,
                                    },
                                )?;
                            }
                        }

                        let player: &mut Player = players.get_mut(&conn_id).unwrap();
                        player.yaw = yaw;
                        player.pitch = pitch;
                        player.on_ground = on_ground;
                    }
                    UseItem { .. } => {
                        let p: &mut Player = players.get_mut(&conn_id).unwrap();

                        let mut yaw: f64 = p.yaw as f64;
                        let mut pitch: f64 = p.pitch as f64;

                        // Exit if no intersection
                        if pitch < 0.0 {
                            continue;
                        }
                        yaw += 90.0; // Mojang chooses the wrong axis
                        yaw = yaw.to_radians();
                        pitch = 90.0 - pitch; // Get the necessary angle
                        pitch = pitch.to_radians();

                        let height: f64 = p.y + 1.62 - 65.0;
                        let radius: f64 = height * pitch.tan();
                        let x_offset: f64 = radius * yaw.cos();
                        let z_offset: f64 = radius * yaw.sin();

                        p.x += x_offset;
                        p.y = 65.0;
                        p.z += z_offset;

                        send_packet(OutgoingPacket::SynchronizePlayerPosition {
                            x: p.x,
                            y: p.y,
                            z: p.z,
                            yaw: p.yaw,
                            pitch: p.pitch,
                            flags: 0,
                            teleport_id: 0,
                        })?;
                    }
                    _ => {}
                }
            }

            // No more packets
            Err(TryRecvError::Empty) => break,

            // Channel closed
            Err(TryRecvError::Disconnected) => {
                return Err(eyre!("Incoming packet channel closed"));
            }
        }
    }

    Ok(())
}
