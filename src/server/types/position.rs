#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}
impl Position {
    pub fn to_bytes(self) -> [u8; 8] {
        (((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF))
            .to_be_bytes()
    }
}
