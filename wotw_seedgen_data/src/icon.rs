#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

/// Icons used in the Opher shop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum OpherIcon {
    Sentry = 0,
    RapidSentry = 1,
    Hammer = 2,
    HammerShockwave = 3,
    Shuriken = 4,
    StaticShuriken = 5,
    Spear = 6,
    ExplodingSpear = 7,
    Blaze = 8,
    ChargeBlaze = 9,
    WaterBreath = 10,
    FastTravel = 11,
}
/// Icons used in the Lupo shop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum LupoIcon {
    EnergyFragmentsMap = 0,
    HealthFragmentsMap = 1,
    ShardsMap = 2,
}
/// Icons used in the Grom shop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum GromIcon {
    RepairTheSpiritWell = 0,
    DwellingRepairs = 1, // TODO ensure consistent names
    RoofsOverHeads = 2,
    OnwardsAndUpwards = 3,
    ClearTheCaveEntrance = 4,
    ThornySituation = 5,
    TheGorlekTouch = 6,
}
/// Icons used in the Tuley shop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum TuleyIcon {
    SelaFlowers = 0,
    StickyGrass = 1,
    Lightcatchers = 2,
    BlueMoon = 3,
    SpringPlants = 4,
    TheLastSeed = 5,
}
// should mirror https://github.com/ori-community/wotw-rando-client/blob/dev/projects/Core/enums/map_icon.h
/// Icons used in the map
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum MapIcon {
    Keystone = 0,
    Mapstone = 1,
    BreakableWall = 2,
    BreakableWallBroken = 3,
    StompableFloor = 4,
    StompableFloorBroken = 5,
    EnergyGateTwo = 6,
    EnergyGateOpen = 7,
    KeystoneDoorFour = 8,
    KeystoneDoorOpen = 9,
    AbilityPedestal = 10,
    HealthUpgrade = 11,
    EnergyUpgrade = 12,
    SavePedestal = 13,
    AbilityPoint = 14,
    KeystoneDoorTwo = 15,
    Invisible = 16,
    Experience = 17,
    MapstonePickup = 18,
    EnergyGateTwelve = 19,
    EnergyGateTen = 20,
    EnergyGateEight = 21,
    EnergyGateSix = 22,
    EnergyGateFour = 23,
    SpiritShard = 24,
    NPC = 25,
    QuestItem = 26,
    ShardSlotUpgrade = 27,
    Teleporter = 28,
    Ore = 29,
    QuestStart = 30,
    QuestEnd = 31,
    RaceStart = 32,
    HealthFragment = 33,
    EnergyFragment = 34,
    Seed = 35,
    RaceEnd = 36,
    Eyestone = 37,
    WatermillDoor = 40,
    TempleDoor = 41,
    SmallDoor = 42,
    Shrine = 43,
    Loremaster = 50,
    Weaponmaster = 51,
    Gardener = 52,
    Mapmaker = 53,
    Shardtrader = 54,
    Wanderer = 55,
    Treekeeper = 56,
    Builder = 57,
    Kwolok = 58,
    Statistician = 59,
    CreepHeart = 60,
    Miner = 61,
    Spiderling = 62,
    Moki = 63,
    MokiBrave = 64,
    MokiAdventurer = 65,
    MokiArtist = 66,
    MokiDarkness = 67,
    MokiFashionable = 68,
    MokiFisherman = 69,
    MokiFrozen = 70,
    MokiKwolokAmulet = 71,
    MokiSpyglass = 72,
    Ku = 73,
    IceFisher = 74,
    Siira = 75,
    // Rando Icons
    SavePedestalInactive = 76,
    RaceStartUnfinished = 77,
    CleanWater = 100,
    BonusItem = 101,
    LaunchFragment = 102,
    PurpleFloor = 103,
    PurpleWall = 104,
    YellowWall = 105,
    OneWayWallLeft = 106,
    OneWayWallRight = 107,
    IceWall = 108,
    IceFloor = 109,
    VerticalDoor = 110,
    HorizontalDoor = 111,
    Lever = 112,
}
