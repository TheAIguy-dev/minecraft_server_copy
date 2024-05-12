use tokio::{
    net::TcpListener,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

use self::{
    accept_connections::accept_connections,
    receive_manager::receive_manager,
    send_manager::send_manager,
    types::{WrappedConnectionState, WrappedIncomingPacket, WrappedOutgoingPacket},
};

mod accept_connections;
mod receive_manager;
mod receive_packets;
mod send_manager;
mod send_packets;
pub mod types;

pub fn init(
    listener: TcpListener,
) -> (
    UnboundedSender<WrappedConnectionState>,
    UnboundedReceiver<WrappedIncomingPacket>,
    UnboundedSender<WrappedOutgoingPacket>,
) {
    let (state_sender, state_receiver) = unbounded_channel();
    let (incoming_packet_sender, incoming_packet_receiver) = unbounded_channel();
    let (outgoing_packet_sender, outgoing_packet_receiver) = unbounded_channel();
    let (receive_half_sender, receive_half_receiver) = unbounded_channel();
    let (send_half_sender, send_half_receiver) = unbounded_channel();

    tokio::spawn(accept_connections(
        listener,
        incoming_packet_sender,
        receive_half_sender,
        send_half_sender,
    ));
    tokio::spawn(receive_manager(state_receiver, receive_half_receiver));
    tokio::spawn(send_manager(outgoing_packet_receiver, send_half_receiver));

    (
        state_sender,
        incoming_packet_receiver,
        outgoing_packet_sender,
    )
}
