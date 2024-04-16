use std::string;

use super::{Uuid, VarInt, WriteString, WriteVarInt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlayerInfoUpdateActions {
    AddPlayer {
        name: string::String,
        properties: Vec<(string::String, string::String, Option<string::String>)>,
    },
    InitializeChat {
        signature: Option<(Uuid, i64, Vec<u8>, Vec<u8>)>,
    },
    UpdateGamemode {
        gamemode: i32,
    },
    UpdateListed {
        listed: bool,
    },
    UpdateLatency {
        ping: i32,
    },
    UpdateDisplayName {
        display_name: Option<string::String>,
    },
}
impl PlayerInfoUpdateActions {
    pub fn to_bytes(&self) -> Vec<u8> {
        use PlayerInfoUpdateActions::*;
        match self {
            AddPlayer { name, properties } => {
                let mut d: Vec<u8> = Vec::with_capacity(5 + name.len() + 5);
                d.write_string(name);
                d.write_varint(properties.len() as i32);
                for (name, value, signature) in properties {
                    d.reserve(
                        5 + name.len()
                            + 5
                            + value.len()
                            + 1
                            + 5
                            + signature.as_ref().map_or(0, |s| s.len()),
                    );
                    d.write_string(name);
                    d.write_string(value);
                    d.push(signature.is_some() as u8);
                    if let Some(signature) = signature {
                        d.write_string(signature);
                    }
                }
                d
            }
            InitializeChat { signature } => {
                let mut d: Vec<u8> = Vec::with_capacity(
                    1 + signature.as_ref().map_or(
                        0,
                        |(_, _, encoded_pub_key, pub_key_signature)| {
                            16 + 8 + 5 + encoded_pub_key.len() + 5 + pub_key_signature.len()
                        },
                    ),
                );
                d.push(signature.is_some() as u8);
                if let Some((
                    chat_session_id,
                    public_key_expiry_time,
                    encoded_pub_key,
                    pub_key_signature,
                )) = signature
                {
                    d.extend_from_slice(&chat_session_id.to_bytes());
                    d.extend_from_slice(&public_key_expiry_time.to_be_bytes());
                    d.write_varint(encoded_pub_key.len() as i32);
                    d.extend_from_slice(encoded_pub_key);
                    d.write_varint(pub_key_signature.len() as i32);
                    d.extend_from_slice(pub_key_signature);
                }
                d
            }
            UpdateGamemode { gamemode } => VarInt(*gamemode).to_bytes(),
            UpdateListed { listed } => vec![*listed as u8],
            UpdateLatency { ping } => VarInt(*ping).to_bytes(),
            UpdateDisplayName { display_name } => {
                let mut d: Vec<u8> =
                    Vec::with_capacity(1 + display_name.as_ref().map_or(0, |s| s.len()));
                d.push(display_name.is_some() as u8);
                if let Some(display_name) = display_name {
                    d.write_string(display_name);
                }
                d
            }
        }
    }
}
