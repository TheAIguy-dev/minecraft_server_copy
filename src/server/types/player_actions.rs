use std::string;

use crate::server::types::VarInt;

use super::{String, Uuid};

#[derive(Debug)]
pub enum PlayerActions {
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
impl PlayerActions {
    pub async fn to_bytes(&self) -> Vec<u8> {
        use PlayerActions::*;
        match self {
            AddPlayer { name, properties } => [
                &String(name.to_string()).to_bytes().await[..],
                &VarInt(properties.len() as i32).to_bytes().await,
                {
                    let mut buf: Vec<Vec<u8>> = vec![];
                    for property in properties {
                        buf.push(
                            [
                                &String(property.0.to_string()).to_bytes().await[..],
                                &String(property.1.to_string()).to_bytes().await,
                                &[property.2.is_some() as u8],
                                &match &property.2 {
                                    Some(signature) => {
                                        String(signature.to_string()).to_bytes().await
                                    }
                                    None => vec![],
                                }
                                .clone(),
                            ]
                            .concat(),
                        );
                    }
                    &buf.concat()
                },
            ]
            .concat(),
            InitializeChat { signature } => [&[signature.is_some() as u8][..], {
                &match signature {
                    Some((
                        chat_session_id,
                        public_key_expiry_time,
                        encoded_public_key,
                        public_key_signature,
                    )) => [
                        &chat_session_id.to_bytes()[..],
                        &public_key_expiry_time.to_be_bytes(),
                        &VarInt(encoded_public_key.len() as i32).to_bytes().await,
                        &encoded_public_key,
                        &VarInt(public_key_signature.len() as i32).to_bytes().await,
                        &public_key_signature,
                    ]
                    .concat(),
                    None => vec![],
                }
                .clone()
            }]
            .concat(),
            UpdateGamemode { gamemode } => VarInt(*gamemode).to_bytes().await,
            UpdateListed { listed } => vec![*listed as u8],
            UpdateLatency { ping } => VarInt(*ping).to_bytes().await,
            UpdateDisplayName { display_name } => [
                &[display_name.is_some() as u8][..],
                &match display_name {
                    Some(display_name) => String(display_name.to_string()).to_bytes().await,
                    None => vec![],
                }
                .clone(),
            ]
            .concat(),
        }
    }
}
