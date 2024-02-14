use std::string;

use fastnbt::Value;

use super::{Position, String, Uuid, VarInt, VarLong};

#[derive(Debug)]
pub enum EntityMetadataField {
    Byte(u8),
    VarInt(i32),
    VarLong(i64),
    Float(f32),
    String(String),
    Chat(String),
    OptChat(Option<String>),
    Slot,
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(Position),
    OptPosition(Option<Position>),
    Direction(i32),
    OptUUID(Option<Uuid>),
    BlockID(i32),
    OptBlockID(Option<i32>),
    NBT(Value),
    Particle, // TODO
    VillagerData(i32, i32, i32),
    OptVarInt(Option<i32>),
    Pose(i32),
    CatVariant(i32),
    FrogVariant(i32),
    OptGlobalPos(Option<(String, Position)>),
    PaintingVariant(i32),
    SnifferState(i32),
    Vector3(f32, f32, f32),
    Quaternion(f32, f32, f32, f32),
}
impl EntityMetadataField {
    pub async fn to_bytes(&self) -> Vec<u8> {
        use EntityMetadataField as EMF;
        match self {
            EMF::Byte(byte) => vec![0, *byte],
            EMF::VarInt(varint) => vec![&[1][..], &VarInt(*varint).to_bytes().await].concat(),
            EMF::VarLong(varlong) => vec![&[2][..], &VarLong(*varlong).to_bytes().await].concat(),
            EMF::Boolean(bool) => vec![8, *bool as u8],
            _ => todo!(),
        }
    }
}
