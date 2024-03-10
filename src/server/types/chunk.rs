use crate::server::types::WriteVarInt;

use super::ChunkSection;

#[derive(Clone, Debug)]
pub struct Chunk {
    pub chunk_sections: Vec<ChunkSection>,
}
impl Chunk {
    pub async fn to_bytes(&self) -> Vec<u8> {
        let mut d: Vec<u8> = Vec::with_capacity(24);
        for chunk_section in &self.chunk_sections {
            d.extend_from_slice(&chunk_section.to_bytes().await);
        }
        d // TODO: check if this is faster than concatenating
    }
}
