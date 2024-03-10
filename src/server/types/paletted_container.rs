use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use log::debug;

use super::{Block, Palette, VarInt};

pub struct PalettedContainer {
    bits_per_entry: u8,
    palette: Palette,
    data_array: Vec<i64>,
}
impl PalettedContainer {
    pub fn new(bits_per_entry: u8, palette: Palette, data_array: Vec<i64>) -> Self {
        Self {
            bits_per_entry,
            palette,
            data_array,
        }
    }
}
