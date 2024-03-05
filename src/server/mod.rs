use std::collections::HashMap;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{fs, string, vec};

use fastnbt::nbt;
use lazy_static::lazy_static;
use log::{debug, info};
use rand::random;
use sha256::digest;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, MutexGuard};
use tokio::task;
use tokio::time::{interval, Interval};
use tokio_util::bytes::Buf;

use net::packets::{IncomingPacket, OutgoingPacket, ReadPacket};
use state::ConnectionState;

use crate::config::Config;
use crate::server::net::packets::WritePacket;
use crate::server::types::{
    EntityMetadata, EntityMetadataField, Gamemode, PlayerActions, Position, Uuid,
};

pub mod net;
mod state;
pub mod types;

lazy_static! {
    pub static ref REGISTRY_CODEC: Vec<u8> = fs::read("registry_codec.nbt").unwrap_or_default();
    pub static ref DEFAULT_CHUNK_DATA: Vec<u8> = get_chunk_data(9);
    pub static ref HASHED_SEED: i64 = digest("69").as_bytes().get_i64();
}

#[derive(Clone, Debug)]
enum Event {
    PlayerJoin(i32),
    PlayerMove(i32, i16, i16, i16),
    PlayerQuit(i32),
}

#[derive(Clone, Debug)]
struct Player {
    id: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

type ChunkData = HashMap<(i32, i32), Vec<u8>>;
type PlayerData = HashMap<i32, Player>;

#[allow(dead_code)]
pub async fn start() {
    let config: Config = Config::default();
    let port: u16 = config.port;
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap_or_else(|_| panic!("Could not start server on port {}", port));
    info!("Started server on localhost:{}", port);

    let mut chunk_data: ChunkData = HashMap::new();
    for x in -16..17 {
        for z in -16..17 {
            chunk_data.insert((x, z), DEFAULT_CHUNK_DATA.clone());
        }
    }
    let config: Arc<Mutex<Config>> = Arc::new(Mutex::new(config));
    let chunk_data: Arc<Mutex<ChunkData>> = Arc::new(Mutex::new(chunk_data));
    let entity_ids: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
    let player_data: Arc<Mutex<PlayerData>> = Arc::new(Mutex::new(HashMap::new()));
    let events: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(vec![]));

    loop {
        if let Ok((connection, _)) = listener.accept().await {
            task::spawn(handle_connection(
                connection,
                config.clone(),
                chunk_data.clone(),
                entity_ids.clone(),
                player_data.clone(),
                events.clone(),
            ));
        }
    }
}

