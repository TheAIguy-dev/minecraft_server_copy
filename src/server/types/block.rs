use super::{
    Axis, BambooLeaves, BedPart, BellAttachment, ChestType, ComparatorMode, DoorHinge,
    DripleafTilt, Face, Facing, Half, HopperFacing, HorizontalAxis, HorizontalFacing,
    JigsawOrientation, NoteBlockInstrument, PistonType, PointedDripstoneDirection,
    PointedDripstoneThickness, RailShape, RedstoneRailShape, RedstoneWireConnection,
    SculkSensorPhase, SlabType, StairsShape, StructureBlockMode, WallConnection, I0_1, I0_15, I0_2,
    I0_24, I0_25, I0_3, I0_4, I0_5, I0_6, I0_7, I0_8, I1_3, I1_4, I1_7, I1_8,
};

// TODO: Write macros to avoid repeating code

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u16)]
#[allow(clippy::enum_variant_names)]
pub enum Block {
    Air = 0,
    Stone = 1,
    Granite = 2,
    PolishedGranite = 3,
    Diorite = 4,
    PolishedDiorite = 5,
    Andesite = 6,
    PolishedAndesite = 7,
    GrassBlock {
        snowy: bool,
    } = 8,
    Dirt = 10,
    CoarseDirt = 11,
    Podzol {
        snowy: bool,
    } = 12,
    Cobblestone = 14,
    OakPlanks = 15,
    SprucePlanks = 16,
    BirchPlanks = 17,
    JunglePlanks = 18,
    AcaciaPlanks = 19,
    CherryPlanks = 20,
    DarkOakPlanks = 21,
    MangrovePlanks = 22,
    BambooPlanks = 23,
    BambooMosaic = 24,
    OakSapling {
        stage: I0_1,
    } = 25,
    SpruceSapling {
        stage: I0_1,
    } = 27,
    BirchSapling {
        stage: I0_1,
    } = 29,
    JungleSapling {
        stage: I0_1,
    } = 31,
    AcaciaSapling {
        stage: I0_1,
    } = 33,
    CherrySapling {
        stage: I0_1,
    } = 35,
    DarkOakSapling {
        stage: I0_1,
    } = 37,
    MangrovePropagule {
        stage: I0_1,
        age: I0_4,
        waterlogged: bool,
        hanging: bool,
    } = 39,
    Bedrock = 79,
    Water {
        level: I0_15,
    } = 80,
    Lava {
        level: I0_15,
    } = 96,
    Sand = 112,
    SuspiciousSand {
        dusted: I0_3,
    } = 113,
    RedSand = 117,
    Gravel = 118,
    SuspiciousGravel {
        dusted: I0_3,
    } = 119,
    GoldOre = 123,
    DeepslateGoldOre = 124,
    IronOre = 125,
    DeepslateIronOre = 126,
    CoalOre = 127,
    DeepslateCoalOre = 128,
    NetherGoldOre = 129,
    OakLog {
        axis: Axis,
    } = 130,
    SpruceLog {
        axis: Axis,
    } = 133,
    BirchLog {
        axis: Axis,
    } = 136,
    JungleLog {
        axis: Axis,
    } = 139,
    AcaciaLog {
        axis: Axis,
    } = 142,
    CherryLog {
        axis: Axis,
    } = 145,
    DarkOakLog {
        axis: Axis,
    } = 148,
    MangroveLog {
        axis: Axis,
    } = 151,
    MangroveRoots {
        waterlogged: bool,
    } = 154,
    MuddyMangroveRoots {
        axis: Axis,
    } = 156,
    BambooBlock {
        axis: Axis,
    } = 159,
    StrippedSpruceLog {
        axis: Axis,
    } = 162,
    StrippedBirchLog {
        axis: Axis,
    } = 165,
    StrippedJungleLog {
        axis: Axis,
    } = 168,
    StrippedAcaciaLog {
        axis: Axis,
    } = 171,
    StrippedCherryLog {
        axis: Axis,
    } = 174,
    StrippedDarkOakLog {
        axis: Axis,
    } = 177,
    StrippedOakLog {
        axis: Axis,
    } = 180,
    StrippedMangroveLog {
        axis: Axis,
    } = 183,
    StrippedBambooBlock {
        axis: Axis,
    } = 186,
    OakWood {
        axis: Axis,
    } = 189,
    SpruceWood {
        axis: Axis,
    } = 192,
    BirchWood {
        axis: Axis,
    } = 195,
    JungleWood {
        axis: Axis,
    } = 198,
    AcaciaWood {
        axis: Axis,
    } = 201,
    CherryWood {
        axis: Axis,
    } = 204,
    DarkOakWood {
        axis: Axis,
    } = 207,
    MangroveWood {
        axis: Axis,
    } = 210,
    StrippedOakWood {
        axis: Axis,
    } = 213,
    StrippedSpruceWood {
        axis: Axis,
    } = 216,
    StrippedBirchWood {
        axis: Axis,
    } = 219,
    StrippedJungleWood {
        axis: Axis,
    } = 222,
    StrippedAcaciaWood {
        axis: Axis,
    } = 225,
    StrippedCherryWood {
        axis: Axis,
    } = 228,
    StrippedDarkOakWood {
        axis: Axis,
    } = 231,
    StrippedMangroveWood {
        axis: Axis,
    } = 234,
    OakLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 237,
    SpruceLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 265,
    BirchLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 293,
    JungleLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 321,
    AcaciaLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 349,
    CherryLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 377,
    DarkOakLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 405,
    MangroveLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 433,
    AzaleaLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 461,
    FloweringAzaleaLeaves {
        distance: I1_7,
        persistent: bool,
        waterlogged: bool,
    } = 489,
    Sponge = 517,
    WetSponge = 518,
    Glass = 519,
    LapisOre = 520,
    DeepslateLapisOre = 521,
    LapisBlock = 522,
    Dispenser {
        facing: Facing,
        triggered: bool,
    } = 523,
    Sandstone = 535,
    ChiseledSandstone = 536,
    CutSandstone = 537,
    NoteBlock {
        instrument: NoteBlockInstrument,
        powered: bool,
        note: I0_24,
    } = 538,
    WhiteBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1688,
    OrangeBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1704,
    MagentaBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1720,
    LightBlueBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1736,
    YellowBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1752,
    LimeBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1768,
    PinkBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1784,
    GrayBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1800,
    LightGrayBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1816,
    CyanBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1832,
    PurpleBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1848,
    BlueBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1864,
    BrownBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1880,
    GreenBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1896,
    RedBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1912,
    BlackBed {
        facing: HorizontalFacing,
        part: BedPart,
        occupied: bool,
    } = 1928,
    PoweredRail {
        shape: RedstoneRailShape,
        powered: bool,
        waterlogged: bool,
    } = 1944,
    DetectorRail {
        shape: RedstoneRailShape,
        powered: bool,
        waterlogged: bool,
    } = 1968,
    StickyPiston {
        facing: Facing,
        extended: bool,
    } = 1992,
    Cobweb = 2004,
    Grass = 2005,
    Fern = 2006,
    DeadBush = 2007,
    Seagrass = 2008,
    TallSeagrass {
        half: Half,
    } = 2009,
    Piston {
        facing: Facing,
        extended: bool,
    } = 2011,
    PistonHead {
        facing: Facing,
        r#type: PistonType,
        short: bool,
    } = 2023,
    WhiteWool = 2047,
    OrangeWool = 2048,
    MagentaWool = 2049,
    LightBlueWool = 2050,
    YellowWool = 2051,
    LimeWool = 2052,
    PinkWool = 2053,
    GrayWool = 2054,
    LightGrayWool = 2055,
    CyanWool = 2056,
    PurpleWool = 2057,
    BlueWool = 2058,
    BrownWool = 2059,
    GreenWool = 2060,
    RedWool = 2061,
    BlackWool = 2062,
    MovingPiston {
        facing: Facing,
        r#type: PistonType,
    } = 2063,
    Dandelion = 2075,
    Torchflower = 2076,
    Poppy = 2077,
    BlueOrchid = 2078,
    Allium = 2079,
    AzureBluet = 2080,
    RedTulip = 2081,
    OrangeTulip = 2082,
    WhiteTulip = 2083,
    PinkTulip = 2084,
    OxeyeDaisy = 2085,
    Cornflower = 2086,
    WitherRose = 2087,
    LilyOfTheValley = 2088,
    BrownMushroom = 2089,
    RedMushroom = 2090,
    GoldBlock = 2091,
    IronBlock = 2092,
    Bricks = 2093,
    Tnt {
        unstable: bool,
    } = 2094,
    Bookshelf = 2096,
    ChiseledBookshelf {
        facing: HorizontalFacing,
        slot_0_occupied: bool,
        slot_1_occupied: bool,
        slot_2_occupied: bool,
        slot_3_occupied: bool,
        slot_4_occupied: bool,
        slot_5_occupied: bool,
    } = 2097,
    MossyCobblestone = 2353,
    Obsidian = 2354,
    Torch = 2355,
    WallTorch {
        facing: HorizontalFacing,
    } = 2356,
    Fire {
        age: I0_15,
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
    } = 2360,
    SoulFire = 2872,
    Spawner = 2873,
    OakStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 2874,
    Chest {
        facing: HorizontalFacing,
        r#type: ChestType,
        waterlogged: bool,
    } = 2954,
    RedstoneWire {
        north: RedstoneWireConnection,
        east: RedstoneWireConnection,
        south: RedstoneWireConnection,
        west: RedstoneWireConnection,
        power: I0_15,
    } = 2978,
    DiamondOre = 4274,
    DeepslateDiamondOre = 4275,
    DiamondBlock = 4276,
    CraftingTable = 4277,
    Wheat {
        age: I0_7,
    } = 4278,
    Farmland {
        moisture: I0_7,
    } = 4286,
    Furnace {
        facing: HorizontalFacing,
        lit: bool,
    } = 4294,
    OakSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4302,
    SpruceSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4334,
    BirchSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4366,
    AcaciaSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4398,
    CherrySign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4430,
    JungleSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4462,
    DarkOakSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4494,
    MangroveSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4526,
    BambooSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 4558,
    OakDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 4590,
    Ladder {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4654,
    Rail {
        shape: RailShape,
        waterlogged: bool,
    } = 4662,
    CobblestoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 4682,
    OakWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4762,
    SpruceWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4770,
    BirchWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4778,
    AcaciaWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4786,
    CherryWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4794,
    JungleWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4802,
    DarkOakWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4810,
    MangroveWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4818,
    BambooWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 4826,
    OakHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 4834,
    SpruceHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 4898,
    BirchHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 4962,
    AcaciaHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5026,
    CherryHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5090,
    JungleHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5154,
    DarkOakHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5218,
    CrimsonHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5282,
    WarpedHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5346,
    MangroveHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5410,
    BambooHangingSign {
        rotation: I0_15,
        attached: bool,
        waterlogged: bool,
    } = 5474,
    OakWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5538,
    SpruceWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5546,
    BirchWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5554,
    AcaciaWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5562,
    CherryWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5570,
    JungleWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5578,
    DarkOakWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5586,
    MangroveWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5594,
    CrimsonWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5602,
    WarpedWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5610,
    BambooWallHangingSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 5618,
    Lever {
        face: Face,
        facing: HorizontalFacing,
        powered: bool,
    } = 5626,
    StonePressurePlate {
        powered: bool,
    } = 5650,
    IronDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 5652,
    OakPressurePlate {
        powered: bool,
    } = 5716,
    SprucePressurePlate {
        powered: bool,
    } = 5718,
    BirchPressurePlate {
        powered: bool,
    } = 5720,
    JunglePressurePlate {
        powered: bool,
    } = 5722,
    AcaciaPressurePlate {
        powered: bool,
    } = 5724,
    CherryPressurePlate {
        powered: bool,
    } = 5726,
    DarkOakPressurePlate {
        powered: bool,
    } = 5728,
    MangrovePressurePlate {
        powered: bool,
    } = 5730,
    BambooPressurePlate {
        powered: bool,
    } = 5732,
    RedstoneOre {
        lit: bool,
    } = 5734,
    DeepslateRedstoneOre {
        lit: bool,
    } = 5736,
    RedstoneTorch {
        lit: bool,
    } = 5738,
    RedstoneWallTorch {
        facing: HorizontalFacing,
        lit: bool,
    } = 5740,
    StoneButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 5748,
    Snow {
        layers: I1_8,
    } = 5772,
    Ice = 5780,
    SnowBlock = 5781,
    Cactus {
        age: I0_15,
    } = 5782,
    Clay = 5798,
    SugarCane {
        age: I0_15,
    } = 5799,
    Jukebox {
        has_record: bool,
    } = 5815,
    OakFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 5817,
    Pumpkin = 5849,
    Netherrack = 5850,
    SoulSand = 5851,
    SoulSoil = 5852,
    Basalt {
        axis: Axis,
    } = 5853,
    PolishedBasalt {
        axis: Axis,
    } = 5856,
    SoulTorch = 5859,
    SoulWallTorch {
        facing: HorizontalFacing,
    } = 5860,
    Glowstone = 5864,
    NetherPortal {
        axis: HorizontalAxis,
    } = 5865,
    CarvedPumpkin {
        facing: HorizontalFacing,
    } = 5867,
    JackOLantern {
        facing: HorizontalFacing,
    } = 5871,
    Cake {
        bites: I0_6,
    } = 5875,
    Repeater {
        facing: HorizontalFacing,
        delay: I1_4,
        locked: bool,
        powered: bool,
    } = 5882,
    WhiteStainedGlass = 5946,
    OrangeStainedGlass = 5947,
    MagentaStainedGlass = 5948,
    LightBlueStainedGlass = 5949,
    YellowStainedGlass = 5950,
    LimeStainedGlass = 5951,
    PinkStainedGlass = 5952,
    GrayStainedGlass = 5953,
    LightGrayStainedGlass = 5954,
    CyanStainedGlass = 5955,
    PurpleStainedGlass = 5956,
    BlueStainedGlass = 5957,
    BrownStainedGlass = 5958,
    GreenStainedGlass = 5959,
    RedStainedGlass = 5960,
    BlackStainedGlass = 5961,
    OakTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 5962,
    SpruceTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6026,
    BirchTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6090,
    JungleTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6154,
    AcaciaTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6218,
    CherryTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6282,
    DarkOakTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6346,
    MangroveTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6410,
    BambooTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 6474,
    StoneBricks = 6538,
    MossyStoneBricks = 6539,
    CrackedStoneBricks = 6540,
    ChiseledStoneBricks = 6541,
    PackedMud = 6542,
    MudBricks = 6543,
    InfestedStone = 6544,
    InfestedCobblestone = 6545,
    InfestedStoneBricks = 6546,
    InfestedMossyStoneBricks = 6547,
    InfestedCrackedStoneBricks = 6548,
    InfestedChiseledStoneBricks = 6549,
    BrownMushroomBlock {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
    } = 6550,
    RedMushroomBlock {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
    } = 6614,
    MushroomStem {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
    } = 6678,
    IronBars {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 6742,
    Chain {
        axis: Axis,
        waterlogged: bool,
    } = 6774,
    GlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 6780,
    Melon = 6812,
    AttachedPumpkinStem {
        facing: HorizontalFacing,
    } = 6813,
    AttachedMelonStem {
        facing: HorizontalFacing,
    } = 6817,
    PumpkinStem {
        age: I0_7,
    } = 6821,
    MelonStem {
        age: I0_7,
    } = 6829,
    Vine {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
    } = 6837,
    GlowLichen {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
        waterlogged: bool,
    } = 6869,
    OakFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 6997,
    BrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7029,
    StoneBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7109,
    MudBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7189,
    Mycelium {
        snowy: bool,
    } = 7269,
    LilyPad = 7271,
    NetherBricks = 7272,
    NetherBrickFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 7273,
    NetherBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7305,
    NetherWart {
        age: I0_3,
    } = 7385,
    EnchantingTable = 7389,
    BrewingStand {
        has_bottle_0: bool,
        has_bottle_1: bool,
        has_bottle_2: bool,
    } = 7390,
    Cauldron = 7398,
    WaterCauldron {
        level: I1_3,
    } = 7399,
    LavaCauldron = 7402,
    PowderSnowCauldron {
        level: I1_3,
    } = 7403,
    EndPortal = 7406,
    EndPortalFrame {
        facing: HorizontalFacing,
        eye: bool,
    } = 7407,
    EndStone = 7415,
    DragonEgg = 7416,
    RedstoneLamp {
        lit: bool,
    } = 7417,
    Cocoa {
        facing: HorizontalFacing,
        age: I0_2,
    } = 7419,
    SandstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7431,
    EmeraldOre = 7511,
    DeepslateEmeraldOre = 7512,
    EnderChest {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 7513,
    TripwireHook {
        facing: HorizontalFacing,
        powered: bool,
        attached: bool,
    } = 7521,
    Tripwire {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        powered: bool,
        attached: bool,
        disarmed: bool,
    } = 7537,
    EmeraldBlock = 7665,
    SpruceStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7666,
    BirchStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7746,
    JungleStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 7826,
    CommandBlock {
        facing: Facing,
        conditional: bool,
    } = 7906,
    Beacon = 7918,
    CobblestoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 7919,
    MossyCobblestoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 8243,
    FlowerPot = 8567,
    PottedTorchflower = 8568,
    PottedOakSapling = 8569,
    PottedSpruceSapling = 8570,
    PottedBirchSapling = 8571,
    PottedJungleSapling = 8572,
    PottedAcaciaSapling = 8573,
    PottedCherrySapling = 8574,
    PottedDarkOakSapling = 8575,
    PottedMangrovePropagule = 8576,
    PottedFern = 8577,
    PottedDandelion = 8578,
    PottedPoppy = 8579,
    PottedBlueOrchid = 8580,
    PottedAllium = 8581,
    PottedAzureBluet = 8582,
    PottedRedTulip = 8583,
    PottedOrangeTulip = 8584,
    PottedWhiteTulip = 8585,
    PottedPinkTulip = 8586,
    PottedOxeyeDaisy = 8587,
    PottedCornflower = 8588,
    PottedLilyOfTheValley = 8589,
    PottedWitherRose = 8590,
    PottedRedMushroom = 8591,
    PottedBrownMushroom = 8592,
    PottedDeadBush = 8593,
    PottedCactus = 8594,
    Carrots {
        age: I0_7,
    } = 8595,
    Potatoes {
        age: I0_7,
    } = 8603,
    OakButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8611,
    SpruceButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8635,
    BirchButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8659,
    JungleButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8683,
    AcaciaButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8707,
    CherryButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8731,
    DarkOakButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8755,
    MangroveButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8779,
    BambooButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 8803,
    SkeletonSkull {
        rotation: I0_15,
    } = 8827,
    SkeletonWallSkull {
        facing: HorizontalFacing,
    } = 8843,
    WitherSkeletonSkull {
        rotation: I0_15,
    } = 8847,
    WitherSkeletonWallSkull {
        facing: HorizontalFacing,
    } = 8863,
    ZombieHead {
        rotation: I0_15,
    } = 8867,
    ZombieWallHead {
        facing: HorizontalFacing,
    } = 8883,
    PlayerHead {
        rotation: I0_15,
    } = 8887,
    PlayerWallHead {
        facing: HorizontalFacing,
    } = 8903,
    CreeperHead {
        rotation: I0_15,
    } = 8907,
    CreeperWallHead {
        facing: HorizontalFacing,
    } = 8923,
    DragonHead {
        rotation: I0_15,
    } = 8927,
    DragonWallHead {
        facing: HorizontalFacing,
    } = 8943,
    PiglinHead {
        rotation: I0_15,
    } = 8947,
    PiglinWallHead {
        facing: HorizontalFacing,
    } = 8963,
    Anvil {
        facing: HorizontalFacing,
    } = 8967,
    ChippedAnvil {
        facing: HorizontalFacing,
    } = 8971,
    DamagedAnvil {
        facing: HorizontalFacing,
    } = 8975,
    TrappedChest {
        facing: HorizontalFacing,
        r#type: ChestType,
        waterlogged: bool,
    } = 8979,
    LightWeightedPressurePlate {
        power: I0_15,
    } = 9003,
    HeavyWeightedPressurePlate {
        power: I0_15,
    } = 9019,
    Comparator {
        facing: HorizontalFacing,
        mode: ComparatorMode,
        powered: bool,
    } = 9035,
    DaylightDetector {
        power: I0_15,
        inverted: bool,
    } = 9051,
    RedstoneBlock = 9083,
    NetherQuartzOre = 9084,
    Hopper {
        facing: HopperFacing,
        enabled: bool,
    } = 9085,
    QuartzBlock = 9095,
    ChiseledQuartzBlock = 9096,
    QuartzPillar {
        axis: Axis,
    } = 9097,
    QuartzStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 9100,
    ActivatorRail {
        shape: RedstoneRailShape,
        powered: bool,
        waterlogged: bool,
    } = 9180,
    Dropper {
        facing: Facing,
        triggered: bool,
    } = 9204,
    WhiteTerracotta = 9216,
    OrangeTerracotta = 9217,
    MagentaTerracotta = 9218,
    LightBlueTerracotta = 9219,
    YellowTerracotta = 9220,
    LimeTerracotta = 9221,
    PinkTerracotta = 9222,
    GrayTerracotta = 9223,
    LightGrayTerracotta = 9224,
    CyanTerracotta = 9225,
    PurpleTerracotta = 9226,
    BlueTerracotta = 9227,
    BrownTerracotta = 9228,
    GreenTerracotta = 9229,
    RedTerracotta = 9230,
    BlackTerracotta = 9231,
    WhiteStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9232,
    OrangeStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9264,
    MagentaStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9296,
    LightBlueStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9328,
    YellowStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9360,
    LimeStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9392,
    PinkStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9424,
    GrayStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9456,
    LightGrayStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9488,
    CyanStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9520,
    PurpleStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9552,
    BlueStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9584,
    BrownStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9616,
    GreenStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9648,
    RedStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9680,
    BlackStainedGlassPane {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 9712,
    AcaciaStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 9744,
    CherryStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 9824,
    DarkOakStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 9904,
    MangroveStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 9984,
    BambooStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10064,
    BambooMosaicStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10144,
    SlimeBlock = 10224,
    Barrier = 10225,
    Light {
        level: I0_15,
        waterlogged: bool,
    } = 10226,
    IronTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 10258,
    Prismarine = 10322,
    PrismarineBricks = 10323,
    DarkPrismarine = 10324,
    PrismarineStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10325,
    PrismarineBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10405,
    DarkPrismarineStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10485,
    PrismarineSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 10565,
    PrismarineBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 10571,
    DarkPrismarineSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 10577,
    SeaLantern = 10583,
    HayBlock {
        axis: Axis,
    } = 10584,
    WhiteCarpet = 10587,
    OrangeCarpet = 10588,
    MagentaCarpet = 10589,
    LightBlueCarpet = 10590,
    YellowCarpet = 10591,
    LimeCarpet = 10592,
    PinkCarpet = 10593,
    GrayCarpet = 10594,
    LightGrayCarpet = 10595,
    CyanCarpet = 10596,
    PurpleCarpet = 10597,
    BlueCarpet = 10598,
    BrownCarpet = 10599,
    GreenCarpet = 10600,
    RedCarpet = 10601,
    BlackCarpet = 10602,
    Terracotta = 10603,
    CoalBlock = 10604,
    PackedIce = 10605,
    Sunflower {
        half: Half,
    } = 10606,
    Lilac {
        half: Half,
    } = 10608,
    RoseBush {
        half: Half,
    } = 10610,
    Peony {
        half: Half,
    } = 10612,
    TallGrass {
        half: Half,
    } = 10614,
    LargeFern {
        half: Half,
    } = 10616,
    WhiteBanner {
        rotation: I0_15,
    } = 10618,
    OrangeBanner {
        rotation: I0_15,
    } = 10634,
    MagentaBanner {
        rotation: I0_15,
    } = 10650,
    LightBlueBanner {
        rotation: I0_15,
    } = 10666,
    YellowBanner {
        rotation: I0_15,
    } = 10682,
    LimeBanner {
        rotation: I0_15,
    } = 10698,
    PinkBanner {
        rotation: I0_15,
    } = 10714,
    GrayBanner {
        rotation: I0_15,
    } = 10730,
    LightGrayBanner {
        rotation: I0_15,
    } = 10746,
    CyanBanner {
        rotation: I0_15,
    } = 10762,
    PurpleBanner {
        rotation: I0_15,
    } = 10778,
    BlueBanner {
        rotation: I0_15,
    } = 10794,
    BrownBanner {
        rotation: I0_15,
    } = 10810,
    GreenBanner {
        rotation: I0_15,
    } = 10826,
    RedBanner {
        rotation: I0_15,
    } = 10842,
    BlackBanner {
        rotation: I0_15,
    } = 10858,
    WhiteWallBanner {
        facing: HorizontalFacing,
    } = 10874,
    OrangeWallBanner {
        facing: HorizontalFacing,
    } = 10878,
    MagentaWallBanner {
        facing: HorizontalFacing,
    } = 10882,
    LightBlueWallBanner {
        facing: HorizontalFacing,
    } = 10886,
    YellowWallBanner {
        facing: HorizontalFacing,
    } = 10890,
    LimeWallBanner {
        facing: HorizontalFacing,
    } = 10894,
    PinkWallBanner {
        facing: HorizontalFacing,
    } = 10898,
    GrayWallBanner {
        facing: HorizontalFacing,
    } = 10902,
    LightGrayWallBanner {
        facing: HorizontalFacing,
    } = 10906,
    CyanWallBanner {
        facing: HorizontalFacing,
    } = 10910,
    PurpleWallBanner {
        facing: HorizontalFacing,
    } = 10914,
    BlueWallBanner {
        facing: HorizontalFacing,
    } = 10918,
    BrownWallBanner {
        facing: HorizontalFacing,
    } = 10922,
    GreenWallBanner {
        facing: HorizontalFacing,
    } = 10926,
    RedWallBanner {
        facing: HorizontalFacing,
    } = 10930,
    BlackWallBanner {
        facing: HorizontalFacing,
    } = 10934,
    RedSandstone = 10938,
    ChiseledRedSandstone = 10939,
    CutRedSandstone = 10940,
    RedSandstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 10941,
    OakSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11021,
    SpruceSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11027,
    BirchSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11033,
    JungleSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11039,
    AcaciaSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11045,
    CherrySlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11051,
    DarkOakSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11057,
    MangroveSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11063,
    BambooSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11069,
    BambooMosaicSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11075,
    StoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11081,
    SmoothStoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11087,
    SandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11093,
    CutSandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11099,
    PetrifiedOakSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11105,
    CobblestoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11111,
    BrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11117,
    StoneBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11123,
    MudBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11129,
    NetherBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11135,
    QuartzSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11141,
    RedSandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11147,
    CutRedSandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11153,
    PurpurSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 11159,
    SmoothStone = 11165,
    SmoothSandstone = 11166,
    SmoothQuartz = 11167,
    SmoothRedSandstone = 11168,
    SpruceFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11169,
    BirchFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11201,
    JungleFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11233,
    AcaciaFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11265,
    CherryFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11297,
    DarkOakFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11329,
    MangroveFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11361,
    BambooFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 11393,
    SpruceFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11425,
    BirchFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11457,
    JungleFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11489,
    AcaciaFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11521,
    CherryFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11553,
    DarkOakFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11585,
    MangroveFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11617,
    BambooFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 11649,
    SpruceDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 11681,
    BirchDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 11745,
    JungleDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 11809,
    AcaciaDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 11873,
    CherryDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 11937,
    DarkOakDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 12001,
    MangroveDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 12065,
    BambooDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 12129,
    EndRod {
        facing: Facing,
    } = 12193,
    ChorusPlant {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
    } = 12199,
    ChorusFlower {
        age: I0_5,
    } = 12263,
    PurpurBlock = 12269,
    PurpurPillar {
        axis: Axis,
    } = 12270,
    PurpurStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 12273,
    EndStoneBricks = 12353,
    TorchflowerCrop {
        age: I0_1,
    } = 12354,
    PitcherCrop = 12356,
    PitcherPlant {
        half: Half,
    } = 12366,
    Beetroots {
        age: I0_3,
    } = 12368,
    DirtPath = 12372,
    EndGateway = 12373,
    RepeatingCommandBlock {
        facing: Facing,
        conditional: bool,
    } = 12374,
    ChainCommandBlock {
        facing: Facing,
        conditional: bool,
    } = 12386,
    FrostedIce {
        age: I0_3,
    } = 12398,
    MagmaBlock = 12402,
    NetherWartBlock = 12403,
    RedNetherBricks = 12404,
    BoneBlock {
        axis: Axis,
    } = 12405,
    StructureVoid = 12408,
    Observer {
        facing: Facing,
        powered: bool,
    } = 12409,
    ShulkerBox {
        facing: Facing,
    } = 12421,
    WhiteShulkerBox {
        facing: Facing,
    } = 12427,
    OrangeShulkerBox {
        facing: Facing,
    } = 12433,
    MagentaShulkerBox {
        facing: Facing,
    } = 12439,
    LightBlueShulkerBox {
        facing: Facing,
    } = 12445,
    YellowShulkerBox {
        facing: Facing,
    } = 12451,
    LimeShulkerBox {
        facing: Facing,
    } = 12457,
    PinkShulkerBox {
        facing: Facing,
    } = 12463,
    GrayShulkerBox {
        facing: Facing,
    } = 12469,
    LightGrayShulkerBox {
        facing: Facing,
    } = 12475,
    CyanShulkerBox {
        facing: Facing,
    } = 12481,
    PurpleShulkerBox {
        facing: Facing,
    } = 12487,
    BlueShulkerBox {
        facing: Facing,
    } = 12493,
    BrownShulkerBox {
        facing: Facing,
    } = 12499,
    GreenShulkerBox {
        facing: Facing,
    } = 12505,
    RedShulkerBox {
        facing: Facing,
    } = 12511,
    BlackShulkerBox {
        facing: Facing,
    } = 12517,
    WhiteGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12523,
    OrangeGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12527,
    MagentaGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12531,
    LightBlueGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12535,
    YellowGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12539,
    LimeGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12543,
    PinkGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12547,
    GrayGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12551,
    LightGrayGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12555,
    CyanGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12559,
    PurpleGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12563,
    BlueGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12567,
    BrownGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12571,
    GreenGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12575,
    RedGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12579,
    BlackGlazedTerracotta {
        facing: HorizontalFacing,
    } = 12583,
    WhiteConcrete = 12587,
    OrangeConcrete = 12588,
    MagentaConcrete = 12589,
    LightBlueConcrete = 12590,
    YellowConcrete = 12591,
    LimeConcrete = 12592,
    PinkConcrete = 12593,
    GrayConcrete = 12594,
    LightGrayConcrete = 12595,
    CyanConcrete = 12596,
    PurpleConcrete = 12597,
    BlueConcrete = 12598,
    BrownConcrete = 12599,
    GreenConcrete = 12600,
    RedConcrete = 12601,
    BlackConcrete = 12602,
    WhiteConcretePowder = 12603,
    OrangeConcretePowder = 12604,
    MagentaConcretePowder = 12605,
    LightBlueConcretePowder = 12606,
    YellowConcretePowder = 12607,
    LimeConcretePowder = 12608,
    PinkConcretePowder = 12609,
    GrayConcretePowder = 12610,
    LightGrayConcretePowder = 12611,
    CyanConcretePowder = 12612,
    PurpleConcretePowder = 12613,
    BlueConcretePowder = 12614,
    BrownConcretePowder = 12615,
    GreenConcretePowder = 12616,
    RedConcretePowder = 12617,
    BlackConcretePowder = 12618,
    Kelp {
        age: I0_25,
    } = 12619,
    KelpPlant = 12645,
    DriedKelpBlock = 12646,
    TurtleEgg {
        hatch: I0_2,
        eggs: I1_4,
    } = 12647,
    SnifferEgg {
        hatch: I0_2,
    } = 12659,
    DeadTubeCoralBlock = 12662,
    DeadBrainCoralBlock = 12663,
    DeadBubbleCoralBlock = 12664,
    DeadFireCoralBlock = 12665,
    DeadHornCoralBlock = 12666,
    TubeCoralBlock = 12667,
    BrainCoralBlock = 12668,
    BubbleCoralBlock = 12669,
    FireCoralBlock = 12670,
    HornCoralBlock = 12671,
    DeadTubeCoral {
        waterlogged: bool,
    } = 12672,
    DeadBrainCoral {
        waterlogged: bool,
    } = 12674,
    DeadBubbleCoral {
        waterlogged: bool,
    } = 12676,
    DeadFireCoral {
        waterlogged: bool,
    } = 12678,
    DeadHornCoral {
        waterlogged: bool,
    } = 12680,
    TubeCoral {
        waterlogged: bool,
    } = 12682,
    BrainCoral {
        waterlogged: bool,
    } = 12684,
    BubbleCoral {
        waterlogged: bool,
    } = 12686,
    FireCoral {
        waterlogged: bool,
    } = 12688,
    HornCoral {
        waterlogged: bool,
    } = 12690,
    DeadTubeCoralFan {
        waterlogged: bool,
    } = 12692,
    DeadBrainCoralFan {
        waterlogged: bool,
    } = 12694,
    DeadBubbleCoralFan {
        waterlogged: bool,
    } = 12696,
    DeadFireCoralFan {
        waterlogged: bool,
    } = 12698,
    DeadHornCoralFan {
        waterlogged: bool,
    } = 12700,
    TubeCoralFan {
        waterlogged: bool,
    } = 12702,
    BrainCoralFan {
        waterlogged: bool,
    } = 12704,
    BubbleCoralFan {
        waterlogged: bool,
    } = 12706,
    FireCoralFan {
        waterlogged: bool,
    } = 12708,
    HornCoralFan {
        waterlogged: bool,
    } = 12710,
    DeadTubeCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12712,
    DeadBrainCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12720,
    DeadBubbleCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12728,
    DeadFireCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12736,
    DeadHornCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12744,
    TubeCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12752,
    BrainCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12760,
    BubbleCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12768,
    FireCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12776,
    HornCoralWallFan {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 12784,
    SeaPickle {
        pickles: I1_4,
        waterlogged: bool,
    } = 12792,
    BlueIce = 12800,
    Conduit {
        waterlogged: bool,
    } = 12801,
    BambooSapling = 12803,
    Bamboo {
        age: I0_1,
        leaves: BambooLeaves,
        stage: I0_1,
    } = 12804,
    PottedBamboo = 12816,
    VoidAir = 12817,
    CaveAir = 12818,
    BubbleColumn {
        drag: bool,
    } = 12819,
    PolishedGraniteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 12821,
    SmoothRedSandstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 12901,
    MossyStoneBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 12981,
    PolishedDioriteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13061,
    MossyCobblestoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13141,
    EndStoneBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13221,
    StoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13301,
    SmoothSandstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13381,
    SmoothQuartzStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13461,
    GraniteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13541,
    AndesiteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13621,
    RedNetherBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13701,
    PolishedAndesiteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13781,
    DioriteStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 13861,
    PolishedGraniteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13941,
    SmoothRedSandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13947,
    MossyStoneBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13953,
    PolishedDioriteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13959,
    MossyCobblestoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13965,
    EndStoneBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13971,
    SmoothSandstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13977,
    SmoothQuartzSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13983,
    GraniteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13989,
    AndesiteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 13995,
    RedNetherBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 14001,
    PolishedAndesiteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 14007,
    DioriteSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 14013,
    BrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 14019,
    PrismarineWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 14343,
    RedSandstoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 14667,
    MossyStoneBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 14991,
    GraniteWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 15315,
    StoneBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 15639,
    MudBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 15963,
    NetherBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 16287,
    AndesiteWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 16611,
    RedNetherBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 16935,
    SandstoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 17259,
    EndStoneBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 17583,
    DioriteWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 17907,
    Scaffolding {
        distance: I0_7,
        waterlogged: bool,
        bottom: bool,
    } = 18231,
    Loom {
        facing: HorizontalFacing,
    } = 18263,
    Barrel {
        facing: Facing,
        open: bool,
    } = 18267,
    Smoker {
        facing: HorizontalFacing,
        lit: bool,
    } = 18279,
    BlastFurnace {
        facing: HorizontalFacing,
        lit: bool,
    } = 18287,
    CartographyTable = 18295,
    FletchingTable = 18296,
    Grindstone {
        facing: HorizontalFacing,
        face: Face,
    } = 18297,
    Lectern {
        facing: HorizontalFacing,
        powered: bool,
        has_book: bool,
    } = 18309,
    SmithingTable = 18325,
    Stonecutter {
        facing: HorizontalFacing,
    } = 18326,
    Bell {
        facing: HorizontalFacing,
        attachment: BellAttachment,
        powered: bool,
    } = 18330,
    Lantern {
        hanging: bool,
        waterlogged: bool,
    } = 18362,
    SoulLantern {
        hanging: bool,
        waterlogged: bool,
    } = 18366,
    Campfire {
        facing: HorizontalFacing,
        lit: bool,
        signal_fire: bool,
        waterlogged: bool,
    } = 18370,
    SoulCampfire {
        facing: HorizontalFacing,
        lit: bool,
        signal_fire: bool,
        waterlogged: bool,
    } = 18402,
    SweetBerryBush {
        age: I0_3,
    } = 18434,
    WarpedStem {
        axis: Axis,
    } = 18438,
    StrippedWarpedStem {
        axis: Axis,
    } = 18441,
    WarpedHyphae {
        axis: Axis,
    } = 18444,
    StrippedWarpedHyphae {
        axis: Axis,
    } = 18447,
    WarpedNylium = 18450,
    WarpedFungus = 18451,
    WarpedWartBlock = 18452,
    WarpedRoots = 18453,
    NetherSprouts = 18454,
    CrimsonStem {
        axis: Axis,
    } = 18455,
    StrippedCrimsonStem {
        axis: Axis,
    } = 18458,
    CrimsonHyphae {
        axis: Axis,
    } = 18461,
    StrippedCrimsonHyphae {
        axis: Axis,
    } = 18464,
    CrimsonNylium = 18467,
    CrimsonFungus = 18468,
    Shroomlight = 18469,
    WeepingVines {
        age: I0_25,
    } = 18470,
    WeepingVinesPlant = 18496,
    TwistingVines {
        age: I0_25,
    } = 18497,
    TwistingVinesPlant = 18523,
    CrimsonRoots = 18524,
    CrimsonPlanks = 18525,
    WarpedPlanks = 18526,
    CrimsonSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 18527,
    WarpedSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 18533,
    CrimsonPressurePlate {
        powered: bool,
    } = 18539,
    WarpedPressurePlate {
        powered: bool,
    } = 18541,
    CrimsonFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 18543,
    WarpedFence {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        waterlogged: bool,
    } = 18575,
    CrimsonTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 18607,
    WarpedTrapdoor {
        facing: HorizontalFacing,
        open: bool,
        half: Half,
        powered: bool,
        waterlogged: bool,
    } = 18671,
    CrimsonFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 18735,
    WarpedFenceGate {
        facing: HorizontalFacing,
        open: bool,
        powered: bool,
        in_wall: bool,
    } = 18767,
    CrimsonStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 18799,
    WarpedStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 18879,
    CrimsonButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 18959,
    WarpedButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 18983,
    CrimsonDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 19007,
    WarpedDoor {
        half: Half,
        facing: HorizontalFacing,
        open: bool,
        hinge: DoorHinge,
        powered: bool,
    } = 19071,
    CrimsonSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 19135,
    WarpedSign {
        rotation: I0_15,
        waterlogged: bool,
    } = 19167,
    CrimsonWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 19199,
    WarpedWallSign {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 19207,
    StructureBlock {
        mode: StructureBlockMode,
    } = 19215,
    Jigsaw {
        orientation: JigsawOrientation,
    } = 19219,
    Composter {
        level: I0_8,
    } = 19231,
    Target {
        power: I0_15,
    } = 19240,
    BeeNest {
        honey_level: I0_5,
        facing: HorizontalFacing,
    } = 19256,
    Beehive {
        honey_level: I0_5,
        facing: HorizontalFacing,
    } = 19280,
    HoneyBlock = 19304,
    HoneycombBlock = 19305,
    NetheriteBlock = 19306,
    AncientDebris = 19307,
    CryingObsidian = 19308,
    RespawnAnchor {
        charges: I0_4,
    } = 19309,
    PottedCrimsonFungus = 19314,
    PottedWarpedFungus = 19315,
    PottedCrimsonRoots = 19316,
    PottedWarpedRoots = 19317,
    Lodestone = 19318,
    Blackstone = 19319,
    BlackstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 19320,
    BlackstoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 19400,
    BlackstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 19724,
    PolishedBlackstone = 19730,
    PolishedBlackstoneBricks = 19731,
    CrackedPolishedBlackstoneBricks = 19732,
    ChiseledPolishedBlackstone = 19733,
    PolishedBlackstoneBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 19734,
    PolishedBlackstoneBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 19740,
    PolishedBlackstoneBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 19820,
    GildedBlackstone = 20144,
    PolishedBlackstoneStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 20145,
    PolishedBlackstoneSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 20225,
    PolishedBlackstonePressurePlate {
        powered: bool,
    } = 20231,
    PolishedBlackstoneButton {
        facing: HorizontalFacing,
        powered: bool,
        face: Face,
    } = 20233,
    PolishedBlackstoneWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 20257,
    ChiseledNetherBricks = 20581,
    CrackedNetherBricks = 20582,
    QuartzBricks = 20583,
    Candle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20584,
    WhiteCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20600,
    OrangeCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20616,
    MagentaCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20632,
    LightBlueCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20648,
    YellowCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20664,
    LimeCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20680,
    PinkCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20696,
    GrayCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20712,
    LightGrayCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20728,
    CyanCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20744,
    PurpleCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20760,
    BlueCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20776,
    BrownCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20792,
    GreenCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20808,
    RedCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20824,
    BlackCandle {
        candles: I1_4,
        lit: bool,
        waterlogged: bool,
    } = 20840,
    CandleCake {
        lit: bool,
    } = 20856,
    WhiteCandleCake {
        lit: bool,
    } = 20858,
    OrangeCandleCake {
        lit: bool,
    } = 20860,
    MagentaCandleCake {
        lit: bool,
    } = 20862,
    LightBlueCandleCake {
        lit: bool,
    } = 20864,
    YellowCandleCake {
        lit: bool,
    } = 20866,
    LimeCandleCake {
        lit: bool,
    } = 20868,
    PinkCandleCake {
        lit: bool,
    } = 20870,
    GrayCandleCake {
        lit: bool,
    } = 20872,
    LightGrayCandleCake {
        lit: bool,
    } = 20874,
    CyanCandleCake {
        lit: bool,
    } = 20876,
    PurpleCandleCake {
        lit: bool,
    } = 20878,
    BlueCandleCake {
        lit: bool,
    } = 20880,
    BrownCandleCake {
        lit: bool,
    } = 20882,
    GreenCandleCake {
        lit: bool,
    } = 20884,
    RedCandleCake {
        lit: bool,
    } = 20886,
    BlackCandleCake {
        lit: bool,
    } = 20888,
    AmethystBlock = 20890,
    BuddingAmethyst = 20891,
    AmethystCluster {
        facing: Facing,
        waterlogged: bool,
    } = 20892,
    LargeAmethystBud {
        facing: Facing,
        waterlogged: bool,
    } = 20904,
    MediumAmethystBud {
        facing: Facing,
        waterlogged: bool,
    } = 20916,
    SmallAmethystBud {
        facing: Facing,
        waterlogged: bool,
    } = 20928,
    Tuff = 20940,
    Calcite = 20941,
    TintedGlass = 20942,
    PowderSnow = 20943,
    SculkSensor {
        sculk_sensor_phase: SculkSensorPhase,
        power: I0_15,
        waterlogged: bool,
    } = 20944,
    CalibratedSculkSensor {
        sculk_sensor_phase: SculkSensorPhase,
        power: I0_15,
        waterlogged: bool,
        facing: HorizontalFacing,
    } = 21040,
    Sculk = 21424,
    SculkVein {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
        up: bool,
        down: bool,
        waterlogged: bool,
    } = 21425,
    SculkCatalyst {
        bloom: bool,
    } = 21553,
    SculkShrieker {
        shrieking: bool,
        waterlogged: bool,
        can_summon: bool,
    } = 21555,
    OxidizedCopper = 21563,
    WeatheredCopper = 21564,
    ExposedCopper = 21565,
    CopperBlock = 21566,
    CopperOre = 21567,
    DeepslateCopperOre = 21568,
    OxidizedCutCopper = 21569,
    WeatheredCutCopper = 21570,
    ExposedCutCopper = 21571,
    CutCopper = 21572,
    OxidizedCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 21573,
    WeatheredCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 21653,
    ExposedCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 21733,
    CutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 21813,
    OxidizedCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 21893,
    WeatheredCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 21899,
    ExposedCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 21905,
    CutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 21911,
    WaxedCopperBlock = 21917,
    WaxedWeatheredCopper = 21918,
    WaxedExposedCopper = 21919,
    WaxedOxidizedCopper = 21920,
    WaxedOxidizedCutCopper = 21921,
    WaxedWeatheredCutCopper = 21922,
    WaxedExposedCutCopper = 21923,
    WaxedCutCopper = 21924,
    WaxedOxidizedCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 21925,
    WaxedWeatheredCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 22005,
    WaxedExposedCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 22085,
    WaxedCutCopperStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 22165,
    WaxedOxidizedCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22245,
    WaxedWeatheredCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22251,
    WaxedExposedCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22257,
    WaxedCutCopperSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22263,
    LightningRod {
        facing: Facing,
        powered: bool,
        waterlogged: bool,
    } = 22269,
    PointedDripstone {
        vertical_direction: PointedDripstoneDirection,
        thickness: PointedDripstoneThickness,
        waterlogged: bool,
    } = 22293,
    DripstoneBlock = 22313,
    CaveVines {
        age: I0_25,
        berries: bool,
    } = 22314,
    CaveVinesPlant {
        berries: bool,
    } = 22366,
    SporeBlossom = 22368,
    Azalea = 22369,
    FloweringAzalea = 22370,
    MossCarpet = 22371,
    PinkPetals {
        facing: HorizontalFacing,
        flower_amount: I1_4,
    } = 22372,
    MossBlock = 22388,
    BigDripleaf {
        facing: HorizontalFacing,
        tilt: DripleafTilt,
        waterlogged: bool,
    } = 22389,
    BigDripleafStem {
        facing: HorizontalFacing,
        waterlogged: bool,
    } = 22421,
    SmallDripleaf {
        facing: HorizontalFacing,
        half: Half,
        waterlogged: bool,
    } = 22429,
    HangingRoots {
        waterlogged: bool,
    } = 22445,
    RootedDirt = 22447,
    Mud = 22448,
    Deepslate {
        axis: Axis,
    } = 22449,
    CobbledDeepslate = 22452,
    CobbledDeepslateStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 22453,
    CobbledDeepslateSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22533,
    CobbledDeepslateWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 22539,
    PolishedDeepslate = 22863,
    PolishedDeepslateStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 22864,
    PolishedDeepslateSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 22944,
    PolishedDeepslateWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 22950,
    DeepslateTiles = 23274,
    DeepslateTileStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 23275,
    DeepslateTileSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 23355,
    DeepslateTileWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 23361,
    DeepslateBricks = 23685,
    DeepslateBrickStairs {
        facing: HorizontalFacing,
        half: Half,
        shape: StairsShape,
        waterlogged: bool,
    } = 23686,
    DeepslateBrickSlab {
        r#type: SlabType,
        waterlogged: bool,
    } = 23766,
    DeepslateBrickWall {
        north: WallConnection,
        east: WallConnection,
        south: WallConnection,
        west: WallConnection,
        up: bool,
        waterlogged: bool,
    } = 23772,
    ChiseledDeepslate = 24096,
    CrackedDeepslateBricks = 24097,
    CrackedDeepslateTiles = 24098,
    InfestedDeepslate {
        axis: Axis,
    } = 24099,
    SmoothBasalt = 24102,
    RawIronBlock = 24103,
    RawCopperBlock = 24104,
    RawGoldBlock = 24105,
    PottedAzaleaBush = 24106,
    PottedFloweringAzaleaBush = 24107,
    OchreFroglight {
        axis: Axis,
    } = 24108,
    VerdantFroglight {
        axis: Axis,
    } = 24111,
    PearlescentFroglight {
        axis: Axis,
    } = 24114,
    Frogspawn = 24117,
    ReinforcedDeepslate = 24118,
    DecoratedPot {
        facing: HorizontalFacing,
        cracked: bool,
        waterlogged: bool,
    } = 24119,
}
impl Block {
    pub fn get_state_id(&self) -> u16 {
        use Block::*;
        match self {
            Air => 0u16,
            Stone => 1u16,
            Granite => 2u16,
            PolishedGranite => 3u16,
            Diorite => 4u16,
            PolishedDiorite => 5u16,
            Andesite => 6u16,
            PolishedAndesite => 7u16,
            GrassBlock { snowy } => 8u16 + !snowy as u16,
            Dirt => 10u16,
            CoarseDirt => 11u16,
            Podzol { snowy } => 12u16 + !snowy as u16,
            Cobblestone => 14u16,
            OakPlanks => 15u16,
            SprucePlanks => 16u16,
            BirchPlanks => 17u16,
            JunglePlanks => 18u16,
            AcaciaPlanks => 19u16,
            CherryPlanks => 20u16,
            DarkOakPlanks => 21u16,
            MangrovePlanks => 22u16,
            BambooPlanks => 23u16,
            BambooMosaic => 24u16,
            OakSapling { stage } => 25u16 + u16::from(*stage),
            SpruceSapling { stage } => 27u16 + u16::from(*stage),
            BirchSapling { stage } => 29u16 + u16::from(*stage),
            JungleSapling { stage } => 31u16 + u16::from(*stage),
            AcaciaSapling { stage } => 33u16 + u16::from(*stage),
            CherrySapling { stage } => 35u16 + u16::from(*stage),
            DarkOakSapling { stage } => 37u16 + u16::from(*stage),
            MangrovePropagule {
                stage,
                age,
                waterlogged,
                hanging,
            } => {
                39u16
                    + u16::from(*age) * 8
                    + !hanging as u16 * 4
                    + u16::from(*stage) * 2
                    + !waterlogged as u16
            }
            Bedrock => 79u16,
            Water { level } => 80u16 + u16::from(*level),
            Lava { level } => 96u16 + u16::from(*level),
            Sand => 112u16,
            SuspiciousSand { dusted } => 113u16 + u16::from(*dusted),
            RedSand => 117u16,
            Gravel => 118u16,
            SuspiciousGravel { dusted } => 119u16 + u16::from(*dusted),
            GoldOre => 123u16,
            DeepslateGoldOre => 124u16,
            IronOre => 125u16,
            DeepslateIronOre => 126u16,
            CoalOre => 127u16,
            DeepslateCoalOre => 128u16,
            NetherGoldOre => 129u16,
            OakLog { axis } => 130u16 + *axis as u16,
            SpruceLog { axis } => 133u16 + *axis as u16,
            BirchLog { axis } => 136u16 + *axis as u16,
            JungleLog { axis } => 139u16 + *axis as u16,
            AcaciaLog { axis } => 142u16 + *axis as u16,
            CherryLog { axis } => 145u16 + *axis as u16,
            DarkOakLog { axis } => 148u16 + *axis as u16,
            MangroveLog { axis } => 151u16 + *axis as u16,
            MangroveRoots { waterlogged } => 154u16 + !waterlogged as u16,
            MuddyMangroveRoots { axis } => 156u16 + *axis as u16,
            BambooBlock { axis } => 159u16 + *axis as u16,
            StrippedSpruceLog { axis } => 162u16 + *axis as u16,
            StrippedBirchLog { axis } => 165u16 + *axis as u16,
            StrippedJungleLog { axis } => 168u16 + *axis as u16,
            StrippedAcaciaLog { axis } => 171u16 + *axis as u16,
            StrippedCherryLog { axis } => 174u16 + *axis as u16,
            StrippedDarkOakLog { axis } => 177u16 + *axis as u16,
            StrippedOakLog { axis } => 180u16 + *axis as u16,
            StrippedMangroveLog { axis } => 183u16 + *axis as u16,
            StrippedBambooBlock { axis } => 186u16 + *axis as u16,
            OakWood { axis } => 189u16 + *axis as u16,
            SpruceWood { axis } => 192u16 + *axis as u16,
            BirchWood { axis } => 195u16 + *axis as u16,
            JungleWood { axis } => 198u16 + *axis as u16,
            AcaciaWood { axis } => 201u16 + *axis as u16,
            CherryWood { axis } => 204u16 + *axis as u16,
            DarkOakWood { axis } => 207u16 + *axis as u16,
            MangroveWood { axis } => 210u16 + *axis as u16,
            StrippedOakWood { axis } => 213u16 + *axis as u16,
            StrippedSpruceWood { axis } => 216u16 + *axis as u16,
            StrippedBirchWood { axis } => 219u16 + *axis as u16,
            StrippedJungleWood { axis } => 222u16 + *axis as u16,
            StrippedAcaciaWood { axis } => 225u16 + *axis as u16,
            StrippedCherryWood { axis } => 228u16 + *axis as u16,
            StrippedDarkOakWood { axis } => 231u16 + *axis as u16,
            StrippedMangroveWood { axis } => 234u16 + *axis as u16,
            OakLeaves {
                distance,
                persistent,
                waterlogged,
            } => 237u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            SpruceLeaves {
                distance,
                persistent,
                waterlogged,
            } => 265u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            BirchLeaves {
                distance,
                persistent,
                waterlogged,
            } => 293u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            JungleLeaves {
                distance,
                persistent,
                waterlogged,
            } => 321u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            AcaciaLeaves {
                distance,
                persistent,
                waterlogged,
            } => 349u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            CherryLeaves {
                distance,
                persistent,
                waterlogged,
            } => 377u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            DarkOakLeaves {
                distance,
                persistent,
                waterlogged,
            } => 405u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            MangroveLeaves {
                distance,
                persistent,
                waterlogged,
            } => 433u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            AzaleaLeaves {
                distance,
                persistent,
                waterlogged,
            } => 461u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            FloweringAzaleaLeaves {
                distance,
                persistent,
                waterlogged,
            } => 489u16 + u16::from(*distance) * 4 + !persistent as u16 * 2 + !waterlogged as u16,
            Sponge => 517u16,
            WetSponge => 518u16,
            Glass => 519u16,
            LapisOre => 520u16,
            DeepslateLapisOre => 521u16,
            LapisBlock => 522u16,
            Dispenser { facing, triggered } => 523u16 + *facing as u16 * 2 + !triggered as u16,
            Sandstone => 535u16,
            ChiseledSandstone => 536u16,
            CutSandstone => 537u16,
            NoteBlock {
                instrument,
                powered,
                note,
            } => 538u16 + *instrument as u16 * 50 + u16::from(*note) * 25 + !powered as u16,
            WhiteBed {
                facing,
                part,
                occupied,
            } => 1688u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            OrangeBed {
                facing,
                part,
                occupied,
            } => 1704u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            MagentaBed {
                facing,
                part,
                occupied,
            } => 1720u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            LightBlueBed {
                facing,
                part,
                occupied,
            } => 1736u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            YellowBed {
                facing,
                part,
                occupied,
            } => 1752u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            LimeBed {
                facing,
                part,
                occupied,
            } => 1768u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            PinkBed {
                facing,
                part,
                occupied,
            } => 1784u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            GrayBed {
                facing,
                part,
                occupied,
            } => 1800u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            LightGrayBed {
                facing,
                part,
                occupied,
            } => 1816u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            CyanBed {
                facing,
                part,
                occupied,
            } => 1832u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            PurpleBed {
                facing,
                part,
                occupied,
            } => 1848u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            BlueBed {
                facing,
                part,
                occupied,
            } => 1864u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            BrownBed {
                facing,
                part,
                occupied,
            } => 1880u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            GreenBed {
                facing,
                part,
                occupied,
            } => 1896u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            RedBed {
                facing,
                part,
                occupied,
            } => 1912u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            BlackBed {
                facing,
                part,
                occupied,
            } => 1928u16 + *facing as u16 * 4 + !occupied as u16 * 2 + *part as u16,
            PoweredRail {
                shape,
                powered,
                waterlogged,
            } => 1944u16 + !powered as u16 * 12 + *shape as u16 * 2 + *waterlogged as u16,
            DetectorRail {
                shape,
                powered,
                waterlogged,
            } => 1968u16 + !powered as u16 * 12 + *shape as u16 * 2 + *waterlogged as u16,
            StickyPiston { facing, extended } => 1992u16 + !extended as u16 * 6 + *facing as u16,
            Cobweb => 2004u16,
            Grass => 2005u16,
            Fern => 2006u16,
            DeadBush => 2007u16,
            Seagrass => 2008u16,
            TallSeagrass { half } => 2009u16 + *half as u16,
            Piston { facing, extended } => 2011u16 + !extended as u16 * 6 + *facing as u16,
            PistonHead {
                facing,
                r#type,
                short,
            } => 2023u16 + *facing as u16 * 4 + !short as u16 * 2 + *r#type as u16,
            WhiteWool => 2047u16,
            OrangeWool => 2048u16,
            MagentaWool => 2049u16,
            LightBlueWool => 2050u16,
            YellowWool => 2051u16,
            LimeWool => 2052u16,
            PinkWool => 2053u16,
            GrayWool => 2054u16,
            LightGrayWool => 2055u16,
            CyanWool => 2056u16,
            PurpleWool => 2057u16,
            BlueWool => 2058u16,
            BrownWool => 2059u16,
            GreenWool => 2060u16,
            RedWool => 2061u16,
            BlackWool => 2062u16,
            MovingPiston { facing, r#type } => 2063u16 + *facing as u16 * 2 + *r#type as u16,
            Dandelion => 2075u16,
            Torchflower => 2076u16,
            Poppy => 2077u16,
            BlueOrchid => 2078u16,
            Allium => 2079u16,
            AzureBluet => 2080u16,
            RedTulip => 2081u16,
            OrangeTulip => 2082u16,
            WhiteTulip => 2083u16,
            PinkTulip => 2084u16,
            OxeyeDaisy => 2085u16,
            Cornflower => 2086u16,
            WitherRose => 2087u16,
            LilyOfTheValley => 2088u16,
            BrownMushroom => 2089u16,
            RedMushroom => 2090u16,
            GoldBlock => 2091u16,
            IronBlock => 2092u16,
            Bricks => 2093u16,
            Tnt { unstable } => 2094u16 + !unstable as u16,
            Bookshelf => 2096u16,
            ChiseledBookshelf {
                facing,
                slot_0_occupied,
                slot_1_occupied,
                slot_2_occupied,
                slot_3_occupied,
                slot_4_occupied,
                slot_5_occupied,
            } => {
                2097u16
                    + *facing as u16 * 64
                    + !slot_0_occupied as u16 * 32
                    + !slot_1_occupied as u16 * 16
                    + !slot_2_occupied as u16 * 8
                    + !slot_3_occupied as u16 * 4
                    + !slot_4_occupied as u16 * 2
                    + !slot_5_occupied as u16
            }
            MossyCobblestone => 2353u16,
            Obsidian => 2354u16,
            Torch => 2355u16,
            WallTorch { facing } => 2356u16 + *facing as u16,
            Fire {
                age,
                north,
                east,
                south,
                west,
                up,
            } => {
                2360u16
                    + u16::from(*age) * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            SoulFire => 2872u16,
            Spawner => 2873u16,
            OakStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                2874u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            Chest {
                facing,
                r#type,
                waterlogged,
            } => 2954u16 + *facing as u16 * 6 + *r#type as u16 * 2 + !waterlogged as u16,
            RedstoneWire {
                north,
                east,
                south,
                west,
                power,
            } => {
                2978u16
                    + *east as u16 * 432
                    + *north as u16 * 144
                    + u16::from(*power) * 9
                    + *south as u16 * 3
                    + *west as u16
            }
            DiamondOre => 4274u16,
            DeepslateDiamondOre => 4275u16,
            DiamondBlock => 4276u16,
            CraftingTable => 4277u16,
            Wheat { age } => 4278u16 + u16::from(*age),
            Farmland { moisture } => 4286u16 + u16::from(*moisture),
            Furnace { facing, lit } => 4294u16 + *facing as u16 * 2 + !lit as u16,
            OakSign {
                rotation,
                waterlogged,
            } => 4302u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            SpruceSign {
                rotation,
                waterlogged,
            } => 4334u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            BirchSign {
                rotation,
                waterlogged,
            } => 4366u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            AcaciaSign {
                rotation,
                waterlogged,
            } => 4398u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            CherrySign {
                rotation,
                waterlogged,
            } => 4430u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            JungleSign {
                rotation,
                waterlogged,
            } => 4462u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            DarkOakSign {
                rotation,
                waterlogged,
            } => 4494u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            MangroveSign {
                rotation,
                waterlogged,
            } => 4526u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            BambooSign {
                rotation,
                waterlogged,
            } => 4558u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            OakDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                4590u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            Ladder {
                facing,
                waterlogged,
            } => 4654u16 + *facing as u16 * 2 + !waterlogged as u16,
            Rail { shape, waterlogged } => 4662u16 + *shape as u16 * 2 + !waterlogged as u16,
            CobblestoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                4682u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            OakWallSign {
                facing,
                waterlogged,
            } => 4762u16 + *facing as u16 * 2 + !waterlogged as u16,
            SpruceWallSign {
                facing,
                waterlogged,
            } => 4770u16 + *facing as u16 * 2 + !waterlogged as u16,
            BirchWallSign {
                facing,
                waterlogged,
            } => 4778u16 + *facing as u16 * 2 + !waterlogged as u16,
            AcaciaWallSign {
                facing,
                waterlogged,
            } => 4786u16 + *facing as u16 * 2 + !waterlogged as u16,
            CherryWallSign {
                facing,
                waterlogged,
            } => 4794u16 + *facing as u16 * 2 + !waterlogged as u16,
            JungleWallSign {
                facing,
                waterlogged,
            } => 4802u16 + *facing as u16 * 2 + !waterlogged as u16,
            DarkOakWallSign {
                facing,
                waterlogged,
            } => 4810u16 + *facing as u16 * 2 + !waterlogged as u16,
            MangroveWallSign {
                facing,
                waterlogged,
            } => 4818u16 + *facing as u16 * 2 + !waterlogged as u16,
            BambooWallSign {
                facing,
                waterlogged,
            } => 4826u16 + *facing as u16 * 2 + !waterlogged as u16,
            OakHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 4834u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            SpruceHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 4898u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            BirchHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 4962u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            AcaciaHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5026u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            CherryHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5090u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            JungleHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5154u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            DarkOakHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5218u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            CrimsonHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5282u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            WarpedHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5346u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            MangroveHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5410u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            BambooHangingSign {
                rotation,
                attached,
                waterlogged,
            } => 5474u16 + !attached as u16 * 32 + u16::from(*rotation) * 2 + !waterlogged as u16,
            OakWallHangingSign {
                facing,
                waterlogged,
            } => 5538u16 + *facing as u16 * 2 + !waterlogged as u16,
            SpruceWallHangingSign {
                facing,
                waterlogged,
            } => 5546u16 + *facing as u16 * 2 + !waterlogged as u16,
            BirchWallHangingSign {
                facing,
                waterlogged,
            } => 5554u16 + *facing as u16 * 2 + !waterlogged as u16,
            AcaciaWallHangingSign {
                facing,
                waterlogged,
            } => 5562u16 + *facing as u16 * 2 + !waterlogged as u16,
            CherryWallHangingSign {
                facing,
                waterlogged,
            } => 5570u16 + *facing as u16 * 2 + !waterlogged as u16,
            JungleWallHangingSign {
                facing,
                waterlogged,
            } => 5578u16 + *facing as u16 * 2 + !waterlogged as u16,
            DarkOakWallHangingSign {
                facing,
                waterlogged,
            } => 5586u16 + *facing as u16 * 2 + !waterlogged as u16,
            MangroveWallHangingSign {
                facing,
                waterlogged,
            } => 5594u16 + *facing as u16 * 2 + !waterlogged as u16,
            CrimsonWallHangingSign {
                facing,
                waterlogged,
            } => 5602u16 + *facing as u16 * 2 + !waterlogged as u16,
            WarpedWallHangingSign {
                facing,
                waterlogged,
            } => 5610u16 + *facing as u16 * 2 + !waterlogged as u16,
            BambooWallHangingSign {
                facing,
                waterlogged,
            } => 5618u16 + *facing as u16 * 2 + !waterlogged as u16,
            Lever {
                face,
                facing,
                powered,
            } => 5626u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            StonePressurePlate { powered } => 5650u16 + !powered as u16,
            IronDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                5652u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            OakPressurePlate { powered } => 5716u16 + !powered as u16,
            SprucePressurePlate { powered } => 5718u16 + !powered as u16,
            BirchPressurePlate { powered } => 5720u16 + !powered as u16,
            JunglePressurePlate { powered } => 5722u16 + !powered as u16,
            AcaciaPressurePlate { powered } => 5724u16 + !powered as u16,
            CherryPressurePlate { powered } => 5726u16 + !powered as u16,
            DarkOakPressurePlate { powered } => 5728u16 + !powered as u16,
            MangrovePressurePlate { powered } => 5730u16 + !powered as u16,
            BambooPressurePlate { powered } => 5732u16 + !powered as u16,
            RedstoneOre { lit } => 5734u16 + !lit as u16,
            DeepslateRedstoneOre { lit } => 5736u16 + !lit as u16,
            RedstoneTorch { lit } => 5738u16 + !lit as u16,
            RedstoneWallTorch { facing, lit } => 5740u16 + *facing as u16 * 2 + !lit as u16,
            StoneButton {
                facing,
                powered,
                face,
            } => 5748u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            Snow { layers } => 5772u16 + u16::from(*layers),
            Ice => 5780u16,
            SnowBlock => 5781u16,
            Cactus { age } => 5782u16 + u16::from(*age),
            Clay => 5798u16,
            SugarCane { age } => 5799u16 + u16::from(*age),
            Jukebox { has_record } => 5815u16 + !has_record as u16,
            OakFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                5817u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            Pumpkin => 5849u16,
            Netherrack => 5850u16,
            SoulSand => 5851u16,
            SoulSoil => 5852u16,
            Basalt { axis } => 5853u16 + *axis as u16,
            PolishedBasalt { axis } => 5856u16 + *axis as u16,
            SoulTorch => 5859u16,
            SoulWallTorch { facing } => 5860u16 + *facing as u16,
            Glowstone => 5864u16,
            NetherPortal { axis } => 5865u16 + *axis as u16,
            CarvedPumpkin { facing } => 5867u16 + *facing as u16,
            JackOLantern { facing } => 5871u16 + *facing as u16,
            Cake { bites } => 5875u16 + u16::from(*bites),
            Repeater {
                facing,
                delay,
                locked,
                powered,
            } => {
                5882u16
                    + u16::from(*delay) * 16
                    + *facing as u16 * 4
                    + !locked as u16 * 2
                    + !powered as u16
            }
            WhiteStainedGlass => 5946u16,
            OrangeStainedGlass => 5947u16,
            MagentaStainedGlass => 5948u16,
            LightBlueStainedGlass => 5949u16,
            YellowStainedGlass => 5950u16,
            LimeStainedGlass => 5951u16,
            PinkStainedGlass => 5952u16,
            GrayStainedGlass => 5953u16,
            LightGrayStainedGlass => 5954u16,
            CyanStainedGlass => 5955u16,
            PurpleStainedGlass => 5956u16,
            BlueStainedGlass => 5957u16,
            BrownStainedGlass => 5958u16,
            GreenStainedGlass => 5959u16,
            RedStainedGlass => 5960u16,
            BlackStainedGlass => 5961u16,
            OakTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                5962u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            SpruceTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6026u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            BirchTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6090u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            JungleTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6154u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            AcaciaTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6218u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            CherryTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6282u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            DarkOakTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6346u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            MangroveTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6410u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            BambooTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                6474u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            StoneBricks => 6538u16,
            MossyStoneBricks => 6539u16,
            CrackedStoneBricks => 6540u16,
            ChiseledStoneBricks => 6541u16,
            PackedMud => 6542u16,
            MudBricks => 6543u16,
            InfestedStone => 6544u16,
            InfestedCobblestone => 6545u16,
            InfestedStoneBricks => 6546u16,
            InfestedMossyStoneBricks => 6547u16,
            InfestedCrackedStoneBricks => 6548u16,
            InfestedChiseledStoneBricks => 6549u16,
            BrownMushroomBlock {
                north,
                east,
                south,
                west,
                up,
                down,
            } => {
                6550u16
                    + !down as u16 * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            RedMushroomBlock {
                north,
                east,
                south,
                west,
                up,
                down,
            } => {
                6614u16
                    + !down as u16 * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            MushroomStem {
                north,
                east,
                south,
                west,
                up,
                down,
            } => {
                6678u16
                    + !down as u16 * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            IronBars {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                6742u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            Chain { axis, waterlogged } => 6774u16 + *axis as u16 * 2 + !waterlogged as u16,
            GlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                6780u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            Melon => 6812u16,
            AttachedPumpkinStem { facing } => 6813u16 + *facing as u16,
            AttachedMelonStem { facing } => 6817u16 + *facing as u16,
            PumpkinStem { age } => 6821u16 + u16::from(*age),
            MelonStem { age } => 6829u16 + u16::from(*age),
            Vine {
                north,
                east,
                south,
                west,
                up,
            } => {
                6837u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            GlowLichen {
                north,
                east,
                south,
                west,
                up,
                down,
                waterlogged,
            } => {
                6869u16
                    + !down as u16 * 64
                    + !east as u16 * 32
                    + !north as u16 * 16
                    + !south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            OakFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                6997u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            BrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7029u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            StoneBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7109u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            MudBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7189u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            Mycelium { snowy } => 7269u16 + !snowy as u16,
            LilyPad => 7271u16,
            NetherBricks => 7272u16,
            NetherBrickFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                7273u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            NetherBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7305u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            NetherWart { age } => 7385u16 + u16::from(*age),
            EnchantingTable => 7389u16,
            BrewingStand {
                has_bottle_0,
                has_bottle_1,
                has_bottle_2,
            } => {
                7390u16 + !has_bottle_0 as u16 * 4 + !has_bottle_1 as u16 * 2 + !has_bottle_2 as u16
            }
            Cauldron => 7398u16,
            WaterCauldron { level } => 7399u16 + u16::from(*level),
            LavaCauldron => 7402u16,
            PowderSnowCauldron { level } => 7403u16 + u16::from(*level),
            EndPortal => 7406u16,
            EndPortalFrame { facing, eye } => 7407u16 + !eye as u16 * 4 + *facing as u16,
            EndStone => 7415u16,
            DragonEgg => 7416u16,
            RedstoneLamp { lit } => 7417u16 + !lit as u16,
            Cocoa { facing, age } => 7419u16 + u16::from(*age) * 4 + *facing as u16,
            SandstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7431u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            EmeraldOre => 7511u16,
            DeepslateEmeraldOre => 7512u16,
            EnderChest {
                facing,
                waterlogged,
            } => 7513u16 + *facing as u16 * 2 + !waterlogged as u16,
            TripwireHook {
                facing,
                powered,
                attached,
            } => 7521u16 + !attached as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            Tripwire {
                north,
                east,
                south,
                west,
                powered,
                attached,
                disarmed,
            } => {
                7537u16
                    + !attached as u16 * 64
                    + !disarmed as u16 * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !powered as u16 * 4
                    + !south as u16 * 2
                    + !west as u16
            }
            EmeraldBlock => 7665u16,
            SpruceStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7666u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            BirchStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7746u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            JungleStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                7826u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            CommandBlock {
                facing,
                conditional,
            } => 7906u16 + !conditional as u16 * 6 + *facing as u16,
            Beacon => 7918u16,
            CobblestoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                7919u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            MossyCobblestoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                8243u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            FlowerPot => 8567u16,
            PottedTorchflower => 8568u16,
            PottedOakSapling => 8569u16,
            PottedSpruceSapling => 8570u16,
            PottedBirchSapling => 8571u16,
            PottedJungleSapling => 8572u16,
            PottedAcaciaSapling => 8573u16,
            PottedCherrySapling => 8574u16,
            PottedDarkOakSapling => 8575u16,
            PottedMangrovePropagule => 8576u16,
            PottedFern => 8577u16,
            PottedDandelion => 8578u16,
            PottedPoppy => 8579u16,
            PottedBlueOrchid => 8580u16,
            PottedAllium => 8581u16,
            PottedAzureBluet => 8582u16,
            PottedRedTulip => 8583u16,
            PottedOrangeTulip => 8584u16,
            PottedWhiteTulip => 8585u16,
            PottedPinkTulip => 8586u16,
            PottedOxeyeDaisy => 8587u16,
            PottedCornflower => 8588u16,
            PottedLilyOfTheValley => 8589u16,
            PottedWitherRose => 8590u16,
            PottedRedMushroom => 8591u16,
            PottedBrownMushroom => 8592u16,
            PottedDeadBush => 8593u16,
            PottedCactus => 8594u16,
            Carrots { age } => 8595u16 + u16::from(*age),
            Potatoes { age } => 8603u16 + u16::from(*age),
            OakButton {
                facing,
                powered,
                face,
            } => 8611u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            SpruceButton {
                facing,
                powered,
                face,
            } => 8635u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            BirchButton {
                facing,
                powered,
                face,
            } => 8659u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            JungleButton {
                facing,
                powered,
                face,
            } => 8683u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            AcaciaButton {
                facing,
                powered,
                face,
            } => 8707u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            CherryButton {
                facing,
                powered,
                face,
            } => 8731u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            DarkOakButton {
                facing,
                powered,
                face,
            } => 8755u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            MangroveButton {
                facing,
                powered,
                face,
            } => 8779u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            BambooButton {
                facing,
                powered,
                face,
            } => 8803u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            SkeletonSkull { rotation } => 8827u16 + u16::from(*rotation),
            SkeletonWallSkull { facing } => 8843u16 + *facing as u16,
            WitherSkeletonSkull { rotation } => 8847u16 + u16::from(*rotation),
            WitherSkeletonWallSkull { facing } => 8863u16 + *facing as u16,
            ZombieHead { rotation } => 8867u16 + u16::from(*rotation),
            ZombieWallHead { facing } => 8883u16 + *facing as u16,
            PlayerHead { rotation } => 8887u16 + u16::from(*rotation),
            PlayerWallHead { facing } => 8903u16 + *facing as u16,
            CreeperHead { rotation } => 8907u16 + u16::from(*rotation),
            CreeperWallHead { facing } => 8923u16 + *facing as u16,
            DragonHead { rotation } => 8927u16 + u16::from(*rotation),
            DragonWallHead { facing } => 8943u16 + *facing as u16,
            PiglinHead { rotation } => 8947u16 + u16::from(*rotation),
            PiglinWallHead { facing } => 8963u16 + *facing as u16,
            Anvil { facing } => 8967u16 + *facing as u16,
            ChippedAnvil { facing } => 8971u16 + *facing as u16,
            DamagedAnvil { facing } => 8975u16 + *facing as u16,
            TrappedChest {
                facing,
                r#type,
                waterlogged,
            } => 8979u16 + *facing as u16 * 6 + *r#type as u16 * 2 + !waterlogged as u16,
            LightWeightedPressurePlate { power } => 9003u16 + u16::from(*power),
            HeavyWeightedPressurePlate { power } => 9019u16 + u16::from(*power),
            Comparator {
                facing,
                mode,
                powered,
            } => 9035u16 + *facing as u16 * 4 + *mode as u16 * 2 + !powered as u16,
            DaylightDetector { power, inverted } => {
                9051u16 + !inverted as u16 * 16 + u16::from(*power)
            }
            RedstoneBlock => 9083u16,
            NetherQuartzOre => 9084u16,
            Hopper { facing, enabled } => 9085u16 + !enabled as u16 * 5 + *facing as u16,
            QuartzBlock => 9095u16,
            ChiseledQuartzBlock => 9096u16,
            QuartzPillar { axis } => 9097u16 + *axis as u16,
            QuartzStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                9100u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            ActivatorRail {
                shape,
                powered,
                waterlogged,
            } => 9180u16 + !powered as u16 * 12 + *shape as u16 * 2 + *waterlogged as u16,
            Dropper { facing, triggered } => 9204u16 + *facing as u16 * 2 + !triggered as u16,
            WhiteTerracotta => 9216u16,
            OrangeTerracotta => 9217u16,
            MagentaTerracotta => 9218u16,
            LightBlueTerracotta => 9219u16,
            YellowTerracotta => 9220u16,
            LimeTerracotta => 9221u16,
            PinkTerracotta => 9222u16,
            GrayTerracotta => 9223u16,
            LightGrayTerracotta => 9224u16,
            CyanTerracotta => 9225u16,
            PurpleTerracotta => 9226u16,
            BlueTerracotta => 9227u16,
            BrownTerracotta => 9228u16,
            GreenTerracotta => 9229u16,
            RedTerracotta => 9230u16,
            BlackTerracotta => 9231u16,
            WhiteStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9232u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            OrangeStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9264u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            MagentaStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9296u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            LightBlueStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9328u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            YellowStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9360u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            LimeStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9392u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            PinkStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9424u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            GrayStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9456u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            LightGrayStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9488u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            CyanStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9520u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            PurpleStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9552u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            BlueStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9584u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            BrownStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9616u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            GreenStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9648u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            RedStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9680u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            BlackStainedGlassPane {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                9712u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            AcaciaStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                9744u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            CherryStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                9824u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            DarkOakStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                9904u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            MangroveStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                9984u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            BambooStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10064u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            BambooMosaicStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10144u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            SlimeBlock => 10224u16,
            Barrier => 10225u16,
            Light { level, waterlogged } => 10226u16 + u16::from(*level) * 2 + !waterlogged as u16,
            IronTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                10258u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            Prismarine => 10322u16,
            PrismarineBricks => 10323u16,
            DarkPrismarine => 10324u16,
            PrismarineStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10325u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PrismarineBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10405u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            DarkPrismarineStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10485u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PrismarineSlab {
                r#type,
                waterlogged,
            } => 10565u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PrismarineBrickSlab {
                r#type,
                waterlogged,
            } => 10571u16 + *r#type as u16 * 2 + !waterlogged as u16,
            DarkPrismarineSlab {
                r#type,
                waterlogged,
            } => 10577u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SeaLantern => 10583u16,
            HayBlock { axis } => 10584u16 + *axis as u16,
            WhiteCarpet => 10587u16,
            OrangeCarpet => 10588u16,
            MagentaCarpet => 10589u16,
            LightBlueCarpet => 10590u16,
            YellowCarpet => 10591u16,
            LimeCarpet => 10592u16,
            PinkCarpet => 10593u16,
            GrayCarpet => 10594u16,
            LightGrayCarpet => 10595u16,
            CyanCarpet => 10596u16,
            PurpleCarpet => 10597u16,
            BlueCarpet => 10598u16,
            BrownCarpet => 10599u16,
            GreenCarpet => 10600u16,
            RedCarpet => 10601u16,
            BlackCarpet => 10602u16,
            Terracotta => 10603u16,
            CoalBlock => 10604u16,
            PackedIce => 10605u16,
            Sunflower { half } => 10606u16 + *half as u16,
            Lilac { half } => 10608u16 + *half as u16,
            RoseBush { half } => 10610u16 + *half as u16,
            Peony { half } => 10612u16 + *half as u16,
            TallGrass { half } => 10614u16 + *half as u16,
            LargeFern { half } => 10616u16 + *half as u16,
            WhiteBanner { rotation } => 10618u16 + u16::from(*rotation),
            OrangeBanner { rotation } => 10634u16 + u16::from(*rotation),
            MagentaBanner { rotation } => 10650u16 + u16::from(*rotation),
            LightBlueBanner { rotation } => 10666u16 + u16::from(*rotation),
            YellowBanner { rotation } => 10682u16 + u16::from(*rotation),
            LimeBanner { rotation } => 10698u16 + u16::from(*rotation),
            PinkBanner { rotation } => 10714u16 + u16::from(*rotation),
            GrayBanner { rotation } => 10730u16 + u16::from(*rotation),
            LightGrayBanner { rotation } => 10746u16 + u16::from(*rotation),
            CyanBanner { rotation } => 10762u16 + u16::from(*rotation),
            PurpleBanner { rotation } => 10778u16 + u16::from(*rotation),
            BlueBanner { rotation } => 10794u16 + u16::from(*rotation),
            BrownBanner { rotation } => 10810u16 + u16::from(*rotation),
            GreenBanner { rotation } => 10826u16 + u16::from(*rotation),
            RedBanner { rotation } => 10842u16 + u16::from(*rotation),
            BlackBanner { rotation } => 10858u16 + u16::from(*rotation),
            WhiteWallBanner { facing } => 10874u16 + *facing as u16,
            OrangeWallBanner { facing } => 10878u16 + *facing as u16,
            MagentaWallBanner { facing } => 10882u16 + *facing as u16,
            LightBlueWallBanner { facing } => 10886u16 + *facing as u16,
            YellowWallBanner { facing } => 10890u16 + *facing as u16,
            LimeWallBanner { facing } => 10894u16 + *facing as u16,
            PinkWallBanner { facing } => 10898u16 + *facing as u16,
            GrayWallBanner { facing } => 10902u16 + *facing as u16,
            LightGrayWallBanner { facing } => 10906u16 + *facing as u16,
            CyanWallBanner { facing } => 10910u16 + *facing as u16,
            PurpleWallBanner { facing } => 10914u16 + *facing as u16,
            BlueWallBanner { facing } => 10918u16 + *facing as u16,
            BrownWallBanner { facing } => 10922u16 + *facing as u16,
            GreenWallBanner { facing } => 10926u16 + *facing as u16,
            RedWallBanner { facing } => 10930u16 + *facing as u16,
            BlackWallBanner { facing } => 10934u16 + *facing as u16,
            RedSandstone => 10938u16,
            ChiseledRedSandstone => 10939u16,
            CutRedSandstone => 10940u16,
            RedSandstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                10941u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            OakSlab {
                r#type,
                waterlogged,
            } => 11021u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SpruceSlab {
                r#type,
                waterlogged,
            } => 11027u16 + *r#type as u16 * 2 + !waterlogged as u16,
            BirchSlab {
                r#type,
                waterlogged,
            } => 11033u16 + *r#type as u16 * 2 + !waterlogged as u16,
            JungleSlab {
                r#type,
                waterlogged,
            } => 11039u16 + *r#type as u16 * 2 + !waterlogged as u16,
            AcaciaSlab {
                r#type,
                waterlogged,
            } => 11045u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CherrySlab {
                r#type,
                waterlogged,
            } => 11051u16 + *r#type as u16 * 2 + !waterlogged as u16,
            DarkOakSlab {
                r#type,
                waterlogged,
            } => 11057u16 + *r#type as u16 * 2 + !waterlogged as u16,
            MangroveSlab {
                r#type,
                waterlogged,
            } => 11063u16 + *r#type as u16 * 2 + !waterlogged as u16,
            BambooSlab {
                r#type,
                waterlogged,
            } => 11069u16 + *r#type as u16 * 2 + !waterlogged as u16,
            BambooMosaicSlab {
                r#type,
                waterlogged,
            } => 11075u16 + *r#type as u16 * 2 + !waterlogged as u16,
            StoneSlab {
                r#type,
                waterlogged,
            } => 11081u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SmoothStoneSlab {
                r#type,
                waterlogged,
            } => 11087u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SandstoneSlab {
                r#type,
                waterlogged,
            } => 11093u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CutSandstoneSlab {
                r#type,
                waterlogged,
            } => 11099u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PetrifiedOakSlab {
                r#type,
                waterlogged,
            } => 11105u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CobblestoneSlab {
                r#type,
                waterlogged,
            } => 11111u16 + *r#type as u16 * 2 + !waterlogged as u16,
            BrickSlab {
                r#type,
                waterlogged,
            } => 11117u16 + *r#type as u16 * 2 + !waterlogged as u16,
            StoneBrickSlab {
                r#type,
                waterlogged,
            } => 11123u16 + *r#type as u16 * 2 + !waterlogged as u16,
            MudBrickSlab {
                r#type,
                waterlogged,
            } => 11129u16 + *r#type as u16 * 2 + !waterlogged as u16,
            NetherBrickSlab {
                r#type,
                waterlogged,
            } => 11135u16 + *r#type as u16 * 2 + !waterlogged as u16,
            QuartzSlab {
                r#type,
                waterlogged,
            } => 11141u16 + *r#type as u16 * 2 + !waterlogged as u16,
            RedSandstoneSlab {
                r#type,
                waterlogged,
            } => 11147u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CutRedSandstoneSlab {
                r#type,
                waterlogged,
            } => 11153u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PurpurSlab {
                r#type,
                waterlogged,
            } => 11159u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SmoothStone => 11165u16,
            SmoothSandstone => 11166u16,
            SmoothQuartz => 11167u16,
            SmoothRedSandstone => 11168u16,
            SpruceFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11169u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            BirchFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11201u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            JungleFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11233u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            AcaciaFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11265u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            CherryFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11297u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            DarkOakFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11329u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            MangroveFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11361u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            BambooFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                11393u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            SpruceFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11425u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            BirchFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11457u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            JungleFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11489u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            AcaciaFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11521u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            CherryFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11553u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            DarkOakFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11585u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            MangroveFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11617u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            BambooFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                11649u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            SpruceDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                11681u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            BirchDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                11745u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            JungleDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                11809u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            AcaciaDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                11873u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            CherryDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                11937u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            DarkOakDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                12001u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            MangroveDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                12065u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            BambooDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                12129u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            EndRod { facing } => 12193u16 + *facing as u16,
            ChorusPlant {
                north,
                east,
                south,
                west,
                up,
                down,
            } => {
                12199u16
                    + !down as u16 * 32
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !up as u16 * 2
                    + !west as u16
            }
            ChorusFlower { age } => 12263u16 + u16::from(*age),
            PurpurBlock => 12269u16,
            PurpurPillar { axis } => 12270u16 + *axis as u16,
            PurpurStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                12273u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            EndStoneBricks => 12353u16,
            TorchflowerCrop { age } => 12354u16 + u16::from(*age),
            PitcherCrop => 12356u16,
            PitcherPlant { half } => 12366u16 + *half as u16,
            Beetroots { age } => 12368u16 + u16::from(*age),
            DirtPath => 12372u16,
            EndGateway => 12373u16,
            RepeatingCommandBlock {
                facing,
                conditional,
            } => 12374u16 + !conditional as u16 * 6 + *facing as u16,
            ChainCommandBlock {
                facing,
                conditional,
            } => 12386u16 + !conditional as u16 * 6 + *facing as u16,
            FrostedIce { age } => 12398u16 + u16::from(*age),
            MagmaBlock => 12402u16,
            NetherWartBlock => 12403u16,
            RedNetherBricks => 12404u16,
            BoneBlock { axis } => 12405u16 + *axis as u16,
            StructureVoid => 12408u16,
            Observer { facing, powered } => 12409u16 + *facing as u16 * 2 + !powered as u16,
            ShulkerBox { facing } => 12421u16 + *facing as u16,
            WhiteShulkerBox { facing } => 12427u16 + *facing as u16,
            OrangeShulkerBox { facing } => 12433u16 + *facing as u16,
            MagentaShulkerBox { facing } => 12439u16 + *facing as u16,
            LightBlueShulkerBox { facing } => 12445u16 + *facing as u16,
            YellowShulkerBox { facing } => 12451u16 + *facing as u16,
            LimeShulkerBox { facing } => 12457u16 + *facing as u16,
            PinkShulkerBox { facing } => 12463u16 + *facing as u16,
            GrayShulkerBox { facing } => 12469u16 + *facing as u16,
            LightGrayShulkerBox { facing } => 12475u16 + *facing as u16,
            CyanShulkerBox { facing } => 12481u16 + *facing as u16,
            PurpleShulkerBox { facing } => 12487u16 + *facing as u16,
            BlueShulkerBox { facing } => 12493u16 + *facing as u16,
            BrownShulkerBox { facing } => 12499u16 + *facing as u16,
            GreenShulkerBox { facing } => 12505u16 + *facing as u16,
            RedShulkerBox { facing } => 12511u16 + *facing as u16,
            BlackShulkerBox { facing } => 12517u16 + *facing as u16,
            WhiteGlazedTerracotta { facing } => 12523u16 + *facing as u16,
            OrangeGlazedTerracotta { facing } => 12527u16 + *facing as u16,
            MagentaGlazedTerracotta { facing } => 12531u16 + *facing as u16,
            LightBlueGlazedTerracotta { facing } => 12535u16 + *facing as u16,
            YellowGlazedTerracotta { facing } => 12539u16 + *facing as u16,
            LimeGlazedTerracotta { facing } => 12543u16 + *facing as u16,
            PinkGlazedTerracotta { facing } => 12547u16 + *facing as u16,
            GrayGlazedTerracotta { facing } => 12551u16 + *facing as u16,
            LightGrayGlazedTerracotta { facing } => 12555u16 + *facing as u16,
            CyanGlazedTerracotta { facing } => 12559u16 + *facing as u16,
            PurpleGlazedTerracotta { facing } => 12563u16 + *facing as u16,
            BlueGlazedTerracotta { facing } => 12567u16 + *facing as u16,
            BrownGlazedTerracotta { facing } => 12571u16 + *facing as u16,
            GreenGlazedTerracotta { facing } => 12575u16 + *facing as u16,
            RedGlazedTerracotta { facing } => 12579u16 + *facing as u16,
            BlackGlazedTerracotta { facing } => 12583u16 + *facing as u16,
            WhiteConcrete => 12587u16,
            OrangeConcrete => 12588u16,
            MagentaConcrete => 12589u16,
            LightBlueConcrete => 12590u16,
            YellowConcrete => 12591u16,
            LimeConcrete => 12592u16,
            PinkConcrete => 12593u16,
            GrayConcrete => 12594u16,
            LightGrayConcrete => 12595u16,
            CyanConcrete => 12596u16,
            PurpleConcrete => 12597u16,
            BlueConcrete => 12598u16,
            BrownConcrete => 12599u16,
            GreenConcrete => 12600u16,
            RedConcrete => 12601u16,
            BlackConcrete => 12602u16,
            WhiteConcretePowder => 12603u16,
            OrangeConcretePowder => 12604u16,
            MagentaConcretePowder => 12605u16,
            LightBlueConcretePowder => 12606u16,
            YellowConcretePowder => 12607u16,
            LimeConcretePowder => 12608u16,
            PinkConcretePowder => 12609u16,
            GrayConcretePowder => 12610u16,
            LightGrayConcretePowder => 12611u16,
            CyanConcretePowder => 12612u16,
            PurpleConcretePowder => 12613u16,
            BlueConcretePowder => 12614u16,
            BrownConcretePowder => 12615u16,
            GreenConcretePowder => 12616u16,
            RedConcretePowder => 12617u16,
            BlackConcretePowder => 12618u16,
            Kelp { age } => 12619u16 + u16::from(*age),
            KelpPlant => 12645u16,
            DriedKelpBlock => 12646u16,
            TurtleEgg { hatch, eggs } => 12647u16 + u16::from(*eggs) * 3 + u16::from(*hatch),
            SnifferEgg { hatch } => 12659u16 + u16::from(*hatch),
            DeadTubeCoralBlock => 12662u16,
            DeadBrainCoralBlock => 12663u16,
            DeadBubbleCoralBlock => 12664u16,
            DeadFireCoralBlock => 12665u16,
            DeadHornCoralBlock => 12666u16,
            TubeCoralBlock => 12667u16,
            BrainCoralBlock => 12668u16,
            BubbleCoralBlock => 12669u16,
            FireCoralBlock => 12670u16,
            HornCoralBlock => 12671u16,
            DeadTubeCoral { waterlogged } => 12672u16 + !waterlogged as u16,
            DeadBrainCoral { waterlogged } => 12674u16 + !waterlogged as u16,
            DeadBubbleCoral { waterlogged } => 12676u16 + !waterlogged as u16,
            DeadFireCoral { waterlogged } => 12678u16 + !waterlogged as u16,
            DeadHornCoral { waterlogged } => 12680u16 + !waterlogged as u16,
            TubeCoral { waterlogged } => 12682u16 + !waterlogged as u16,
            BrainCoral { waterlogged } => 12684u16 + !waterlogged as u16,
            BubbleCoral { waterlogged } => 12686u16 + !waterlogged as u16,
            FireCoral { waterlogged } => 12688u16 + !waterlogged as u16,
            HornCoral { waterlogged } => 12690u16 + !waterlogged as u16,
            DeadTubeCoralFan { waterlogged } => 12692u16 + !waterlogged as u16,
            DeadBrainCoralFan { waterlogged } => 12694u16 + !waterlogged as u16,
            DeadBubbleCoralFan { waterlogged } => 12696u16 + !waterlogged as u16,
            DeadFireCoralFan { waterlogged } => 12698u16 + !waterlogged as u16,
            DeadHornCoralFan { waterlogged } => 12700u16 + !waterlogged as u16,
            TubeCoralFan { waterlogged } => 12702u16 + !waterlogged as u16,
            BrainCoralFan { waterlogged } => 12704u16 + !waterlogged as u16,
            BubbleCoralFan { waterlogged } => 12706u16 + !waterlogged as u16,
            FireCoralFan { waterlogged } => 12708u16 + !waterlogged as u16,
            HornCoralFan { waterlogged } => 12710u16 + !waterlogged as u16,
            DeadTubeCoralWallFan {
                facing,
                waterlogged,
            } => 12712u16 + *facing as u16 * 2 + !waterlogged as u16,
            DeadBrainCoralWallFan {
                facing,
                waterlogged,
            } => 12720u16 + *facing as u16 * 2 + !waterlogged as u16,
            DeadBubbleCoralWallFan {
                facing,
                waterlogged,
            } => 12728u16 + *facing as u16 * 2 + !waterlogged as u16,
            DeadFireCoralWallFan {
                facing,
                waterlogged,
            } => 12736u16 + *facing as u16 * 2 + !waterlogged as u16,
            DeadHornCoralWallFan {
                facing,
                waterlogged,
            } => 12744u16 + *facing as u16 * 2 + !waterlogged as u16,
            TubeCoralWallFan {
                facing,
                waterlogged,
            } => 12752u16 + *facing as u16 * 2 + !waterlogged as u16,
            BrainCoralWallFan {
                facing,
                waterlogged,
            } => 12760u16 + *facing as u16 * 2 + !waterlogged as u16,
            BubbleCoralWallFan {
                facing,
                waterlogged,
            } => 12768u16 + *facing as u16 * 2 + !waterlogged as u16,
            FireCoralWallFan {
                facing,
                waterlogged,
            } => 12776u16 + *facing as u16 * 2 + !waterlogged as u16,
            HornCoralWallFan {
                facing,
                waterlogged,
            } => 12784u16 + *facing as u16 * 2 + !waterlogged as u16,
            SeaPickle {
                pickles,
                waterlogged,
            } => 12792u16 + u16::from(*pickles) * 2 + !waterlogged as u16,
            BlueIce => 12800u16,
            Conduit { waterlogged } => 12801u16 + !waterlogged as u16,
            BambooSapling => 12803u16,
            Bamboo { age, leaves, stage } => {
                12804u16 + u16::from(*age) * 6 + *leaves as u16 * 2 + u16::from(*stage)
            }
            PottedBamboo => 12816u16,
            VoidAir => 12817u16,
            CaveAir => 12818u16,
            BubbleColumn { drag } => 12819u16 + !drag as u16,
            PolishedGraniteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                12821u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            SmoothRedSandstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                12901u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            MossyStoneBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                12981u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedDioriteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13061u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            MossyCobblestoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13141u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            EndStoneBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13221u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            StoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13301u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            SmoothSandstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13381u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            SmoothQuartzStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13461u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            GraniteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13541u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            AndesiteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13621u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            RedNetherBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13701u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedAndesiteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13781u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            DioriteStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                13861u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedGraniteSlab {
                r#type,
                waterlogged,
            } => 13941u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SmoothRedSandstoneSlab {
                r#type,
                waterlogged,
            } => 13947u16 + *r#type as u16 * 2 + !waterlogged as u16,
            MossyStoneBrickSlab {
                r#type,
                waterlogged,
            } => 13953u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedDioriteSlab {
                r#type,
                waterlogged,
            } => 13959u16 + *r#type as u16 * 2 + !waterlogged as u16,
            MossyCobblestoneSlab {
                r#type,
                waterlogged,
            } => 13965u16 + *r#type as u16 * 2 + !waterlogged as u16,
            EndStoneBrickSlab {
                r#type,
                waterlogged,
            } => 13971u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SmoothSandstoneSlab {
                r#type,
                waterlogged,
            } => 13977u16 + *r#type as u16 * 2 + !waterlogged as u16,
            SmoothQuartzSlab {
                r#type,
                waterlogged,
            } => 13983u16 + *r#type as u16 * 2 + !waterlogged as u16,
            GraniteSlab {
                r#type,
                waterlogged,
            } => 13989u16 + *r#type as u16 * 2 + !waterlogged as u16,
            AndesiteSlab {
                r#type,
                waterlogged,
            } => 13995u16 + *r#type as u16 * 2 + !waterlogged as u16,
            RedNetherBrickSlab {
                r#type,
                waterlogged,
            } => 14001u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedAndesiteSlab {
                r#type,
                waterlogged,
            } => 14007u16 + *r#type as u16 * 2 + !waterlogged as u16,
            DioriteSlab {
                r#type,
                waterlogged,
            } => 14013u16 + *r#type as u16 * 2 + !waterlogged as u16,
            BrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                14019u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            PrismarineWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                14343u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            RedSandstoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                14667u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            MossyStoneBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                14991u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            GraniteWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                15315u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            StoneBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                15639u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            MudBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                15963u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            NetherBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                16287u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            AndesiteWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                16611u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            RedNetherBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                16935u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            SandstoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                17259u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            EndStoneBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                17583u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            DioriteWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                17907u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            Scaffolding {
                distance,
                waterlogged,
                bottom,
            } => 18231u16 + !bottom as u16 * 16 + u16::from(*distance) * 2 + !waterlogged as u16,
            Loom { facing } => 18263u16 + *facing as u16,
            Barrel { facing, open } => 18267u16 + *facing as u16 * 2 + !open as u16,
            Smoker { facing, lit } => 18279u16 + *facing as u16 * 2 + !lit as u16,
            BlastFurnace { facing, lit } => 18287u16 + *facing as u16 * 2 + !lit as u16,
            CartographyTable => 18295u16,
            FletchingTable => 18296u16,
            Grindstone { facing, face } => 18297u16 + *face as u16 * 4 + *facing as u16,
            Lectern {
                facing,
                powered,
                has_book,
            } => 18309u16 + *facing as u16 * 4 + !has_book as u16 * 2 + !powered as u16,
            SmithingTable => 18325u16,
            Stonecutter { facing } => 18326u16 + *facing as u16,
            Bell {
                facing,
                attachment,
                powered,
            } => 18330u16 + *attachment as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            Lantern {
                hanging,
                waterlogged,
            } => 18362u16 + !hanging as u16 * 2 + !waterlogged as u16,
            SoulLantern {
                hanging,
                waterlogged,
            } => 18366u16 + !hanging as u16 * 2 + !waterlogged as u16,
            Campfire {
                facing,
                lit,
                signal_fire,
                waterlogged,
            } => {
                18370u16
                    + *facing as u16 * 8
                    + !lit as u16 * 4
                    + !signal_fire as u16 * 2
                    + !waterlogged as u16
            }
            SoulCampfire {
                facing,
                lit,
                signal_fire,
                waterlogged,
            } => {
                18402u16
                    + *facing as u16 * 8
                    + !lit as u16 * 4
                    + !signal_fire as u16 * 2
                    + !waterlogged as u16
            }
            SweetBerryBush { age } => 18434u16 + u16::from(*age),
            WarpedStem { axis } => 18438u16 + *axis as u16,
            StrippedWarpedStem { axis } => 18441u16 + *axis as u16,
            WarpedHyphae { axis } => 18444u16 + *axis as u16,
            StrippedWarpedHyphae { axis } => 18447u16 + *axis as u16,
            WarpedNylium => 18450u16,
            WarpedFungus => 18451u16,
            WarpedWartBlock => 18452u16,
            WarpedRoots => 18453u16,
            NetherSprouts => 18454u16,
            CrimsonStem { axis } => 18455u16 + *axis as u16,
            StrippedCrimsonStem { axis } => 18458u16 + *axis as u16,
            CrimsonHyphae { axis } => 18461u16 + *axis as u16,
            StrippedCrimsonHyphae { axis } => 18464u16 + *axis as u16,
            CrimsonNylium => 18467u16,
            CrimsonFungus => 18468u16,
            Shroomlight => 18469u16,
            WeepingVines { age } => 18470u16 + u16::from(*age),
            WeepingVinesPlant => 18496u16,
            TwistingVines { age } => 18497u16 + u16::from(*age),
            TwistingVinesPlant => 18523u16,
            CrimsonRoots => 18524u16,
            CrimsonPlanks => 18525u16,
            WarpedPlanks => 18526u16,
            CrimsonSlab {
                r#type,
                waterlogged,
            } => 18527u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WarpedSlab {
                r#type,
                waterlogged,
            } => 18533u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CrimsonPressurePlate { powered } => 18539u16 + !powered as u16,
            WarpedPressurePlate { powered } => 18541u16 + !powered as u16,
            CrimsonFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                18543u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            WarpedFence {
                north,
                east,
                south,
                west,
                waterlogged,
            } => {
                18575u16
                    + !east as u16 * 16
                    + !north as u16 * 8
                    + !south as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            CrimsonTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                18607u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            WarpedTrapdoor {
                facing,
                open,
                half,
                powered,
                waterlogged,
            } => {
                18671u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *open as u16 * 4
                    + *powered as u16 * 2
                    + *waterlogged as u16
            }
            CrimsonFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                18735u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            WarpedFenceGate {
                facing,
                open,
                powered,
                in_wall,
            } => {
                18767u16
                    + *facing as u16 * 8
                    + !in_wall as u16 * 4
                    + !open as u16 * 2
                    + !powered as u16
            }
            CrimsonStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                18799u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WarpedStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                18879u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            CrimsonButton {
                facing,
                powered,
                face,
            } => 18959u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            WarpedButton {
                facing,
                powered,
                face,
            } => 18983u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            CrimsonDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                19007u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            WarpedDoor {
                half,
                facing,
                open,
                hinge,
                powered,
            } => {
                19071u16
                    + *facing as u16 * 16
                    + *half as u16 * 8
                    + *hinge as u16 * 4
                    + *open as u16 * 2
                    + *powered as u16
            }
            CrimsonSign {
                rotation,
                waterlogged,
            } => 19135u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            WarpedSign {
                rotation,
                waterlogged,
            } => 19167u16 + u16::from(*rotation) * 2 + !waterlogged as u16,
            CrimsonWallSign {
                facing,
                waterlogged,
            } => 19199u16 + *facing as u16 * 2 + !waterlogged as u16,
            WarpedWallSign {
                facing,
                waterlogged,
            } => 19207u16 + *facing as u16 * 2 + !waterlogged as u16,
            StructureBlock { mode } => 19215u16 + *mode as u16,
            Jigsaw { orientation } => 19219u16 + *orientation as u16,
            Composter { level } => 19231u16 + u16::from(*level),
            Target { power } => 19240u16 + u16::from(*power),
            BeeNest {
                honey_level,
                facing,
            } => 19256u16 + *facing as u16 * 6 + u16::from(*honey_level),
            Beehive {
                honey_level,
                facing,
            } => 19280u16 + *facing as u16 * 6 + u16::from(*honey_level),
            HoneyBlock => 19304u16,
            HoneycombBlock => 19305u16,
            NetheriteBlock => 19306u16,
            AncientDebris => 19307u16,
            CryingObsidian => 19308u16,
            RespawnAnchor { charges } => 19309u16 + u16::from(*charges),
            PottedCrimsonFungus => 19314u16,
            PottedWarpedFungus => 19315u16,
            PottedCrimsonRoots => 19316u16,
            PottedWarpedRoots => 19317u16,
            Lodestone => 19318u16,
            Blackstone => 19319u16,
            BlackstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                19320u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            BlackstoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                19400u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            BlackstoneSlab {
                r#type,
                waterlogged,
            } => 19724u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedBlackstone => 19730u16,
            PolishedBlackstoneBricks => 19731u16,
            CrackedPolishedBlackstoneBricks => 19732u16,
            ChiseledPolishedBlackstone => 19733u16,
            PolishedBlackstoneBrickSlab {
                r#type,
                waterlogged,
            } => 19734u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedBlackstoneBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                19740u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedBlackstoneBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                19820u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            GildedBlackstone => 20144u16,
            PolishedBlackstoneStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                20145u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedBlackstoneSlab {
                r#type,
                waterlogged,
            } => 20225u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedBlackstonePressurePlate { powered } => 20231u16 + !powered as u16,
            PolishedBlackstoneButton {
                facing,
                powered,
                face,
            } => 20233u16 + *face as u16 * 8 + *facing as u16 * 2 + !powered as u16,
            PolishedBlackstoneWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                20257u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            ChiseledNetherBricks => 20581u16,
            CrackedNetherBricks => 20582u16,
            QuartzBricks => 20583u16,
            Candle {
                candles,
                lit,
                waterlogged,
            } => 20584u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            WhiteCandle {
                candles,
                lit,
                waterlogged,
            } => 20600u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            OrangeCandle {
                candles,
                lit,
                waterlogged,
            } => 20616u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            MagentaCandle {
                candles,
                lit,
                waterlogged,
            } => 20632u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            LightBlueCandle {
                candles,
                lit,
                waterlogged,
            } => 20648u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            YellowCandle {
                candles,
                lit,
                waterlogged,
            } => 20664u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            LimeCandle {
                candles,
                lit,
                waterlogged,
            } => 20680u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            PinkCandle {
                candles,
                lit,
                waterlogged,
            } => 20696u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            GrayCandle {
                candles,
                lit,
                waterlogged,
            } => 20712u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            LightGrayCandle {
                candles,
                lit,
                waterlogged,
            } => 20728u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            CyanCandle {
                candles,
                lit,
                waterlogged,
            } => 20744u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            PurpleCandle {
                candles,
                lit,
                waterlogged,
            } => 20760u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            BlueCandle {
                candles,
                lit,
                waterlogged,
            } => 20776u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            BrownCandle {
                candles,
                lit,
                waterlogged,
            } => 20792u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            GreenCandle {
                candles,
                lit,
                waterlogged,
            } => 20808u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            RedCandle {
                candles,
                lit,
                waterlogged,
            } => 20824u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            BlackCandle {
                candles,
                lit,
                waterlogged,
            } => 20840u16 + u16::from(*candles) * 4 + !lit as u16 + !waterlogged as u16,
            CandleCake { lit } => 20856u16 + !lit as u16,
            WhiteCandleCake { lit } => 20858u16 + !lit as u16,
            OrangeCandleCake { lit } => 20860u16 + !lit as u16,
            MagentaCandleCake { lit } => 20862u16 + !lit as u16,
            LightBlueCandleCake { lit } => 20864u16 + !lit as u16,
            YellowCandleCake { lit } => 20866u16 + !lit as u16,
            LimeCandleCake { lit } => 20868u16 + !lit as u16,
            PinkCandleCake { lit } => 20870u16 + !lit as u16,
            GrayCandleCake { lit } => 20872u16 + !lit as u16,
            LightGrayCandleCake { lit } => 20874u16 + !lit as u16,
            CyanCandleCake { lit } => 20876u16 + !lit as u16,
            PurpleCandleCake { lit } => 20878u16 + !lit as u16,
            BlueCandleCake { lit } => 20880u16 + !lit as u16,
            BrownCandleCake { lit } => 20882u16 + !lit as u16,
            GreenCandleCake { lit } => 20884u16 + !lit as u16,
            RedCandleCake { lit } => 20886u16 + !lit as u16,
            BlackCandleCake { lit } => 20888u16 + !lit as u16,
            AmethystBlock => 20890u16,
            BuddingAmethyst => 20891u16,
            AmethystCluster {
                facing,
                waterlogged,
            } => 20892u16 + *facing as u16 * 2 + !waterlogged as u16,
            LargeAmethystBud {
                facing,
                waterlogged,
            } => 20904u16 + *facing as u16 * 2 + !waterlogged as u16,
            MediumAmethystBud {
                facing,
                waterlogged,
            } => 20916u16 + *facing as u16 * 2 + !waterlogged as u16,
            SmallAmethystBud {
                facing,
                waterlogged,
            } => 20928u16 + *facing as u16 * 2 + !waterlogged as u16,
            Tuff => 20940u16,
            Calcite => 20941u16,
            TintedGlass => 20942u16,
            PowderSnow => 20943u16,
            SculkSensor {
                sculk_sensor_phase,
                power,
                waterlogged,
            } => {
                20944u16
                    + u16::from(*power) * 6
                    + *sculk_sensor_phase as u16 * 2
                    + !waterlogged as u16
            }
            CalibratedSculkSensor {
                sculk_sensor_phase,
                power,
                waterlogged,
                facing,
            } => {
                21040u16
                    + *facing as u16 * 96
                    + u16::from(*power) * 6
                    + *sculk_sensor_phase as u16 * 2
                    + !waterlogged as u16
            }
            Sculk => 21424u16,
            SculkVein {
                north,
                east,
                south,
                west,
                up,
                down,
                waterlogged,
            } => {
                21425u16
                    + !down as u16 * 64
                    + !east as u16 * 32
                    + !north as u16 * 16
                    + !south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + !west as u16
            }
            SculkCatalyst { bloom } => 21553u16 + !bloom as u16,
            SculkShrieker {
                shrieking,
                waterlogged,
                can_summon,
            } => 21555u16 + !can_summon as u16 * 4 + !shrieking as u16 * 2 + !waterlogged as u16,
            OxidizedCopper => 21563u16,
            WeatheredCopper => 21564u16,
            ExposedCopper => 21565u16,
            CopperBlock => 21566u16,
            CopperOre => 21567u16,
            DeepslateCopperOre => 21568u16,
            OxidizedCutCopper => 21569u16,
            WeatheredCutCopper => 21570u16,
            ExposedCutCopper => 21571u16,
            CutCopper => 21572u16,
            OxidizedCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                21573u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WeatheredCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                21653u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            ExposedCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                21733u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            CutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                21813u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            OxidizedCutCopperSlab {
                r#type,
                waterlogged,
            } => 21893u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WeatheredCutCopperSlab {
                r#type,
                waterlogged,
            } => 21899u16 + *r#type as u16 * 2 + !waterlogged as u16,
            ExposedCutCopperSlab {
                r#type,
                waterlogged,
            } => 21905u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CutCopperSlab {
                r#type,
                waterlogged,
            } => 21911u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WaxedCopperBlock => 21917u16,
            WaxedWeatheredCopper => 21918u16,
            WaxedExposedCopper => 21919u16,
            WaxedOxidizedCopper => 21920u16,
            WaxedOxidizedCutCopper => 21921u16,
            WaxedWeatheredCutCopper => 21922u16,
            WaxedExposedCutCopper => 21923u16,
            WaxedCutCopper => 21924u16,
            WaxedOxidizedCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                21925u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WaxedWeatheredCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                22005u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WaxedExposedCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                22085u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WaxedCutCopperStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                22165u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            WaxedOxidizedCutCopperSlab {
                r#type,
                waterlogged,
            } => 22245u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WaxedWeatheredCutCopperSlab {
                r#type,
                waterlogged,
            } => 22251u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WaxedExposedCutCopperSlab {
                r#type,
                waterlogged,
            } => 22257u16 + *r#type as u16 * 2 + !waterlogged as u16,
            WaxedCutCopperSlab {
                r#type,
                waterlogged,
            } => 22263u16 + *r#type as u16 * 2 + !waterlogged as u16,
            LightningRod {
                facing,
                powered,
                waterlogged,
            } => 22269u16 + *facing as u16 * 4 + !powered as u16 + !waterlogged as u16,
            PointedDripstone {
                vertical_direction,
                thickness,
                waterlogged,
            } => {
                22293u16
                    + *thickness as u16 * 4
                    + *vertical_direction as u16 * 2
                    + !waterlogged as u16
            }
            DripstoneBlock => 22313u16,
            CaveVines { age, berries } => 22314u16 + u16::from(*age) * 2 + !berries as u16,
            CaveVinesPlant { berries } => 22366u16 + !berries as u16,
            SporeBlossom => 22368u16,
            Azalea => 22369u16,
            FloweringAzalea => 22370u16,
            MossCarpet => 22371u16,
            PinkPetals {
                facing,
                flower_amount,
            } => 22372u16 + *facing as u16 * 4 + u16::from(*flower_amount),
            MossBlock => 22388u16,
            BigDripleaf {
                facing,
                tilt,
                waterlogged,
            } => 22389u16 + *facing as u16 * 8 + *tilt as u16 * 2 + !waterlogged as u16,
            BigDripleafStem {
                facing,
                waterlogged,
            } => 22421u16 + *facing as u16 * 2 + !waterlogged as u16,
            SmallDripleaf {
                facing,
                half,
                waterlogged,
            } => 22429u16 + *facing as u16 * 4 + *half as u16 * 2 + !waterlogged as u16,
            HangingRoots { waterlogged } => 22445u16 + !waterlogged as u16,
            RootedDirt => 22447u16,
            Mud => 22448u16,
            Deepslate { axis } => 22449u16 + *axis as u16,
            CobbledDeepslate => 22452u16,
            CobbledDeepslateStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                22453u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            CobbledDeepslateSlab {
                r#type,
                waterlogged,
            } => 22533u16 + *r#type as u16 * 2 + !waterlogged as u16,
            CobbledDeepslateWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                22539u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            PolishedDeepslate => 22863u16,
            PolishedDeepslateStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                22864u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            PolishedDeepslateSlab {
                r#type,
                waterlogged,
            } => 22944u16 + *r#type as u16 * 2 + !waterlogged as u16,
            PolishedDeepslateWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                22950u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            DeepslateTiles => 23274u16,
            DeepslateTileStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                23275u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            DeepslateTileSlab {
                r#type,
                waterlogged,
            } => 23355u16 + *r#type as u16 * 2 + !waterlogged as u16,
            DeepslateTileWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                23361u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            DeepslateBricks => 23685u16,
            DeepslateBrickStairs {
                facing,
                half,
                shape,
                waterlogged,
            } => {
                23686u16
                    + *facing as u16 * 20
                    + *half as u16 * 10
                    + *shape as u16 * 2
                    + !waterlogged as u16
            }
            DeepslateBrickSlab {
                r#type,
                waterlogged,
            } => 23766u16 + *r#type as u16 * 2 + !waterlogged as u16,
            DeepslateBrickWall {
                north,
                east,
                south,
                west,
                up,
                waterlogged,
            } => {
                23772u16
                    + *east as u16 * 32
                    + *north as u16 * 16
                    + *south as u16 * 8
                    + !up as u16 * 4
                    + !waterlogged as u16 * 2
                    + *west as u16
            }
            ChiseledDeepslate => 24096u16,
            CrackedDeepslateBricks => 24097u16,
            CrackedDeepslateTiles => 24098u16,
            InfestedDeepslate { axis } => 24099u16 + *axis as u16,
            SmoothBasalt => 24102u16,
            RawIronBlock => 24103u16,
            RawCopperBlock => 24104u16,
            RawGoldBlock => 24105u16,
            PottedAzaleaBush => 24106u16,
            PottedFloweringAzaleaBush => 24107u16,
            OchreFroglight { axis } => 24108u16 + *axis as u16,
            VerdantFroglight { axis } => 24111u16 + *axis as u16,
            PearlescentFroglight { axis } => 24114u16 + *axis as u16,
            Frogspawn => 24117u16,
            ReinforcedDeepslate => 24118u16,
            DecoratedPot {
                facing,
                cracked,
                waterlogged,
            } => 24119u16 + !cracked as u16 * 8 + *facing as u16 * 2 + !waterlogged as u16,
        }
    }
}
