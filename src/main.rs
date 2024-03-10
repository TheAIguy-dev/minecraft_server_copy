#![forbid(unsafe_code)]
#![recursion_limit = "256"]

use std::io::Write;
use std::{env, fs};

use base64::{engine::general_purpose, Engine};
use chrono::Local;
use env_logger::Builder;
use lazy_static::lazy_static;
use log::{debug, info, warn, LevelFilter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use config::Config;
use net::packets::prefix_with_length;
use server::net;
use server::types::{ReadVarInt, WriteString, WriteVarInt};

mod config;
mod leb128_async;
mod server;

#[cfg(test)]
mod tests;

async fn client() {
    info!("Starting client");

    let address: &str = "mc.globalworlds.net";
    let port: u16 = 25565;

    let mut connection: TcpStream = TcpStream::connect(format!("{address}:{port}"))
        .await
        .unwrap();
    info!("Client connected");

    let mut handshake_packet: Vec<u8> = vec![];
    handshake_packet.write_varint(0).await;
    handshake_packet.write_varint(763).await;
    handshake_packet.write_string(address).await;
    handshake_packet.extend_from_slice(&port.to_be_bytes());
    handshake_packet.write_varint(2).await;
    prefix_with_length(&mut handshake_packet).await;
    connection.write_all(&handshake_packet).await.unwrap();

    let mut login_start_packet: Vec<u8> = vec![];
    handshake_packet.write_varint(0).await;
    handshake_packet.write_string("tester").await;
    handshake_packet.write_varint(0).await;
    prefix_with_length(&mut login_start_packet).await;
    connection.write_all(&login_start_packet).await.unwrap();

    debug!("All packets sent");

    loop {
        let packet_length: i32 = connection.read_varint().await;

        if packet_length == 0 {
            continue;
        }

        let packet_id: i32 = connection.read_varint().await;
        info!("{packet_length} {packet_id:#x}");

        let mut response: Vec<u8> =
            vec![0; packet_length as usize - vec![].write_varint(packet_id).await];
        connection
            .read_exact(&mut response)
            .await
            .unwrap_or_default();

        debug!("{response:?}");
    }
}

lazy_static! {
    pub static ref PROTOCOL_VERSION: i32 = 763;
}

pub fn get_config() -> Config {
    let mut config: Config =
        confy::load_path("config.toml").expect("Something went wrong when loading config.");

    if let Some(favicon_file) = config.status.favicon {
        if let Ok(favicon_binary) = &fs::read(favicon_file) {
            let favicon_base64 = general_purpose::STANDARD.encode(favicon_binary);
            config.status.favicon = Some("data:image/png;base64,".to_string() + &favicon_base64);
        } else {
            warn!("Could not load favicon!");
            config.status.favicon = None;
        }
    }

    config
}

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    debug!("{args:?}");

    match args.get(1).map(|s: &String| s.as_str()) {
        Some("" | "server") | None => {
            server::start().await;

            server::test().await;
        }
        Some("client") => {
            client().await;
        }
        _ => panic!("Invalid arguments"),
    }
}
