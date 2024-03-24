#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
}
