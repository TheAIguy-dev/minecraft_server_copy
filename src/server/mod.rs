use std::fs;
use std::time::Instant;

use bracket_noise::prelude::{FastNoise, NoiseType};
use lazy_static::lazy_static;
use log::info;
use sha256::digest;

use tokio::{
    self,
    net::TcpListener,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tokio_util::bytes::Buf;

use crate::{
    config::{get_config, Config},
    server::net::connections::{accept_connections, data_distributor},
    SEED,
};

use self::net::connections::{
    ConnectionStateWrapper, IncomingPacketWrapper, OutgoingPacketWrapper,
};

pub mod net;
mod state;
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
//     id: i32,
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
//
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
pub async fn start() {
    let start: Instant = Instant::now();

    let config: Config = get_config();
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap_or_else(|_| panic!("Could not start server on port {}", config.port));
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
        state_sender,
        incoming_packet_receiver,
        outgoing_packet_sender,
    )
    .await;
}

async fn run(
    state_sender: UnboundedSender<ConnectionStateWrapper>,
    incoming_packet_receiver: UnboundedReceiver<IncomingPacketWrapper>,
    outgoing_packet_sender: UnboundedSender<OutgoingPacketWrapper>,
) {
    todo!()
}
