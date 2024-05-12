use std::collections::HashMap;

use eyre::{eyre, Result};
use tokio::{
    select,
    sync::{mpsc::UnboundedReceiver, watch},
};

use crate::server::state::ConnectionState;

use super::types::{ReceiveHalf, WrappedConnectionState};

pub async fn receive_manager(
    mut state_receiver: UnboundedReceiver<WrappedConnectionState>,
    mut receive_half_receiver: UnboundedReceiver<ReceiveHalf>,
) -> Result<()> {
    let mut receive_halves: HashMap<i32, watch::Sender<ConnectionState>> = HashMap::new();

    loop {
        select! {
            Some(WrappedConnectionState { conn_id, state }) = state_receiver.recv() => {
                let mut remove = false;

                if let Some(state_sender) = receive_halves.get(&conn_id) {
                    if state == ConnectionState::Disconnect || state_sender.send(state).is_err() {
                        remove = true;
                    }
                }

                if remove {
                    receive_halves.remove(&conn_id);
                }
            },

            Some(ReceiveHalf { conn_id, state_sender }) = receive_half_receiver.recv() => {
                receive_halves.insert(conn_id, state_sender);
            },

            else => return Err(eyre!("Channel was closed")),
        }
    }
}
