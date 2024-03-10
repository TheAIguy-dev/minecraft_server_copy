pub enum Palette {
    SingleValued(i32),
    Indirect(u8, Vec<i32>, Vec<i64>),
    Direct(u8, Vec<i64>),
}
