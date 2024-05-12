use eyre::{eyre, Context, Result};
use tokio::{net::tcp::OwnedWriteHalf, sync::mpsc::UnboundedReceiver};

use crate::server::net::packets::{OutgoingPacket, WritePacket};

pub async fn send_packets(
    mut write_half: OwnedWriteHalf,
    mut outgoing_packet_receiver: UnboundedReceiver<OutgoingPacket>,
) -> Result<()> {
    while let Some(packet) = outgoing_packet_receiver.recv().await {
        write_half
            .write_packet(packet)
            .await
            .context("Failed to write packet")?;
    }

    Err(eyre!("Outgoing packet channel closed"))
}
