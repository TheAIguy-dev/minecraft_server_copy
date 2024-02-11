#![recursion_limit = "256"]
// #![allow(dead_code)]

mod config;
mod leb128_async;
mod server;
mod tests;

use base64::{engine::general_purpose, Engine};
use config::Config;
//use debug_print::debug_println;
use chrono::Local;
use env_logger::Builder;
use lazy_static::lazy_static;
use log::{debug, info, warn, LevelFilter};
use net::packets::prefix_with_length;
use server::net;
use server::types::{ReadVarInt, WriteString, WriteVarInt};
use std::io::Write;
use std::{env, fs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn client() {
    info!("Starting client");

    let address = "localhost";
    let port: u16 = 25565;

    let mut connection: TcpStream = TcpStream::connect(format!("{address}:{port}"))
        .await
        .unwrap();
    info!("Client connected");

    let mut handshake_packet: Vec<u8> = vec![];
    handshake_packet.write_var_int(0).await;
    handshake_packet.write_var_int(763).await;
    handshake_packet.write_string(address).await;
    handshake_packet.write_u16(port).await.unwrap_or_default();
    handshake_packet.write_var_int(2).await;
    prefix_with_length(&mut handshake_packet).await;
    connection.write_all(&handshake_packet).await.unwrap();

    let mut login_start_packet: Vec<u8> = vec![];
    handshake_packet.write_var_int(0).await;
    handshake_packet.write_string("tester").await;
    handshake_packet
        .write_u8(false as u8)
        .await
        .unwrap_or_default();
    prefix_with_length(&mut login_start_packet).await;
    connection.write_all(&login_start_packet).await.unwrap();

    info!("All packets sent");

    loop {
        let packet_length: i32 = connection.read_var_int().await;
        // let packet_id = VarInt::decode(&mut connection).await;

        // if packet_id == 0x03 {
        //     //println!("I'm not doing this");
        //     info! {"I'm not doing this"};
        //     break;
        // }

        if packet_length == 0 {
            continue;
        }

        // info!("{packet_length} {packet_id:#x}");

        let mut response: Vec<u8> = vec![0; packet_length as usize];
        connection
            .read_exact(&mut response)
            .await
            .unwrap_or_default();
        info!("{response:?}");
    }
}

// Config
lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config: Config =
            confy::load_path("config.toml").expect("Something went wrong when loading config.");

        if let Some(favicon_file) = config.status.favicon {
            if let Ok(favicon_binary) = &fs::read(favicon_file) {
                let favicon_base64 = general_purpose::STANDARD.encode(favicon_binary);
                config.status.favicon =
                    Some("data:image/png;base64,".to_string() + &favicon_base64);
            } else {
                warn!("Could not load favicon!");
                config.status.favicon = None;
            }
        }

        config
    };
    pub static ref PROTOCOL_VERSION: i32 = 763;
}

#[tokio::main]
async fn main() {
    // fs::write("registry_codec.nbt", REGISTRY_CODEC.clone()).unwrap();

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
        }
        Some("client") => {
            client().await;
        }
        _ => panic!("Invalid arguments"),
    }
}
