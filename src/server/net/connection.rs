use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use eyre::{eyre, Context, Result};
use log::debug;
use tokio::{
    self,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    spawn,
    sync::mpsc::{self, error::TryRecvError},
    sync::watch,
};

use super::packets::{IncomingPacket, OutgoingPacket, ReadPacket, WritePacket};
use crate::server::{state::ConnectionState, util::gen_unique_key};

pub struct IncomingPacketWrapper {
    pub conn_id: i32,
    pub packet: IncomingPacket,
}

pub struct OutgoingPacketWrapper {
    pub conn_id: i32,
    pub packet: OutgoingPacket,
}

pub struct ConnectionStateWrapper {
    pub conn_id: i32,
    pub state: ConnectionState,
}

pub struct _Connection {
    pub state_sender: watch::Sender<ConnectionState>,
    pub incoming_packet_receiver: mpsc::UnboundedReceiver<IncomingPacket>,
    pub outgoing_packet_sender: mpsc::UnboundedSender<OutgoingPacket>,
}

pub struct Connection {
    pub last_timeout: Instant,
    pub state: ConnectionState,
}
impl Connection {
    pub fn from_state(state: ConnectionState) -> Self {
        Self {
            last_timeout: Instant::now(),
            state,
        }
    }
}
impl Default for Connection {
    fn default() -> Self {
        Self {
            last_timeout: Instant::now(),
            state: ConnectionState::default(),
        }
    }
}

pub async fn data_distributor(
    mut connection_receiver: mpsc::UnboundedReceiver<_Connection>,
    mut state_receiver: mpsc::UnboundedReceiver<ConnectionStateWrapper>,
    incoming_packet_sender: mpsc::UnboundedSender<IncomingPacketWrapper>,
    mut outgoing_packet_receiver: mpsc::UnboundedReceiver<OutgoingPacketWrapper>,
) -> Result<()> {
    let mut connections: HashMap<i32, _Connection> = HashMap::new();

    loop {
        // Receive new connections
        loop {
            match connection_receiver.try_recv() {
                // Receive new connection
                Ok(connection) => {
                    connections.insert(gen_unique_key(&connections), connection);
                }

                // No more connections
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => return Err(eyre!("Connection channel closed")),
            }
        }

        // Distribute states
        loop {
            match state_receiver.try_recv() {
                // Send state
                Ok(state) => {
                    if let Some(connection) = connections.get(&state.conn_id) {
                        if connection.state_sender.send(state.state).is_err() {
                            connections.remove(&state.conn_id);
                        }
                    }
                }

                // No more states
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => {
                    return Err(eyre!("State channel closed"));
                }
            }
        }

        // Receive incoming packets
        let mut disconnected: HashSet<i32> = HashSet::new();
        for (&conn_id, connection) in &mut connections {
            loop {
                match connection.incoming_packet_receiver.try_recv() {
                    // Receive incoming packet
                    Ok(packet) => {
                        debug!("[DATA_DISTRIBUTOR] Sending {packet}");
                        incoming_packet_sender
                            .send(IncomingPacketWrapper { conn_id, packet })
                            .context("Incoming packet channel closed")?
                    }

                    // No more incoming packets
                    Err(TryRecvError::Empty) => break,

                    // Connection closed
                    Err(TryRecvError::Disconnected) => {
                        debug!("[DATA_DISTRIBUTOR] Incoming packet channel closed");
                        disconnected.insert(conn_id);
                        break;
                    }
                }
            }
        }
        for conn_id in disconnected {
            debug!("[DATA_DISTRIBUTOR] Removing {conn_id}");
            connections.remove(&conn_id);
        }

        // Distribute outgoing packets
        loop {
            match outgoing_packet_receiver.try_recv() {
                // Send outgoing packet
                Ok(packet) => {
                    if let Some(connection) = connections.get(&packet.conn_id) {
                        if packet.packet == OutgoingPacket::Drop
                            || connection
                                .outgoing_packet_sender
                                .send(packet.packet)
                                .is_err()
                        {
                            connections.remove(&packet.conn_id);
                        }
                    }
                }

                // No more outgoing packets
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => {
                    return Err(eyre!("Outgoing packet channel closed"));
                }
            }
        }
    }
}

pub async fn accept_connections(
    listener: TcpListener,
    connection_sender: mpsc::UnboundedSender<_Connection>,
) -> Result<()> {
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let (read_half, write_half) = stream.into_split();

            let (state_sender, state_receiver) = watch::channel(ConnectionState::Handshake);
            let (incoming_packet_sender, incoming_packet_receiver) = mpsc::unbounded_channel();
            let (outgoing_packet_sender, outgoing_packet_receiver) = mpsc::unbounded_channel();

            state_sender.send(ConnectionState::Handshake)?;

            spawn(read_packets(
                read_half,
                state_receiver,
                incoming_packet_sender,
            ));
            spawn(write_packets(write_half, outgoing_packet_receiver));

            connection_sender
                .send(_Connection {
                    state_sender,
                    incoming_packet_receiver,
                    outgoing_packet_sender,
                })
                .context("Connection channel closed")?;
        }
    }
}

pub async fn read_packets(
    mut read_half: OwnedReadHalf,
    mut state_receiver: watch::Receiver<ConnectionState>,
    incoming_packet_sender: mpsc::UnboundedSender<IncomingPacket>,
) -> Result<()> {
    let mut state: ConnectionState = *state_receiver.borrow();
    // while let Ok(()) = state_receiver.changed().await {
    loop {
        // debug!("Reading, state {state}");

        if state != ConnectionState::Play {
            debug!("[READ_PACKETS] State not play, waiting for new state");
            if let Ok(()) = state_receiver.changed().await {
                // if has_changed {
                state = *state_receiver.borrow_and_update();
            } else {
                break;
            }
            debug!("[READ_PACKETS] Received state {state}");
        } else {
            if let Ok(has_changed) = state_receiver.has_changed() {
                if has_changed {
                    state = *state_receiver.borrow_and_update();
                }
            } else {
                break;
            }
        }

        debug!("[READ_PACKETS] Starting read with state {state}");
        let packet: IncomingPacket = read_half.read_packet(&state).await?;
        debug!("[READ_PACKETS] Received packet {packet}");

        debug!(
            "[READ_PACKETS] Is channel closed: {}",
            incoming_packet_sender.is_closed()
        );
        if let Err(e) = incoming_packet_sender.send(packet) {
            debug!("[READ_PACKETS] Unable to send packet to channel: {e}");
            break;
        }
    }

    Ok(())
}

pub async fn write_packets(
    mut write_half: OwnedWriteHalf,
    mut outgoing_packet_receiver: mpsc::UnboundedReceiver<OutgoingPacket>,
) -> Result<()> {
    while let Some(packet) = outgoing_packet_receiver.recv().await {
        write_half.write_packet(packet).await?;
    }

    Ok(())
}
