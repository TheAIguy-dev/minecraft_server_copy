use eyre::{ensure, ContextCompat, Result};
use paste::paste;
use rand::{
    distributions::{Distribution, Standard},
    random,
};
use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};
use std::{
    collections::VecDeque,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use log::debug;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use super::types::{leb128::Error::*, Block, Chunk, ChunkSection, Dimension};

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

macro_rules! read_type {
    ($x:ty) => {
        paste! {
            fn [<read_ $x>](&mut self) -> Result<$x>;
        }
    };
}
macro_rules! read_int_impl {
    ($x:ty) => {
        paste! {
            fn [<read_ $x>](&mut self) -> Result<$x> {
                ensure!($x::BITS as usize / 8 <= self.len(), EndOfFile);
                let mut result: $x = 0;
                for _ in 0..$x::BITS / 8 - 1 {
                    result |= self.pop_front().context(EndOfFile)? as $x;
                    result <<= 8;
                }
                result |= self.pop_front().context(EndOfFile)? as $x;
                Ok(result)
            }
        }
    };
}
pub trait ReadExt {
    read_type!(u8);
    read_type!(u16);
    read_type!(u32);
    read_type!(u64);
    read_type!(u128);
    read_type!(i8);
    read_type!(i16);
    read_type!(i32);
    read_type!(i64);
    read_type!(i128);
    read_type!(f32);
    read_type!(f64);
}
impl ReadExt for VecDeque<u8> {
    fn read_u8(&mut self) -> Result<u8> {
        ensure!(!self.is_empty(), EndOfFile);
        self.pop_front().context(EndOfFile)
    }
    read_int_impl!(u16);
    read_int_impl!(u32);
    read_int_impl!(u64);
    read_int_impl!(u128);
    fn read_i8(&mut self) -> Result<i8> {
        ensure!(!self.is_empty(), EndOfFile);
        Ok(self.pop_front().context(EndOfFile)? as i8)
    }
    read_int_impl!(i16);
    read_int_impl!(i32);
    read_int_impl!(i64);
    read_int_impl!(i128);
    fn read_f32(&mut self) -> Result<f32> {
        ensure!(4 <= self.len(), EndOfFile);
        let mut result: u32 = 0;
        for _ in 0..3 {
            result |= self.pop_front().context(EndOfFile)? as u32;
            result <<= 8;
        }
        result |= self.pop_front().context(EndOfFile)? as u32;
        Ok(f32::from_bits(result))
    }
    fn read_f64(&mut self) -> Result<f64> {
        ensure!(8 <= self.len(), EndOfFile);
        let mut result: u64 = 0;
        for _ in 0..7 {
            result |= self.pop_front().context(EndOfFile)? as u64;
            result <<= 8;
        }
        result |= self.pop_front().context(EndOfFile)? as u64;
        Ok(f64::from_bits(result))
    }
}
