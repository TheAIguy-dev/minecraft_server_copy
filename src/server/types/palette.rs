use super::VarInt;

pub enum Palette {
    SingleValued(VarInt),
    Indirect {
        length: VarInt,
        palette: Vec<VarInt>,
    },
    Direct,
}
