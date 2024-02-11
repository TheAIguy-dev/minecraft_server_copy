#[allow(dead_code)]
#[derive(Debug)]
pub enum ConnectionState {
    Handshake,
    Status,
    Ping,
    Login,
    Play,
}
