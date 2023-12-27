use crate::{uber_identifier::weapon_upgrade, UberIdentifier};
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
        match self {
            WeaponUpgrade::ExplodingSpear => weapon_upgrade::EXPLODING_SPEAR,
            WeaponUpgrade::HammerShockwave => weapon_upgrade::SHOCK_HAMMER,
            WeaponUpgrade::StaticShuriken => weapon_upgrade::STATIC_SHURIKEN,
            WeaponUpgrade::ChargeBlaze => weapon_upgrade::CHARGE_BLAZE,
            WeaponUpgrade::RapidSentry => weapon_upgrade::RAPID_SENTRY,
        }
    }
    #[cfg(feature = "strum")]
    pub const fn from_uber_identifier(uber_identifier: UberIdentifier) -> Option<Self> {
        match uber_identifier {
            weapon_upgrade::EXPLODING_SPEAR => Some(WeaponUpgrade::ExplodingSpear),
            weapon_upgrade::SHOCK_HAMMER => Some(WeaponUpgrade::HammerShockwave),
            weapon_upgrade::STATIC_SHURIKEN => Some(WeaponUpgrade::StaticShuriken),
            weapon_upgrade::CHARGE_BLAZE => Some(WeaponUpgrade::ChargeBlaze),
            weapon_upgrade::RAPID_SENTRY => Some(WeaponUpgrade::RapidSentry),
            _ => None,
        }
    }
}
