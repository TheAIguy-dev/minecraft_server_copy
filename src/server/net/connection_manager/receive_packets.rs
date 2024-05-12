use eyre::{Context, Result};

use tokio::{
    net::tcp::OwnedReadHalf,
    sync::{mpsc::UnboundedSender, watch},
};

use crate::server::{
    net::packets::{IncomingPacket, ReadPacket},
    state::ConnectionState,
};

use super::types::WrappedIncomingPacket;

pub async fn receive_packets(
    conn_id: i32,
    mut read_half: OwnedReadHalf,
    mut state_receiver: watch::Receiver<ConnectionState>,
    incoming_packet_sender: UnboundedSender<WrappedIncomingPacket>,
) -> Result<()> {
    let mut wait_for_state: bool = false;
    let mut state: ConnectionState = *state_receiver.borrow_and_update();

    loop {
        if wait_for_state {
            state_receiver
                .changed()
                .await
                .context("State channel closed")?;
            state = *state_receiver.borrow_and_update();
        } else if state_receiver
            .has_changed()
            .context("State channel closed")?
        {
            state = *state_receiver.borrow_and_update();
        }

        let packet: IncomingPacket = read_half
            .read_packet(&state)
            .await
            .context("Failed to read packet")?;
        wait_for_state = packet.can_change_state();

        incoming_packet_sender.send(WrappedIncomingPacket { conn_id, packet })?;
    }
}
