#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Handshake,
    Status,
    Ping,
    Login,
    Play,
}
