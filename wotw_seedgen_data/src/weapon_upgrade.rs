use crate::UberIdentifier;
#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(feature = "strum")]
use strum::{Display, EnumString, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "strum", derive(Display, EnumString, FromRepr))]
#[repr(u8)]
pub enum WeaponUpgrade {
    ExplodingSpear = 0,
    HammerShockwave = 1,
    StaticShuriken = 2,
    ChargeBlaze = 3,
    RapidSentry = 4,
}
impl WeaponUpgrade {
    pub const fn uber_identifier(self) -> UberIdentifier {
        UberIdentifier::new(6, self as i32 + 5000) // TODO client has to add these
    }
    #[cfg(feature = "strum")]
    pub const fn from_uber_identifier(uber_identifier: UberIdentifier) -> Option<Self> {
        match uber_identifier {
            UberIdentifier {
                group: 6,
                member: id @ 5000..=5004,
            } => Self::from_repr((id - 5000) as u8),
            _ => None,
        }
    }
}
