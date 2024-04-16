use super::EntityMetadataField;

#[derive(Debug, PartialEq, Clone)]
pub struct EntityMetadata(pub Vec<(u8, EntityMetadataField)>);
