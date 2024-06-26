use std::fs;

use base64::{engine::general_purpose, Engine};
use log::warn;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::PROTOCOL_VERSION;

pub fn get_config() -> Config {
    let mut config: Config =
        confy::load_path("config.toml").expect("Something went wrong when loading config.");

    if let Some(favicon_file) = config.status.favicon {
        if let Ok(favicon_binary) = &fs::read(favicon_file) {
            let favicon_base64: String = general_purpose::STANDARD.encode(favicon_binary);
            config.status.favicon = Some("data:image/png;base64,".to_string() + &favicon_base64);
        } else {
            warn!("Could not load favicon!");
            config.status.favicon = None;
        }
    }

    config
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub status: Status,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 25565,
            status: Status::default(),
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Status {
    pub enforcesSecureChat: bool,
    pub previewsChat: bool,
    pub favicon: Option<String>,
    pub version: Version,
    pub players: Players,
    pub description: Description,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    pub name: String,
    pub protocol: u16,
}
impl Default for Version {
    fn default() -> Self {
        Self {
            name: "Unnamed Server Software in Rust 1.20.1".to_string(),
            protocol: PROTOCOL_VERSION,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<Player>>,
}
impl Default for Players {
    fn default() -> Self {
        Self {
            max: 20,
            online: 0,
            sample: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: String,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            id: "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Description {
    pub text: String,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub font: Option<String>,
    pub color: Option<String>,
    // pub insertion: Option<String>,
    // pub clickEvent: Option<ClickEvent>,
    // pub hoverEvent: Option<HoverEvent>,
    pub extra: Option<Box<Description>>,
}
impl Default for Description {
    fn default() -> Self {
        Self {
            text: "A Minecraft Server".to_string(),
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            color: None,
            extra: None,
        }
    }
}
