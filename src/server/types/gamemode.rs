#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}
