use crate::UberIdentifier;
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
            Teleporter::Inkwater => UberIdentifier::new(21786, 10185),
            Teleporter::Den => UberIdentifier::new(11666, 61594),
            Teleporter::Hollow => UberIdentifier::new(937, 26601),
            Teleporter::Glades => UberIdentifier::new(42178, 42096),
            Teleporter::Wellspring => UberIdentifier::new(53632, 18181),
            Teleporter::Burrows => UberIdentifier::new(24922, 42531),
            Teleporter::WoodsEntrance => UberIdentifier::new(58674, 7071),
            Teleporter::WoodsExit => UberIdentifier::new(58674, 1965),
            Teleporter::Reach => UberIdentifier::new(28895, 54235),
            Teleporter::Depths => UberIdentifier::new(18793, 38871),
            Teleporter::CentralLuma => UberIdentifier::new(945, 58183),
            Teleporter::LumaBoss => UberIdentifier::new(945, 1370),
            Teleporter::FeedingGrounds => UberIdentifier::new(58674, 10029),
            Teleporter::CentralWastes => UberIdentifier::new(20120, 49994),
            Teleporter::OuterRuins => UberIdentifier::new(20120, 41398),
            Teleporter::InnerRuins => UberIdentifier::new(10289, 4928),
            Teleporter::Willow => UberIdentifier::new(16155, 41465),
            Teleporter::Shriek => UberIdentifier::new(16155, 50867),
        }
    }
    pub const fn from_uber_identifier(uber_identifier: UberIdentifier) -> Option<Self> {
        match uber_identifier {
            UberIdentifier {
                group: 21786,
                member: 10185,
            } => Some(Teleporter::Inkwater),
            UberIdentifier {
                group: 11666,
                member: 61594,
            } => Some(Teleporter::Den),
            UberIdentifier {
                group: 937,
                member: 26601,
            } => Some(Teleporter::Hollow),
            UberIdentifier {
                group: 42178,
                member: 42096,
            } => Some(Teleporter::Glades),
            UberIdentifier {
                group: 53632,
                member: 18181,
            } => Some(Teleporter::Wellspring),
            UberIdentifier {
                group: 24922,
                member: 42531,
            } => Some(Teleporter::Burrows),
            UberIdentifier {
                group: 58674,
                member: 7071,
            } => Some(Teleporter::WoodsEntrance),
            UberIdentifier {
                group: 58674,
                member: 1965,
            } => Some(Teleporter::WoodsExit),
            UberIdentifier {
                group: 28895,
                member: 54235,
            } => Some(Teleporter::Reach),
            UberIdentifier {
                group: 18793,
                member: 38871,
            } => Some(Teleporter::Depths),
            UberIdentifier {
                group: 945,
                member: 58183,
            } => Some(Teleporter::CentralLuma),
            UberIdentifier {
                group: 945,
                member: 1370,
            } => Some(Teleporter::LumaBoss),
            UberIdentifier {
                group: 58674,
                member: 10029,
            } => Some(Teleporter::FeedingGrounds),
            UberIdentifier {
                group: 20120,
                member: 49994,
            } => Some(Teleporter::CentralWastes),
            UberIdentifier {
                group: 20120,
                member: 41398,
            } => Some(Teleporter::OuterRuins),
            UberIdentifier {
                group: 10289,
                member: 4928,
            } => Some(Teleporter::InnerRuins),
            UberIdentifier {
                group: 16155,
                member: 41465,
            } => Some(Teleporter::Willow),
            UberIdentifier {
                group: 16155,
                member: 50867,
            } => Some(Teleporter::Shriek),
            _ => None,
        }
    }
}
