use log::{debug, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::server::{
    net::packets::prefix_with_length,
    types::{ReadVarInt, WriteString, WriteVarInt},
};

pub async fn start() {
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
