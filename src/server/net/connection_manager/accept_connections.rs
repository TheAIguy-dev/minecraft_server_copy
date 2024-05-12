use eyre::{Context, Result};
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        watch,
    },
};

use crate::server::state::ConnectionState;

use super::{
    receive_packets::receive_packets,
    send_packets::send_packets,
    types::{ReceiveHalf, SendHalf, WrappedIncomingPacket},
};

pub async fn accept_connections(
    listener: TcpListener,
    incoming_packet_sender: UnboundedSender<WrappedIncomingPacket>,
    receive_half_sender: UnboundedSender<ReceiveHalf>,
    send_half_sender: UnboundedSender<SendHalf>,
) -> Result<()> {
    let mut conn_id: i32 = 0; // Funny thing, this actually determines entity IDs

    loop {
        // TODO: A HashSet is needed to block connections from the same IP
        if let Ok((stream, _)) = listener.accept().await {
            let (read_half, write_half) = stream.into_split();

            let (state_sender, state_receiver) = watch::channel(ConnectionState::Handshake);
            // let (incoming_packet_sender, incoming_packet_receiver) = unbounded_channel();
            let (outgoing_packet_sender, outgoing_packet_receiver) = unbounded_channel();
            let send_half: SendHalf = SendHalf {
                conn_id,
                outgoing_packet_sender,
            };
            let receive_half: ReceiveHalf = ReceiveHalf {
                conn_id,
                state_sender,
            };

            tokio::spawn(receive_packets(
                conn_id,
                read_half,
                state_receiver,
                incoming_packet_sender.clone(),
            ));
            tokio::spawn(send_packets(write_half, outgoing_packet_receiver));

            receive_half_sender
                .send(receive_half)
                .context("Receive half channel closed")?;
            send_half_sender
                .send(send_half)
                .context("Send half channel closed")?;

            conn_id += 1;
        }
    }
}
