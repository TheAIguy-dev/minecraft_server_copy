use super::{Block, ChunkSection, Dimension};

#[derive(Clone, Debug)]
pub struct Chunk {
    pub dimension: Dimension,
    pub chunk_sections: Vec<ChunkSection>,
}
impl Chunk {
    pub fn get_block(&self, x: i32, mut y: i32, z: i32) -> Option<&Block> {
        if self.dimension == Dimension::Overworld {
            y += 64;
        }

        self.chunk_sections
            .get(y as usize / 16)
            .map(|cs| cs.get_block(x, y, z))?
    }

    pub fn is_empty(&self, x: i32, y: i32, z: i32) -> bool {
        self.get_block(x, y, z).unwrap_or(&Block::Air).is_empty()
    }

    pub fn max_height_at(&self, x: i32, z: i32) -> Option<i32> {
        for i in (0..self.chunk_sections.len()).rev() {
            if let Some(y) = self.chunk_sections[i].max_height_at(x, z) {
                return Some(i as i32 * 16 + y + self.dimension.min_height());
            }
        }
        None
    }

    pub fn highest_block_at(&self, x: i32, z: i32) -> Option<&Block> {
        for i in (0..self.chunk_sections.len()).rev() {
            if let Some(block) = self.chunk_sections[i].get_highest_block_at(x, z) {
                return Some(block);
            }
        }
        None
    }

    pub async fn to_bytes(&self) -> Vec<u8> {
        let mut d: Vec<u8> = Vec::with_capacity(24);
        let chunk_sections = match self.dimension {
            Dimension::Overworld => 24,
            _ => 16,
        };
        for i in 0..chunk_sections {
            d.extend_from_slice(
                &self
                    .chunk_sections
                    .get(i)
                    .unwrap_or(&ChunkSection { blocks: vec![] })
                    .to_bytes()
                    .await,
            );
        }
        d // TODO: check if this is faster than concatenating
    }
}
