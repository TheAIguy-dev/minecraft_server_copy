use tokio::{
    sync::{mpsc::UnboundedSender, watch},
    time::Instant,
};

use crate::server::{
    net::packets::{IncomingPacket, OutgoingPacket},
    state::ConnectionState,
};

pub struct WrappedIncomingPacket {
    pub conn_id: i32,
    pub packet: IncomingPacket,
}

pub struct WrappedOutgoingPacket {
    pub conn_id: i32,
    pub packet: OutgoingPacket,
}

pub struct WrappedConnectionState {
    pub conn_id: i32,
    pub state: ConnectionState,
}

pub struct ReceiveHalf {
    pub conn_id: i32,
    pub state_sender: watch::Sender<ConnectionState>,
}

pub struct SendHalf {
    pub conn_id: i32,
    pub outgoing_packet_sender: UnboundedSender<OutgoingPacket>,
}

pub struct Connection {
    pub last_timeout: Instant,
    pub state: ConnectionState,
}
impl Connection {
    pub fn new() -> Self {
        Self::default()
    }
    // pub fn from_state(state: ConnectionState) -> Self {
    //     Self {
    //         last_timeout: Instant::now(),
    //         state,
    //     }
    // }
}
impl Default for Connection {
    fn default() -> Self {
        Self {
            last_timeout: Instant::now(),
            state: ConnectionState::default(),
        }
    }
}