#[allow(dead_code)]
async fn handle_connection(
    mut connection: TcpStream,
    config: Arc<Mutex<Config>>,
    chunk_data: Arc<Mutex<ChunkData>>,
    entity_ids: Arc<Mutex<Vec<i32>>>,
    player_data: Arc<Mutex<PlayerData>>,
    events: Arc<Mutex<Vec<Event>>>,
) {
    // let (read_half, write_half) = connection.split();
    // let (mut read_half, mut write_half) = (BufReader::new(read_half), BufWriter::new(write_half));

    let timeout: u64 = 5;
    let entity_id: i32 = gen_entity_id(entity_ids.clone()).await;

    let mut tick_interval: Interval = interval(Duration::from_millis(1));
    let mut playstate_tick: u128 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;
    let mut last_timeout: Instant = Instant::now();
    let mut player: Player = Player {
        id: entity_id,
        name: "".to_string(),
        x: 8.0,
        y: 0.0,
        z: 8.0,
        yaw: 0.0,
        pitch: 0.0,
        on_ground: true,
    };
    let mut event_index: usize = events.lock().await.len();

    // let mut frame: u32 = 0;
    let mut armor_stand_eids: Vec<i32> = vec![];

    // Infinite connection
    'conn: loop {
        tick_interval.tick().await;

        let start: Instant = Instant::now();

        // Send keep alive
        if state == ConnectionState::Play {
            // debug!("Playstate tick #{playstate_tick} of player {}", player.name);
            playstate_tick += 1;
            if playstate_tick % 1000 == 0 {
                connection
                    .write_packet(OutgoingPacket::KeepAlive {
                        keep_alive_id: random(),
                    })
                    .await;
            }
        }

        // Check timeout
        if Instant::now().duration_since(last_timeout).as_secs() > timeout {
            break 'conn;
        }

        // Get all incoming data
        let mut incoming: Cursor<Vec<u8>> = Cursor::new(vec![]);
        let mut buf: [u8; 1] = [0];
        match connection.try_read(&mut buf) {
            Ok(0) => break 'conn,
            Ok(_) => {
                incoming.get_mut().extend_from_slice(&buf);
                loop {
                    let mut buf: [u8; 1024] = [0; 1024];
                    let n: usize = connection.try_read(&mut buf).unwrap_or_default();
                    if n == 0 {
                        break;
                    }
                    incoming.get_mut().extend_from_slice(&buf[..n]);
                }

                // Handle all incoming packets
                loop {
                    // Read the packet
                    let packet: IncomingPacket = incoming.read_packet(&state).await;

                    // debug!(
                    //     "NEW PACKET | State: {state:?}{} | From: {}{} | Type: {packet}",
                    //     " ".repeat(9 - format!("{state:?}").len()),
                    //     player.name,
                    //     " ".repeat(16 - player.name.len()),
                    // );

                    use IncomingPacket::*;
                    match packet {
                        Unknown { .. } => break,
                        Handshake {
                            protocol_version: _,
                            server_address: _,
                            server_port: _,
                            next_state,
                        } => {
                            last_timeout = Instant::now();
                            state = match next_state {
                                0x01 => ConnectionState::Status,
                                0x02 => ConnectionState::Login,
                                _ => break 'conn,
                            }
                        }
                        StatusRequest {} => {
                            last_timeout = Instant::now();
                            connection
                                .write_packet(OutgoingPacket::StatusResponse {
                                    json_response: serde_json::to_string(
                                        &config.lock().await.status,
                                    )
                                    .unwrap_or_default(),
                                })
                                .await;
                        }
                        PingRequest { payload } => {
                            connection
                                .write_packet(OutgoingPacket::PingResponse { payload })
                                .await;
                            break 'conn;
                        }
                        LoginStart { name, .. } => {
                            let mut player_data = player_data.lock().await;

                            config.lock().await.status.players.online += 1;
                            events.lock().await.push(Event::PlayerJoin(entity_id));

                            player.name = name;
                            player_data.insert(entity_id, player.clone());
                            state = ConnectionState::Play;
                            last_timeout = Instant::now();

                            connection
                                .write_packet(OutgoingPacket::LoginSuccess {
                                    uuid: Uuid::offline(player.name.to_string()),
                                    username: player.name.to_string(),
                                })
                                .await;

                            use OutgoingPacket::*;
                            use PlayerActions::*;
                            let mut packets: Vec<OutgoingPacket> =
                                Vec::with_capacity(256 + player_data.len() - 1 + 9);
                            packets.push(LoginPlay {
                                entity_id: player.id,
                                is_hardcore: false,
                                gamemode: Gamemode::Creative,
                                previous_gamemode: None,
                                dimension_names: vec!["minecraft:overworld".to_string()],
                                registry_codec: REGISTRY_CODEC.clone(),
                                dimension_type: "minecraft:overworld".to_string(),
                                dimension_name: "minecraft:overworld".to_string(),
                                hashed_seed: *HASHED_SEED,
                                max_players: 20,
                                view_distance: 16,
                                simulation_distance: 12,
                                reduced_debug_info: false,
                                enable_respawn_screen: true,
                                is_debug: false,
                                is_flat: true,
                                death_location: None,
                                portal_cooldown: 0,
                            });
                            packets.push(PlayerInfoUpdate {
                                actions: 0x01 | 0x08,
                                players: player_data
                                    .values()
                                    .map(|p| {
                                        (
                                            Uuid::offline(p.name.to_string()),
                                            vec![
                                                AddPlayer {
                                                    name: p.name.to_string(),
                                                    properties: vec![],
                                                },
                                                UpdateListed { listed: true },
                                            ],
                                        )
                                    })
                                    .collect(),
                            });
                            packets.push(SetCenterChunk {
                                chunk_x: 0,
                                chunk_z: 0,
                            });
                            for x in -16..17 {
                                for z in -16..17 {
                                    packets.push(ChunkDataAndUpdateLight {
                                        chunk_x: x,
                                        chunk_z: z,
                                        heightmaps: nbt!({}),
                                        data: chunk_data
                                            .lock()
                                            .await
                                            .get(&(x, z))
                                            .cloned()
                                            .unwrap_or(DEFAULT_CHUNK_DATA.clone()),
                                        block_entities: vec![],
                                        sky_light_mask: vec![],
                                        block_light_mask: vec![],
                                        empty_sky_light_mask: vec![],
                                        empty_block_light_mask: vec![],
                                        sky_light_arrays: vec![],
                                        block_light_arrays: vec![],
                                    })
                                }
                            }
                            packets.push(SetDefaultSpawnPosition {
                                location: Position { x: 8, y: 0, z: 8 },
                                angle: 0.0,
                            });
                            packets.push(SynchronizePlayerPosition {
                                x: player.x,
                                y: player.y,
                                z: player.z,
                                yaw: player.yaw,
                                pitch: player.pitch,
                                flags: 0,
                                teleport_id: 0,
                            });
                            packets.push(EntityEffect {
                                entity_id: player.id,
                                effect_id: 1,
                                amplifier: 4,
                                duration: -1,
                                flags: 0x02 | 0x04,
                                factor_codec: None,
                            });
                            packets.push(EntityEffect {
                                entity_id: player.id,
                                effect_id: 16,
                                amplifier: 0,
                                duration: -1,
                                flags: 0x02 | 0x04,
                                factor_codec: None,
                            });
                            packets.push(UpdateAttributes {
                                entity_id: player.id,
                                properties: vec![(
                                    "minecraft:generic.movement_speed".to_string(),
                                    0.1,
                                    vec![(
                                        Uuid(193644973882603813127438245391424226869),
                                        1.0,
                                        0x01,
                                    )],
                                )],
                            });
                            packets.push(SetTabListHeaderAndFooter {
                                header: "{\"text\": \"\nUSSR 1.20.1\n\"}".to_string(),
                                footer: "{\"text\": \"\n\"}".to_string(),
                            });
                            for p in player_data.values() {
                                if p.id == entity_id {
                                    continue;
                                }
                                packets.push(SpawnPlayer {
                                    entity_id: p.id,
                                    player_uuid: Uuid::offline(p.name.to_string()),
                                    x: p.x,
                                    y: p.y,
                                    z: p.z,
                                    yaw: p.yaw,
                                    pitch: p.pitch,
                                });
                            }

                            connection.write_packets(packets).await;
                        }
                        KeepAlive { .. } => {
                            last_timeout = Instant::now();
                        }
                        SetPlayerPosition { x, y, z, on_ground } => {
                            events.lock().await.push(Event::PlayerMove(
                                entity_id,
                                ((x * 32.0 - player.x * 32.0) * 128.0) as i16,
                                ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
                                ((z * 32.0 - player.z * 32.0) * 128.0) as i16,
                            ));

                            (player.x, player.y, player.z, player.on_ground) = (x, y, z, on_ground);
                            player_data.lock().await.insert(entity_id, player.clone());
                        }
                        SetPlayerPositionAndRotation {
                            x,
                            y,
                            z,
                            mut yaw,
                            pitch,
                            on_ground,
                        } => {
                            events.lock().await.push(Event::PlayerMove(
                                entity_id,
                                ((x * 32.0 - player.x * 32.0) * 128.0) as i16,
                                ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
                                ((z * 32.0 - player.z * 32.0) * 128.0) as i16,
                            ));

                            yaw = yaw.rem_euclid(360.0);
                            if yaw > 180.0 {
                                yaw -= 360.0;
                            }
                            (
                                player.x,
                                player.y,
                                player.z,
                                player.yaw,
                                player.pitch,
                                player.on_ground,
                            ) = (x, y, z, yaw, pitch, on_ground);
                            player_data.lock().await.insert(entity_id, player.clone());
                        }
                        SetPlayerRotation {
                            mut yaw,
                            pitch,
                            on_ground,
                        } => {
                            events
                                .lock()
                                .await
                                .push(Event::PlayerMove(entity_id, 0, 0, 0));

                            yaw = yaw.rem_euclid(360.0);
                            if yaw > 180.0 {
                                yaw -= 360.0;
                            }

                            (player.yaw, player.pitch, player.on_ground) = (yaw, pitch, on_ground);
                            player_data.lock().await.insert(entity_id, player.clone());
                        }
                        SetPlayerOnGround { on_ground } => {
                            events
                                .lock()
                                .await
                                .push(Event::PlayerMove(entity_id, 0, 0, 0));

                            player.on_ground = on_ground;
                            player_data.lock().await.insert(entity_id, player.clone());
                        }
                    }
                }
            }
            Err(_) => (),
        }

        {
            let start: Instant = Instant::now();
            const START: u128 = 2000;
            if playstate_tick == START {
                let mut packets: Vec<OutgoingPacket> = vec![];
                for x in 0..10 {
                    for y in 0..10 {
                        for z in 0..10 {
                            use OutgoingPacket::*;
                            let eid: i32 = gen_entity_id(entity_ids.clone()).await;
                            armor_stand_eids.push(eid);
                            packets.push(SpawnEntity {
                                entity_id: eid,
                                entity_uuid: Uuid(uuid::Uuid::new_v4().as_u128()),
                                entity_type: 2,
                                x: x as f64,
                                y: y as f64,
                                z: z as f64,
                                pitch: 0.0,
                                yaw: 0.0,
                                head_yaw: 0.0,
                                data: 0,
                                velocity_x: 0,
                                velocity_y: 0,
                                velocity_z: 0,
                            });
                        }
                    }
                }
                connection.write_packets(packets).await;
            } else if playstate_tick > START {
                let mut packets: Vec<OutgoingPacket> =
                    Vec::with_capacity(armor_stand_eids.len() * 2);

                {
                    let start: Instant = Instant::now();
                    for eid in armor_stand_eids.clone() {
                        use EntityMetadataField::*;
                        use OutgoingPacket::*;
                        if playstate_tick % 2 == 0 {
                            packets.push(SetEntityMetadata {
                                entity_id: eid,
                                metadata: EntityMetadata(vec![
                                    (0, Byte(0x20 | 0x40)),
                                    (5, Boolean(true)),
                                ]),
                            });
                            packets.push(SetEquipment {
                                entity_id: eid,
                                equipment: vec![(5, Some((637, 1, nbt!({}))))],
                            });
                        } else {
                            packets.push(SetEntityMetadata {
                                entity_id: eid,
                                metadata: EntityMetadata(vec![
                                    (0, Byte(0x20 | 0x01)),
                                    (5, Boolean(true)),
                                ]),
                            });
                            packets.push(SetEquipment {
                                entity_id: eid,
                                equipment: vec![(5, Some((77, 1, nbt!({}))))],
                            });
                        }
                    }
                    debug!("Packet construction time: {:?}", start.elapsed());
                }

                {
                    let start: Instant = Instant::now();
                    connection.write_packets(packets).await;
                    debug!("Packet write time: {:?}", start.elapsed());
                }
            }
            debug!("Total armor stand time: {:?}", start.elapsed());
        }

        // Handle events
        let mut packets: Vec<OutgoingPacket> = vec![];
        if let Some(event) = events.lock().await.iter().skip(event_index).next() {
            event_index += 1;

            use Event::*;
            use OutgoingPacket::*;
            match event {
                PlayerJoin(eid) if *eid != entity_id => {
                    let pd: MutexGuard<'_, HashMap<i32, Player>> = player_data.lock().await;
                    let p: Option<&Player> = pd.get(eid);
                    if let Some(p) = p {
                        use PlayerActions::*;
                        packets.push(PlayerInfoUpdate {
                            actions: 0x01 | 0x08,
                            players: vec![(
                                Uuid::offline(p.name.to_string()),
                                vec![
                                    AddPlayer {
                                        name: p.name.to_string(),
                                        properties: vec![],
                                    },
                                    // UpdateLatency { ping: 0 },
                                    UpdateListed { listed: true },
                                ],
                            )],
                        });
                        packets.push(SpawnPlayer {
                            entity_id: p.id,
                            player_uuid: Uuid::offline(p.name.to_string()),
                            x: p.x,
                            y: p.y,
                            z: p.z,
                            yaw: p.yaw,
                            pitch: p.pitch,
                        });
                    }
                }
                PlayerMove(eid, dx, dy, dz) if *eid != entity_id => {
                    let pd: MutexGuard<'_, HashMap<i32, Player>> = player_data.lock().await;
                    let p: Option<&Player> = pd.get(eid);
                    if p.is_some() {
                        let p = p.unwrap();
                        packets.push(UpdateEntityPositionAndRotation {
                            entity_id: p.id,
                            dx: *dx,
                            dy: *dy,
                            dz: *dz,
                            yaw: p.yaw,
                            pitch: p.pitch,
                            on_ground: p.on_ground,
                        });
                        packets.push(SetHeadRotation {
                            entity_id: p.id,
                            head_yaw: p.yaw,
                        });
                    }
                }
                PlayerQuit(eid) if *eid != entity_id => {
                    packets.push(RemoveEntities {
                        entity_ids: vec![*eid],
                    });
                }
                _ => {}
            }
        }

        connection.write_packets(packets).await;

        debug!("Total tick time: {:?}", start.elapsed());
        debug!("");
    }

    events.lock().await.push(Event::PlayerQuit(entity_id));
    entity_ids
        .lock()
        .await
        .retain(|eid: &i32| *eid != entity_id);
    player_data.lock().await.remove(&entity_id);
    if state == ConnectionState::Play {
        config.lock().await.status.players.online -= 1;
    }

    debug!("Connection with {} closed", player.name);
}

