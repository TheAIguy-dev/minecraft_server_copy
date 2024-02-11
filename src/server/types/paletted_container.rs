use super::{Palette, VarInt};

pub struct PalettedContainer {
    bits_per_entry: u8,
    palette: Palette,
    data_array_length: VarInt,
    data_array: Vec<i64>,
}
