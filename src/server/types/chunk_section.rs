use super::PalettedContainer;

pub struct ChunkSection {
    block_count: u16,
    block_states: PalettedContainer,
    biomes: PalettedContainer,
}
