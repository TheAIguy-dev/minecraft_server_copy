#[derive(Debug)]
pub struct Uuid(pub u128);
impl Uuid {
    pub fn offline(name: String) -> Self {
        let mut hash: [u8; 16] = md5::compute(format!("OfflinePlayer:{name}")).0;
        hash[6] = hash[6] & 0x0f | 0x30;
        hash[8] = hash[8] & 0x3f | 0x80;
        Self(uuid::Uuid::from_bytes(hash).as_u128())
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

impl Default for Uuid {
    /// Generated from "OfflinePlayer:default"
    fn default() -> Self {
        Self(110388530216132912663215514682830219291)
    }
}
