#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}
impl Dimension {
    pub fn chunk_section_n(&self) -> i32 {
        match self {
            Self::Overworld => 24,
            _ => 16,
        }
    }

    pub fn min_height(&self) -> i32 {
        match self {
            Self::Overworld => -64,
            _ => 0,
        }
    }

    pub fn max_height(&self) -> i32 {
        match self {
            Self::Overworld => 319,
            _ => 255,
        }
    }
}
