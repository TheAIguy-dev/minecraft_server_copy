use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{fs, time::Duration};

use bracket_noise::prelude::{FastNoise, NoiseType};
use eyre::{eyre, Context, Result};
use lazy_static::lazy_static;
use log::{debug, info};
use sha256::digest;

use tokio::sync::mpsc::error::TryRecvError;
use tokio::time::Interval;
use tokio::{
    self,
    net::TcpListener,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::interval,
};
use tokio_util::bytes::Buf;

use crate::server::net::packets::OutgoingPacket;
use crate::{
    config::{get_config, Config},
    SEED,
};
use net::connection::{
    accept_connections, data_distributor, ConnectionStateWrapper, IncomingPacketWrapper,
    OutgoingPacketWrapper,
};

use self::net::connection::Connection;
use self::net::packets::IncomingPacket;
use self::state::ConnectionState;

pub mod net;
pub mod state;
#[allow(dead_code)]
pub mod types;
pub mod util;

lazy_static! {
    pub static ref REGISTRY_CODEC: Vec<u8> = fs::read("registry_codec.nbt").unwrap_or_default();
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

// #[derive(Clone, Debug)]
// struct Player {
//     pub entity_id: i32,
//     pub name: String,
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
//     pub yaw: f32,
//     pub pitch: f32,
//     pub on_ground: bool,
// }
// impl Player {
//     pub fn get_block(&self) -> (i32, i32, i32) {
//         (
//             self.x.floor() as i32,
//             self.y.floor() as i32,
//             self.z.floor() as i32,
//         )
//     }

//     pub fn get_chunk(&self) -> (i32, i32) {
//         (
//             (self.x / 16.0).floor() as i32,
//             (self.z / 16.0).floor() as i32,
//         )
//     }
// }

// type PlayerData = HashMap<i32, Player>;
// type ChunkData = HashMap<(i32, i32), Chunk>;

#[allow(dead_code)]
pub async fn start() -> Result<()> {
    let start: Instant = Instant::now();

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

    let (connection_sender, connection_receiver) = mpsc::unbounded_channel();
    let (state_sender, state_receiver) = mpsc::unbounded_channel();
    let (incoming_packet_sender, incoming_packet_receiver) = mpsc::unbounded_channel();
    let (outgoing_packet_sender, outgoing_packet_receiver) = mpsc::unbounded_channel();

    tokio::spawn(accept_connections(listener, connection_sender));
    tokio::spawn(data_distributor(
        connection_receiver,
        state_receiver,
        incoming_packet_sender,
        outgoing_packet_receiver,
    ));

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
    config: Config,
    state_sender: UnboundedSender<ConnectionStateWrapper>,
    mut incoming_packet_receiver: UnboundedReceiver<IncomingPacketWrapper>,
    outgoing_packet_sender: UnboundedSender<OutgoingPacketWrapper>,
) -> Result<()> {
    let mut tick: Interval = interval(Duration::from_millis(50));
    let mut connections: HashMap<i32, Connection> = HashMap::new();
    // let mut players: HashMap<i32, Player> = HashMap::new();

    loop {
        tick.tick().await;
        let mut drop_connections: HashSet<i32> = HashSet::new();

        // let mut packets: HashMap<i32, Vec<IncomingPacket>> = HashMap::new();

        // Handle incoming packets
        loop {
            use IncomingPacket::*;
            match incoming_packet_receiver.try_recv() {
                // Receive outgoing packet
                Ok(IncomingPacketWrapper { conn_id, packet }) => {
                    if connections.get(&conn_id).is_none() {
                        connections.insert(conn_id, Connection::default());
                    }
                    let connection: &mut Connection = connections.get_mut(&conn_id).unwrap();

                    debug!("{packet}");

                    match packet {
                        Handshake {
                            protocol_version: _,
                            server_address: _,
                            server_port: _,
                            next_state,
                        } => {
                            // Verify state
                            if connection.state != ConnectionState::Handshake {
                                drop_connections.insert(conn_id);
                                continue;
                            }

                            debug!("[MAIN] Received handshake");

                            // Transition to the next state
                            let state: ConnectionState = match next_state {
                                0x01 => ConnectionState::Status,
                                0x02 => ConnectionState::Login,
                                // Drop the connection if the state is invalid
                                _ => {
                                    drop_connection(&outgoing_packet_sender, conn_id)?;
                                    continue;
                                }
                            };

                            connection.last_timeout = Instant::now();
                            connection.state = state;

                            debug!("[MAIN] Sending state {state}");

                            // Update state
                            state_sender.send(ConnectionStateWrapper { conn_id, state })?;

                            debug!("[MAIN] Sent state");
                        }
                        StatusRequest => {
                            // Verify state
                            if connection.state != ConnectionState::Status {
                                drop_connections.insert(conn_id);
                                continue;
                            }

                            connection.last_timeout = Instant::now();

                            // Send status
                            outgoing_packet_sender.send(OutgoingPacketWrapper {
                                conn_id,
                                packet: OutgoingPacket::StatusResponse {
                                    json_response: serde_json::to_string(&config.status)
                                        .unwrap_or_default(),
                                },
                            })?;
                        }
                        PingRequest { payload } => {
                            // Verify state
                            if connection.state != ConnectionState::Status {
                                drop_connections.insert(conn_id);
                                continue;
                            }

                            // Send ping response
                            outgoing_packet_sender.send(OutgoingPacketWrapper {
                                conn_id,
                                packet: OutgoingPacket::PingResponse { payload },
                            })?;

                            // Drop the connection
                            outgoing_packet_sender.send(OutgoingPacketWrapper {
                                conn_id,
                                packet: OutgoingPacket::Drop,
                            })?;
                        }
                        _ => {}
                    }
                }

                // No more outgoing packets
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => {
                    return Err(eyre!("Outgoing packet channel closed"));
                }
            }
        }

        // Drop connections
        for (&conn_id, connection) in &mut connections {
            if connection.last_timeout.elapsed() > Duration::from_secs(5) {
                drop_connections.insert(conn_id);
            }
        }
        for conn_id in drop_connections {
            // debug!("Dropping connection {conn_id}");
            drop_connection(&outgoing_packet_sender, conn_id)?;
            connections.remove(&conn_id);
        }
    }
}

fn drop_connection(
    outgoing_packet_sender: &UnboundedSender<OutgoingPacketWrapper>,
    conn_id: i32,
) -> Result<()> {
    outgoing_packet_sender.send(OutgoingPacketWrapper {
        conn_id,
        packet: OutgoingPacket::Drop,
    })?;
    Ok(())
}
