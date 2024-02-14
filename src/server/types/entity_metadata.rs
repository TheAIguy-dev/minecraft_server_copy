use super::EntityMetadataField;

#[derive(Debug)]
pub struct EntityMetadata(pub Vec<(u8, EntityMetadataField)>);
