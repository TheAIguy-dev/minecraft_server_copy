use bounded_integer::BoundedU8;

// State ids are verified

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HorizontalFacing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HopperFacing {
    Down,
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum DoorHinge {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SlabType {
    Top,
    Bottom,
    Double,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum StairsShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum RedstoneRailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum WallConnection {
    None,
    Low,
    Tall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BellAttachment {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum DripleafTilt {
    None,
    Unstable,
    Partial,
    Full,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum BedPart {
    Head,
    Foot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SculkSensorPhase {
    Inactive,
    Active,
    Cooldown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ChestType {
    Single,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum JigsawOrientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PistonType {
    Normal,
    Sticky,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HorizontalAxis {
    X,
    Z,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum NoteBlockInstrument {
    Harp,
    BaseDrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Zombie,
    Skeleton,
    Creeper,
    Dragon,
    WitherSkeleton,
    Piglin,
    CustomHead,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PointedDripstoneDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PointedDripstoneThickness {
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum RedstoneWireConnection {
    Up,
    Side,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

pub type I0_1 = BoundedU8<0, 1>;
pub type I0_2 = BoundedU8<0, 2>;
pub type I0_3 = BoundedU8<0, 3>;
pub type I0_4 = BoundedU8<0, 4>;
pub type I0_5 = BoundedU8<0, 5>;
pub type I0_6 = BoundedU8<0, 6>;
pub type I0_7 = BoundedU8<0, 7>;
pub type I0_8 = BoundedU8<0, 8>;
pub type I0_15 = BoundedU8<0, 15>;
pub type I0_24 = BoundedU8<0, 24>;
pub type I0_25 = BoundedU8<0, 25>;
pub type I1_3 = BoundedU8<1, 3>;
pub type I1_4 = BoundedU8<1, 4>;
pub type I1_7 = BoundedU8<1, 7>;
pub type I1_8 = BoundedU8<1, 8>;
