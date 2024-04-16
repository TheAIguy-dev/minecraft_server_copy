use strum_macros::Display;

#[derive(Display, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ConnectionState {
    #[default]
    Handshake,
    Status,
    Login,
    Play,
}
