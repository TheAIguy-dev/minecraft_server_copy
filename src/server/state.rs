use strum_macros::Display;

#[derive(Display, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// This state indicates that the connection should be closed.
    Disconnect,
    #[default]
    Handshake,
    Status,
    Login,
    Play,
}
