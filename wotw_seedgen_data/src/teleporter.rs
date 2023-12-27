use crate::{uber_identifier::teleporter, UberIdentifier};
#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[repr(u8)]
pub enum Teleporter {
    Inkwater = 16,
    Den = 1,
    Hollow = 5,
    Glades = 17,
    Wellspring = 3,
    Burrows = 0,
    WoodsEntrance = 7,
    WoodsExit = 8,
    Reach = 4,
    Depths = 6,
    CentralLuma = 2,
    LumaBoss = 13,
    FeedingGrounds = 9,
    CentralWastes = 10,
    OuterRuins = 11,
    InnerRuins = 14,
    Willow = 12,
    Shriek = 15,
}
impl Teleporter {
    pub const fn uber_identifier(self) -> UberIdentifier {
        match self {
            Teleporter::Inkwater => teleporter::INKWATER,
            Teleporter::Den => teleporter::DEN,
            Teleporter::Hollow => teleporter::HOLLOW,
            Teleporter::Glades => teleporter::GLADES,
            Teleporter::Wellspring => teleporter::WELLSPRING,
            Teleporter::Burrows => teleporter::BURROWS,
            Teleporter::WoodsEntrance => teleporter::WOODS_ENTRANCE,
            Teleporter::WoodsExit => teleporter::WOODS_EXIT,
            Teleporter::Reach => teleporter::REACH,
            Teleporter::Depths => teleporter::DEPTHS,
            Teleporter::CentralLuma => teleporter::CENTRAL_LUMA,
            Teleporter::LumaBoss => teleporter::LUMA_BOSS,
            Teleporter::FeedingGrounds => teleporter::FEEDING_GROUNDS,
            Teleporter::CentralWastes => teleporter::CENTRAL_WASTES,
            Teleporter::OuterRuins => teleporter::OUTER_RUINS,
            Teleporter::InnerRuins => teleporter::INNER_RUINS,
            Teleporter::Willow => teleporter::WILLOW,
            Teleporter::Shriek => teleporter::SHRIEK,
        }
    }
    pub const fn from_uber_identifier(uber_identifier: UberIdentifier) -> Option<Self> {
        match uber_identifier {
            teleporter::INKWATER => Some(Teleporter::Inkwater),
            teleporter::DEN => Some(Teleporter::Den),
            teleporter::HOLLOW => Some(Teleporter::Hollow),
            teleporter::GLADES => Some(Teleporter::Glades),
            teleporter::WELLSPRING => Some(Teleporter::Wellspring),
            teleporter::BURROWS => Some(Teleporter::Burrows),
            teleporter::WOODS_ENTRANCE => Some(Teleporter::WoodsEntrance),
            teleporter::WOODS_EXIT => Some(Teleporter::WoodsExit),
            teleporter::REACH => Some(Teleporter::Reach),
            teleporter::DEPTHS => Some(Teleporter::Depths),
            teleporter::CENTRAL_LUMA => Some(Teleporter::CentralLuma),
            teleporter::LUMA_BOSS => Some(Teleporter::LumaBoss),
            teleporter::FEEDING_GROUNDS => Some(Teleporter::FeedingGrounds),
            teleporter::CENTRAL_WASTES => Some(Teleporter::CentralWastes),
            teleporter::OUTER_RUINS => Some(Teleporter::OuterRuins),
            teleporter::INNER_RUINS => Some(Teleporter::InnerRuins),
            teleporter::WILLOW => Some(Teleporter::Willow),
            teleporter::SHRIEK => Some(Teleporter::Shriek),
            _ => None,
        }
    }
}
