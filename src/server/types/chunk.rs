use super::{Block, ChunkSection, Dimension};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    pub dimension: Dimension,
    pub chunk_sections: Vec<ChunkSection>,
}
impl Chunk {
    pub fn get_block(&self, x: u8, mut y: i32, z: u8) -> Option<&Block> {
        if self.dimension == Dimension::Overworld {
            y += 64;
        }

        self.chunk_sections
            .get(y as usize / 16)
            .map(|cs| cs.get_block(x, (y % 16) as u8, z))?
    }

    pub fn is_empty(&self, x: u8, y: i32, z: u8) -> bool {
        self.get_block(x, y, z).unwrap_or(&Block::Air).is_empty()
    }

    pub fn max_height_at(&self, x: u8, z: u8) -> Option<i32> {
        for i in (0..self.chunk_sections.len()).rev() {
            if let Some(y) = self.chunk_sections[i].max_height_at(x, z) {
                return Some(i as i32 * 16 + y as i32 + self.dimension.min_height());
            }
        }
        None
    }

    pub fn highest_block_at(&self, x: u8, z: u8) -> Option<&Block> {
        for i in (0..self.chunk_sections.len()).rev() {
            if let Some(block) = self.chunk_sections[i].get_highest_block_at(x, z) {
                return Some(block);
            }
        }
        None
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: check if this is faster than concatenating
        let mut d: Vec<u8> = Vec::with_capacity(24);
        for i in 0..self.dimension.chunk_section_count() as usize {
            d.extend_from_slice(
                &self
                    .chunk_sections
                    .get(i)
                    .unwrap_or(&ChunkSection { blocks: vec![] })
                    .to_bytes(),
            );
        }
        d
    }
}
