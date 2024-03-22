use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use rand::{
    distributions::{Distribution, Standard},
    random,
};
use tokio::{
    self,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    spawn,
    sync::mpsc::{error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender},
};

struct IncomingPacketWrapper {
    pub conn_id: i32,
    pub packet: IncomingPacket,
}

struct OutgoingPacketWrapper {
    pub conn_id: i32,
    pub packet: OutgoingPacket,
}

enum IncomingPacket {}

enum OutgoingPacket {}

#[tokio::main]
async fn main() {
    let addr: &str = "localhost:25565";
    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to start server at {addr:?}"));

    let (packet_channel_sender, packet_channel_receiver) = unbounded_channel();
    let (incoming_packet_sender, incoming_packet_receiver) = unbounded_channel();
    let (outgoing_packet_sender, outgoing_packet_receiver) = unbounded_channel();

    spawn(accept_connections(listener, packet_channel_sender));
    spawn(packet_distributor(
        incoming_packet_sender,
        outgoing_packet_receiver,
        packet_channel_receiver,
    ));
}

type PacketChannel = (
    UnboundedReceiver<IncomingPacket>,
    UnboundedSender<OutgoingPacket>,
);

fn gen_unique_key<T, U>(hash_map: &HashMap<T, U>) -> T
where
    T: Eq + Hash,
    Standard: Distribution<T>,
{
    let mut key: T = random();

    while hash_map.contains_key(&key) {
        key = random();
    }

    key
}

async fn packet_distributor(
    incoming_packet_sender: UnboundedSender<IncomingPacketWrapper>,
    mut outgoing_packet_receiver: UnboundedReceiver<OutgoingPacketWrapper>,
    mut packet_channel_receiver: UnboundedReceiver<PacketChannel>,
) {
    let mut connections: HashMap<i32, PacketChannel> = HashMap::new();

    loop {
        // Receive new connections
        loop {
            match packet_channel_receiver.try_recv() {
                // Receive new connection
                Ok((incoming_packet_sender, outgoing_packet_receiver)) => {
                    connections.insert(
                        gen_unique_key(&connections),
                        (incoming_packet_sender, outgoing_packet_receiver),
                    );
                }

                // No more connections
                Err(TryRecvError::Empty) => break,

                // Connection closed
                Err(TryRecvError::Disconnected) => return,
            }
        }

        let mut disconnected: HashSet<i32> = HashSet::new();

        // Distribute packets
        for (&conn_id, (incoming_packet_receiver, outgoing_packet_sender)) in &mut connections {
            // Read, wrap and send all incoming packets
            loop {
                match incoming_packet_receiver.try_recv() {
                    // Send incoming packet
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

            // Read, unwrap and send all outgoing packets
            loop {
                match outgoing_packet_receiver.try_recv() {
                    // Send outgoing packet
                    Ok(packet) => {
                        if outgoing_packet_sender.send(packet.packet).is_err() {
                            return;
                        }
                    }

                    // No more outgoing packets
                    Err(TryRecvError::Empty) => break,

                    // Connection closed
                    Err(TryRecvError::Disconnected) => {
                        disconnected.insert(conn_id);
                        break;
                    }
                }
            }
        }

        // Remove disconnected connections
        for conn_id in disconnected {
            connections.remove(&conn_id);
        }
    }
}

/// Accept all incoming connections and spawn read/write tasks.
async fn accept_connections(
    listener: TcpListener,
    packet_channel_sender: UnboundedSender<PacketChannel>,
) {
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let (read_half, write_half) = stream.into_split();

            let (incoming_packet_sender, incoming_packet_receiver) = unbounded_channel();
            let (outgoing_packet_sender, outgoing_packet_receiver) = unbounded_channel();

            spawn(read_packets(read_half, incoming_packet_sender));
            spawn(write_packets(write_half, outgoing_packet_receiver));

            if packet_channel_sender
                .send((incoming_packet_receiver, outgoing_packet_sender))
                .is_err()
            {
                break;
            }
        }
    }
}

/// Read and send all incoming packets.
async fn read_packets(
    read_half: OwnedReadHalf,
    incoming_packet_sender: UnboundedSender<IncomingPacket>,
) {
    loop {
        let packet: IncomingPacket = todo!();

        if incoming_packet_sender.send(packet).is_err() {
            break;
        }
    }
}

/// Receive and write all outgoing packets.
async fn write_packets(
    write_half: OwnedWriteHalf,
    mut outgoing_packet_receiver: UnboundedReceiver<OutgoingPacket>,
) {
    while let Some(packet) = outgoing_packet_receiver.recv().await {
        todo!();
    }
}
