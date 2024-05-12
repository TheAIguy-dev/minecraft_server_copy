use std::collections::HashMap;

use eyre::{eyre, Result};
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::server::net::packets::OutgoingPacket;

use super::types::{SendHalf, WrappedOutgoingPacket};

pub async fn send_manager(
    mut outgoing_packet_receiver: UnboundedReceiver<WrappedOutgoingPacket>,
    mut send_half_receiver: UnboundedReceiver<SendHalf>,
) -> Result<()> {
    let mut send_halves: HashMap<i32, UnboundedSender<OutgoingPacket>> = HashMap::new();

    loop {
        select! {
            Some(WrappedOutgoingPacket { conn_id, packet }) = outgoing_packet_receiver.recv() => {
                let mut remove = false;

                if let Some(outgoing_packet_sender) = send_halves.get(&conn_id) {
                    if packet == OutgoingPacket::Disconnect || outgoing_packet_sender.send(packet).is_err() {
                        remove = true;
                    }
                }

                if remove {
                    send_halves.remove(&conn_id);
                }
            },

            Some(SendHalf { conn_id, outgoing_packet_sender }) = send_half_receiver.recv() => {
                send_halves.insert(conn_id, outgoing_packet_sender);
            },

            else => return Err(eyre!("Channel was closed")),
        }
    }
}
