use log::debug;

use super::Block;
use crate::server::types::WriteVarInt;

#[derive(Clone, Debug)]
pub struct ChunkSection {
    //? Maybe place size restriction: [Block; 4096]
    pub blocks: Vec<Block>,
    // TODO: pub biomes: Vec<Biome>,
}
impl ChunkSection {
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.blocks
            .get(((x & 15) + (z & 15) * 16 + (y & 15) * 256) as usize)
    }

    pub fn is_empty(&self, x: i32, y: i32, z: i32) -> bool {
        self.get_block(x, y, z).unwrap_or(&Block::Air).is_empty()
    }

    pub fn max_height_at(&self, x: i32, z: i32) -> Option<i32> {
        for y in (0..16).rev() {
            if let Some(block) = self.get_block(x, y, z) {
                if !block.is_empty() {
                    return Some(y);
                }
            }
        }
        None
    }

    pub fn get_highest_block_at(&self, x: i32, z: i32) -> Option<&Block> {
        for y in (0..16).rev() {
            if let Some(block) = self.get_block(x, y, z) {
                return Some(block);
            }
        }
        None
    }

    pub async fn to_bytes(&self) -> Vec<u8> {
        const AIR_STATE_ID: u16 = 0;
        const VOID_AIR_STATE_ID: u16 = 12817;
        const CAVE_AIR_STATE_ID: u16 = 12818;

        let mut block_count: u16 = 0;
        let mut blocks: Vec<u16> = self
            .blocks
            .iter()
            .take(4096)
            .map(|b| {
                let sid: u16 = b.get_state_id();
                // TODO: check if this is faster than filtering non-air blocks
                if sid != AIR_STATE_ID && sid != VOID_AIR_STATE_ID && sid != CAVE_AIR_STATE_ID {
                    block_count += 1;
                }
                sid
            })
            .collect();
        blocks.resize(4096, AIR_STATE_ID);

        // TODO: check if this is faster than counting when mapping and making a hashset
        let mut unique_blocks: Vec<u16> = blocks.clone();
        unique_blocks.sort_unstable();
        unique_blocks.dedup();

        let bpe: u8 = match unique_blocks.len() {
            0..=1 => 0,
            2..=15 => 4,
            16..=31 => 5,
            32..=63 => 6,
            64..=127 => 7,
            128..=255 => 8,
            _ => 15,
        };

        let block_states: Vec<u8> = match bpe {
            0 => {
                let mut d: Vec<u8> = Vec::with_capacity(1 + 3 + 1);
                d.push(0);
                d.write_varint(blocks[0] as i32).await;
                d.push(0);
                d
            }
            4..=8 => {
                let palette: Vec<u8> = {
                    let mut d: Vec<u8> = Vec::with_capacity(3 + unique_blocks.len() * 3);
                    for block in &unique_blocks {
                        d.write_varint(*block as i32).await;
                    }
                    d
                };

                let blocks_per_long: usize = (i64::BITS / bpe as u32) as usize;
                let data_length: usize = 4096 / blocks_per_long;

                let data: Vec<u8> = {
                    let mut d: Vec<i64> = vec![0; data_length];
                    for i in 0..4096 {
                        d[i / blocks_per_long] |= (unique_blocks
                            .iter()
                            .position(|&b| b == blocks[i])
                            .unwrap_or_default()
                            as i64)
                            << (bpe as usize * (i % blocks_per_long));
                    }

                    d.into_iter()
                        .map(|l: i64| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat()
                };

                let mut d: Vec<u8> = Vec::with_capacity(1 + 5 + palette.len() + 2 + data.len());
                d.push(bpe);
                d.write_varint(unique_blocks.len() as i32).await;
                d.extend_from_slice(&palette);
                d.write_varint(data_length as i32).await;
                d.extend_from_slice(&data);
                d
            }
            _ => {
                let blocks_per_long: usize = (i64::BITS / bpe as u32) as usize;
                let data_length: usize = 4096 / blocks_per_long;

                let data: Vec<u8> = {
                    let mut d: Vec<i64> = vec![0; data_length];
                    for i in 0..4096 {
                        d[i / blocks_per_long] |=
                            (blocks[i] as i64) << (bpe * (i % blocks_per_long) as u8);
                    }

                    d.into_iter()
                        .map(|l: i64| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat()
                };

                let mut d: Vec<u8> = Vec::with_capacity(1 + 2 + data.len());
                d.push(bpe);
                d.write_varint(data_length as i32).await;
                d.extend_from_slice(&data);
                d
            }
        };

        let biomes: Vec<u8> = {
            let mut d: Vec<u8> = Vec::with_capacity(1 + 3 + 1);
            d.push(0);
            d.write_varint(56).await;
            d.write_varint(0).await;
            d
        };

        let mut d: Vec<u8> = Vec::with_capacity(2 + block_states.len() + biomes.len());
        d.extend_from_slice(&block_count.to_be_bytes());
        d.extend_from_slice(&block_states);
        d.extend_from_slice(&biomes);
        d
    }
}