#[no_mangle]
pub async fn bench() {
    let mut buf: Vec<u8> = vec![];
    buf.write_packet(OutgoingPacket::LoginPlay {
        entity_id: 0,
        is_hardcore: false,
        gamemode: Gamemode::Creative,
        previous_gamemode: None,
        dimension_names: vec!["minecraft:overworld".to_string()],
        registry_codec: REGISTRY_CODEC.clone(),
        dimension_type: "minecraft:overworld".to_string(),
        dimension_name: "minecraft:overworld".to_string(),
        hashed_seed: *HASHED_SEED,
        max_players: 20,
        view_distance: 16,
        simulation_distance: 12,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        is_debug: false,
        is_flat: true,
        death_location: None,
        portal_cooldown: 0,
    })
    .await;
    debug!("{:?}", buf);

    // let mut times: Vec<Duration> = vec![];

    // for _ in 0..1 {
    //     let start: Instant = Instant::now();

    //     {
    //         let mut packets: Vec<OutgoingPacket> = vec![];

    //         for i in 0..1000 {
    //             use EntityMetadataField::*;
    //             use OutgoingPacket::*;
    //             if i % 2 == 0 {
    //                 packets.push(SetEntityMetadata {
    //                     entity_id: 0,
    //                     metadata: EntityMetadata(vec![(0, Byte(0x20 | 0x40)), (5, Boolean(true))]),
    //                 });
    //                 packets.push(SetEquipment {
    //                     entity_id: 0,
    //                     equipment: vec![(5, Some((637, 1, nbt!({}))))],
    //                 });
    //             } else {
    //                 packets.push(SetEntityMetadata {
    //                     entity_id: 0,
    //                     metadata: EntityMetadata(vec![(0, Byte(0x20 | 0x01)), (5, Boolean(true))]),
    //                 });
    //                 packets.push(SetEquipment {
    //                     entity_id: 0,
    //                     equipment: vec![(5, Some((77, 1, nbt!({}))))],
    //                 });
    //             }
    //         }

    //         Vec::with_capacity(2000).write_packets(packets).await;
    //     }

    //     times.push(start.elapsed());
    // }

    // debug!(
    //     "Average time: {:?}µs",
    //     times.iter().map(|d| d.as_micros()).sum::<u128>() as f64 / times.len() as f64
    // );
}

/// Get a unique entity identifier
async fn gen_entity_id(entity_ids: Arc<Mutex<Vec<i32>>>) -> i32 {
    let mut entity_id: i32 = entity_ids.lock().await.len() as i32;
    while entity_ids.lock().await.contains(&entity_id) {
        entity_id = random();
    }
    entity_ids.lock().await.push(entity_id);
    entity_id
}

#[allow(dead_code)]
fn get_peer_address(stream: &TcpStream) -> string::String {
    stream
        .peer_addr()
        .unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
        .to_string()
}

/// Returns a solid chunk section of `block_state_id`.
/// The highest empty block is at y level 0.
pub fn get_chunk_data(block_state_id: u8) -> Vec<u8> {
    vec![
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        16,
        0,
        0,
        block_state_id,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        56,
        0,
    ]
}
