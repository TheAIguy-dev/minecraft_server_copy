pub struct Angle(f32);
impl Angle {
    pub fn from_deg(deg: f32) -> Self {
        Self(deg)
    }

    pub fn from_angle(angle: u8) -> Self {
        Self((angle as f32 / 360.0) * 256.0)
    }

    pub fn to_angle(&self) -> u8 {
        ((self.0 / 360.0) * 256.0) as i8 as u8
    }

    pub fn get_deg(&self) -> f32 {
        self.0
    }
}
