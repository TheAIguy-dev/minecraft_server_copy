use std::collections::{HashMap, HashSet};

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

pub struct PlayerConnection {
    pub state_sender: watch::Sender<ConnectionState>,
    pub incoming_packet_receiver: mpsc::UnboundedReceiver<IncomingPacket>,
    pub outgoing_packet_sender: mpsc::UnboundedSender<OutgoingPacket>,
}

pub async fn data_distributor(
    mut connection_receiver: mpsc::UnboundedReceiver<PlayerConnection>,
    mut state_receiver: mpsc::UnboundedReceiver<ConnectionStateWrapper>,
    incoming_packet_sender: mpsc::UnboundedSender<IncomingPacketWrapper>,
    mut outgoing_packet_receiver: mpsc::UnboundedReceiver<OutgoingPacketWrapper>,
) {
    let mut connections: HashMap<i32, PlayerConnection> = HashMap::new();

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
                Err(TryRecvError::Disconnected) => return,
            }
        }

        // Distribute states
        loop {
            match state_receiver.try_recv() {
                // Send state
                Ok(state) => {
                    if connections
                        .get(&state.conn_id)
                        .unwrap()
                        .state_sender
                        .send(state.state)
                        .is_err()
                    {
                        connections.remove(&state.conn_id);
                    }
                }

                // No more states
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => {
                    return;
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
                        if incoming_packet_sender
                            .send(IncomingPacketWrapper { conn_id, packet })
                            .is_err()
                        {
                            return;
                        }
                    }

                    // No more incoming packets
                    Err(TryRecvError::Empty) => break,

                    // Connection closed
                    Err(TryRecvError::Disconnected) => {
                        disconnected.insert(conn_id);
                        break;
                    }
                }
            }
        }
        for conn_id in disconnected {
            connections.remove(&conn_id);
        }

        // Distribute outgoing packets
        loop {
            match outgoing_packet_receiver.try_recv() {
                // Send outgoing packet
                Ok(packet) => {
                    if connections
                        .get(&packet.conn_id)
                        .unwrap()
                        .outgoing_packet_sender
                        .send(packet.packet)
                        .is_err()
                    {
                        connections.remove(&packet.conn_id);
                    }
                }

                // No more outgoing packets
                Err(TryRecvError::Empty) => break,

                // Channel closed
                Err(TryRecvError::Disconnected) => {
                    return;
                }
            }
        }
    }
}

pub async fn accept_connections(
    listener: TcpListener,
    connection_sender: mpsc::UnboundedSender<PlayerConnection>,
) {
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let (read_half, write_half) = stream.into_split();

            let (state_sender, state_receiver) = watch::channel(ConnectionState::Handshake);
            let (incoming_packet_sender, incoming_packet_receiver) = mpsc::unbounded_channel();
            let (outgoing_packet_sender, outgoing_packet_receiver) = mpsc::unbounded_channel();

            spawn(read_packets(
                read_half,
                state_receiver,
                incoming_packet_sender,
            ));
            spawn(write_packets(write_half, outgoing_packet_receiver));

            if connection_sender
                .send(PlayerConnection {
                    state_sender,
                    incoming_packet_receiver,
                    outgoing_packet_sender,
                })
                .is_err()
            {
                break;
            }
        }
    }
}

pub async fn read_packets(
    mut read_half: OwnedReadHalf,
    mut state_receiver: watch::Receiver<ConnectionState>,
    incoming_packet_sender: mpsc::UnboundedSender<IncomingPacket>,
) {
    let mut state: ConnectionState = *state_receiver.borrow_and_update();
    loop {
        if let Ok(has_changed) = state_receiver.has_changed() {
            if has_changed {
                state = *state_receiver.borrow_and_update();
            }
        } else {
            break;
        }

        let packet: IncomingPacket = read_half.read_packet(&state).await;

        if incoming_packet_sender.send(packet).is_err() {
            break;
        }
    }
}

pub async fn write_packets(
    mut write_half: OwnedWriteHalf,
    mut outgoing_packet_receiver: mpsc::UnboundedReceiver<OutgoingPacket>,
) {
    while let Some(packet) = outgoing_packet_receiver.recv().await {
        write_half.write_packet(packet).await;
    }
}
