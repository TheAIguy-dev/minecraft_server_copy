use rand::{
    distributions::{Distribution, Standard},
    random,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};

use log::debug;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use super::types::{Block, Chunk, ChunkSection, Dimension};

pub fn gen_unique_key<T, U>(hash_map: &HashMap<T, U>) -> T
where
    T: Eq + Hash,
    Standard: Distribution<T>,
{
    let mut key: T = random();

    while hash_map.contains_key(&key) {
        key = random();
    }

    key
}

pub fn generate_chunk(x: i32, z: i32) -> Chunk {
    use Block::*;
    let mut chunk: Chunk = Chunk {
        dimension: Dimension::Overworld,
        chunk_sections: vec![],
    };

    let mut chunk_section: ChunkSection = ChunkSection { blocks: vec![] };
    for _ in 0..256 {
        chunk_section.blocks.push(Bedrock);
    }
    for _ in 0..512 {
        chunk_section.blocks.push(Dirt);
    }
    for _ in 0..256 {
        chunk_section.blocks.push(GrassBlock { snowy: false });
    }
    chunk.chunk_sections.push(chunk_section);

    // for section_y in 0..24 {
    //     let mut chunk_section: ChunkSection = ChunkSection { blocks: vec![] };
    //     for block_y in 0..16 {
    //         for block_z in 0..16 {
    //             for block_x in 0..16 {
    //                 let y: i32 = section_y * 16 + block_y - 64;
    //                 if (y as f32)
    //                     < 96.0
    //                         + NOISE.get_noise((x * 16 + block_x) as f32, (z * 16 + block_z) as f32)
    //                             * 128.0
    //                 {
    //                     chunk_section.blocks.push(Stone);
    //                 } else if y < 63 {
    //                     chunk_section.blocks.push(Water { level: I0_15::MAX });
    //                 } else {
    //                     chunk_section.blocks.push(Air);
    //                 }
    //             }
    //         }
    //     }
    //     chunk.chunk_sections.push(chunk_section);
    // }

    debug!("Generated chunk {x} {z}");
    chunk
}

async fn gen_entity_id(entity_ids: Arc<Mutex<Vec<i32>>>) -> i32 {
    let mut entity_id: i32 = entity_ids.lock().await.len() as i32;
    while entity_ids.lock().await.contains(&entity_id) {
        entity_id = random();
    }
    entity_ids.lock().await.push(entity_id);
    entity_id
}

fn get_peer_address(stream: &TcpStream) -> String {
    stream
        .peer_addr()
        .unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
        .to_string()
}

macro_rules! import_all {
    ($($x:ident),+) => {
        $(
            pub mod $x;
            pub use $x::*;
        )*
    };
}
pub(crate) use import_all;
