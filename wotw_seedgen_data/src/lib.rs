use decorum::R32;
#[cfg(feature = "try_from_number")]
use num_enum::TryFromPrimitive;
#[cfg(feature = "parse_display")]
use parse_display::{Display, FromStr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{self, Display};

/// Identifier for an UberState to store values in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UberIdentifier {
    pub group: i32,
    pub member: i32,
}
impl UberIdentifier {
    pub const SPIRIT_LIGHT: Self = UberIdentifier::new(6, 3);
    pub const CLEAN_WATER: Self = UberIdentifier::new(6, 2000);
    pub const HEALTH: UberIdentifier = UberIdentifier::new(15, 11);
    pub const MAX_HEALTH: UberIdentifier = Resource::HealthFragment.uber_identifier();
    pub const ENERGY: UberIdentifier = UberIdentifier::new(15, 13);
    pub const MAX_ENERGY: UberIdentifier = Resource::EnergyFragment.uber_identifier();
    pub const GORLEK_ORE: UberIdentifier = Resource::GorlekOre.uber_identifier();
    pub const KEYSTONE: UberIdentifier = Resource::Keystone.uber_identifier();
    pub const SHARD_SLOT: UberIdentifier = Resource::ShardSlot.uber_identifier();
    pub const BASH: UberIdentifier = Skill::Bash.uber_identifier();
    // pub const CHARGE_FLAME: UberIdentifier = Skill::ChargeFlame.uber_identifier();
    pub const WALL_JUMP: UberIdentifier = Skill::WallJump.uber_identifier();
    // pub const STOMP: UberIdentifier = Skill::Stomp.uber_identifier();
    pub const DOUBLE_JUMP: UberIdentifier = Skill::DoubleJump.uber_identifier();
    pub const LAUNCH: UberIdentifier = Skill::Launch.uber_identifier();
    // pub const MAGNET: UberIdentifier = Skill::Magnet.uber_identifier();
    // pub const ULTRA_MAGNET: UberIdentifier = Skill::UltraMagnet.uber_identifier();
    // pub const CLIMB: UberIdentifier = Skill::Climb.uber_identifier();
    pub const GLIDE: UberIdentifier = Skill::Glide.uber_identifier();
    pub const SPIRIT_FLAME: UberIdentifier = Skill::SpiritFlame.uber_identifier();
    // pub const RAPID_FLAME: UberIdentifier = Skill::RapidFlame.uber_identifier();
    // pub const SPLIT_FLAME_UPGRADE: UberIdentifier = Skill::SplitFlameUpgrade.uber_identifier();
    // pub const SOUL_EFFICIENCY: UberIdentifier = Skill::SoulEfficiency.uber_identifier();
    pub const WATER_BREATH: UberIdentifier = Skill::WaterBreath.uber_identifier();
    // pub const CHARGE_FLAME_BLAST: UberIdentifier = Skill::ChargeFlameBlast.uber_identifier();
    // pub const CHARGE_FLAME_BURN: UberIdentifier = Skill::ChargeFlameBurn.uber_identifier();
    // pub const DOUBLE_JUMP_UPGRADE: UberIdentifier = Skill::DoubleJumpUpgrade.uber_identifier();
    // pub const BASH_BUFF: UberIdentifier = Skill::BashBuff.uber_identifier();
    // pub const ULTRA_DEFENSE: UberIdentifier = Skill::UltraDefense.uber_identifier();
    // pub const HEALTH_EFFICIENCY: UberIdentifier = Skill::HealthEfficiency.uber_identifier();
    // pub const SENSE: UberIdentifier = Skill::Sense.uber_identifier();
    // pub const ULTRA_STOMP: UberIdentifier = Skill::UltraStomp.uber_identifier();
    // pub const SPARK_FLAME: UberIdentifier = Skill::SparkFlame.uber_identifier();
    // pub const QUICK_FLAME: UberIdentifier = Skill::QuickFlame.uber_identifier();
    // pub const MAP_MARKERS: UberIdentifier = Skill::MapMarkers.uber_identifier();
    // pub const ENERGY_EFFICIENCY: UberIdentifier = Skill::EnergyEfficiency.uber_identifier();
    // pub const HEALTH_MARKERS: UberIdentifier = Skill::HealthMarkers.uber_identifier();
    // pub const ENERGY_MARKERS: UberIdentifier = Skill::EnergyMarkers.uber_identifier();
    // pub const ABILITY_MARKERS: UberIdentifier = Skill::AbilityMarkers.uber_identifier();
    // pub const REKINDLE: UberIdentifier = Skill::Rekindle.uber_identifier();
    // pub const REGROUP: UberIdentifier = Skill::Regroup.uber_identifier();
    // pub const CHARGE_FLAME_EFFICIENCY: UberIdentifier = Skill::ChargeFlameEfficiency.uber_identifier();
    // pub const ULTRA_SOUL_FLAME: UberIdentifier = Skill::UltraSoulFlame.uber_identifier();
    // pub const SOUL_FLAME_EFFICIENCY: UberIdentifier = Skill::SoulFlameEfficiency.uber_identifier();
    // pub const CINDER_FLAME: UberIdentifier = Skill::CinderFlame.uber_identifier();
    // pub const ULTRA_SPLIT_FLAME: UberIdentifier = Skill::UltraSplitFlame.uber_identifier();
    // pub const DASH: UberIdentifier = Skill::Dash.uber_identifier();
    pub const GRENADE: UberIdentifier = Skill::Grenade.uber_identifier();
    // pub const GRENADE_UPGRADE: UberIdentifier = Skill::GrenadeUpgrade.uber_identifier();
    // pub const CHARGE_DASH: UberIdentifier = Skill::ChargeDash.uber_identifier();
    // pub const AIR_DASH: UberIdentifier = Skill::AirDash.uber_identifier();
    // pub const GRENADE_EFFICIENCY: UberIdentifier = Skill::GrenadeEfficiency.uber_identifier();
    // pub const BOUNCE: UberIdentifier = Skill::Bounce.uber_identifier();
    pub const GRAPPLE: UberIdentifier = Skill::Grapple.uber_identifier();
    // pub const SPIRIT_SLASH: UberIdentifier = Skill::SpiritSlash.uber_identifier();
    // pub const HEAVY_SPIRIT_SLASH: UberIdentifier = Skill::HeavySpiritSlash.uber_identifier();
    // pub const FIRE_BURST_SPELL: UberIdentifier = Skill::FireBurstSpell.uber_identifier();
    // pub const FIRE_WHIRL_SPELL: UberIdentifier = Skill::FireWhirlSpell.uber_identifier();
    pub const FLASH: UberIdentifier = Skill::Flash.uber_identifier();
    // pub const LOCK_ON_SPELL: UberIdentifier = Skill::LockOnSpell.uber_identifier();
    // pub const TIME_WARP_SPELL: UberIdentifier = Skill::TimeWarpSpell.uber_identifier();
    // pub const SHIELD_SPELL: UberIdentifier = Skill::ShieldSpell.uber_identifier();
    // pub const ENERGY_WALL_SPELL: UberIdentifier = Skill::EnergyWallSpell.uber_identifier();
    // pub const INVISIBILITY_SPELL: UberIdentifier = Skill::InvisibilitySpell.uber_identifier();
    // pub const TRAP_SPELL: UberIdentifier = Skill::TrapSpell.uber_identifier();
    // pub const WARP_SPELL: UberIdentifier = Skill::WarpSpell.uber_identifier();
    // pub const LIGHT_SPELL: UberIdentifier = Skill::LightSpell.uber_identifier();
    // pub const MIND_CONTROL_SPELL: UberIdentifier = Skill::MindControlSpell.uber_identifier();
    // pub const MIRAGE_SPELL: UberIdentifier = Skill::MirageSpell.uber_identifier();
    // pub const STICKY_MINE_SPELL: UberIdentifier = Skill::StickyMineSpell.uber_identifier();
    pub const SPEAR: UberIdentifier = Skill::Spear.uber_identifier();
    // pub const LIGHT_SPEAR_SPELL: UberIdentifier = Skill::LightSpearSpell.uber_identifier();
    // pub const LIFE_ABSORB_SPELL: UberIdentifier = Skill::LifeAbsorbSpell.uber_identifier();
    pub const REGENERATE: UberIdentifier = Skill::Regenerate.uber_identifier();
    // pub const CHARGE_SHOT_SPELL: UberIdentifier = Skill::ChargeShotSpell.uber_identifier();
    // pub const SPIRIT_SHARDS_SPELL: UberIdentifier = Skill::SpiritShardsSpell.uber_identifier();
    // pub const SPIRIT_SENTRY_SPELL: UberIdentifier = Skill::SpiritSentrySpell.uber_identifier();
    // pub const POWERSLIDE_SPELL: UberIdentifier = Skill::PowerslideSpell.uber_identifier();
    // pub const COUNTERSTRIKE_SPELL: UberIdentifier = Skill::CounterstrikeSpell.uber_identifier();
    // pub const EARTH_SHATTER_SPELL: UberIdentifier = Skill::EarthShatterSpell.uber_identifier();
    // pub const JUMP_SHOT_SPELL: UberIdentifier = Skill::JumpShotSpell.uber_identifier();
    // pub const ROUNDUP_LEASH_SPELL: UberIdentifier = Skill::RoundupLeashSpell.uber_identifier();
    // pub const BURROW_SPELL: UberIdentifier = Skill::BurrowSpell.uber_identifier();
    // pub const POWER_OF_FRIENDSHIP_SPELL: UberIdentifier = Skill::PowerOfFriendshipSpell.uber_identifier();
    // pub const LIGHTNING_SPELL: UberIdentifier = Skill::LightningSpell.uber_identifier();
    // pub const SPIRIT_FLARE_SPELL: UberIdentifier = Skill::SpiritFlareSpell.uber_identifier();
    // pub const ENTANGLING_ROOTS_SPELL: UberIdentifier = Skill::EntanglingRootsSpell.uber_identifier();
    // pub const MARK_OF_THE_WILDS_SPELL: UberIdentifier = Skill::MarkOfTheWildsSpell.uber_identifier();
    // pub const HOMING_MISSILE_SPELL: UberIdentifier = Skill::HomingMissileSpell.uber_identifier();
    // pub const SPIRIT_CRESCENT_SPELL: UberIdentifier = Skill::SpiritCrescentSpell.uber_identifier();
    // pub const MINE_SPELL: UberIdentifier = Skill::MineSpell.uber_identifier();
    // pub const PINNED: UberIdentifier = Skill::Pinned.uber_identifier();
    // pub const LEACHED: UberIdentifier = Skill::Leached.uber_identifier();
    pub const BOW: UberIdentifier = Skill::Bow.uber_identifier();
    pub const HAMMER: UberIdentifier = Skill::Hammer.uber_identifier();
    // pub const TORCH: UberIdentifier = Skill::Torch.uber_identifier();
    pub const SWORD: UberIdentifier = Skill::Sword.uber_identifier();
    pub const BURROW: UberIdentifier = Skill::Burrow.uber_identifier();
    pub const DASH: UberIdentifier = Skill::Dash.uber_identifier();
    // pub const LAUNCH: UberIdentifier = Skill::Launch.uber_identifier();
    pub const WATER_DASH: UberIdentifier = Skill::WaterDash.uber_identifier();
    // pub const TELEPORT_SPELL: UberIdentifier = Skill::TeleportSpell.uber_identifier();
    pub const SHURIKEN: UberIdentifier = Skill::Shuriken.uber_identifier();
    // pub const DRILL: UberIdentifier = Skill::Drill.uber_identifier();
    pub const SEIR: UberIdentifier = Skill::Seir.uber_identifier();
    pub const BOW_CHARGE: UberIdentifier = Skill::BowCharge.uber_identifier();
    // pub const SWORDSTAFF: UberIdentifier = Skill::Swordstaff.uber_identifier();
    // pub const CHAINSWORD: UberIdentifier = Skill::Chainsword.uber_identifier();
    pub const MAGNET_SKILL: UberIdentifier = Skill::Magnet.uber_identifier();
    // pub const SWORD_CHARGE: UberIdentifier = Skill::SwordCharge.uber_identifier();
    // pub const HAMMER_CHARGE: UberIdentifier = Skill::HammerCharge.uber_identifier();
    pub const BLAZE: UberIdentifier = Skill::Blaze.uber_identifier();
    pub const SENTRY: UberIdentifier = Skill::Sentry.uber_identifier();
    // pub const REGENERATE: UberIdentifier = Skill::Regenerate.uber_identifier();
    pub const FLAP: UberIdentifier = Skill::Flap.uber_identifier();
    pub const WEAPON_CHARGE: UberIdentifier = Skill::WeaponCharge.uber_identifier();
    pub const GLADES_ANCESTRAL_LIGHT: UberIdentifier =
        Skill::GladesAncestralLight.uber_identifier();
    pub const INKWATER_ANCESTRAL_LIGHT: UberIdentifier =
        Skill::InkwaterAncestralLight.uber_identifier();
    pub const OVERCHARGE: UberIdentifier = Shard::Overcharge.uber_identifier();
    pub const TRIPLE_JUMP: UberIdentifier = Shard::TripleJump.uber_identifier();
    pub const WINGCLIP: UberIdentifier = Shard::Wingclip.uber_identifier();
    pub const BOUNTY: UberIdentifier = Shard::Bounty.uber_identifier();
    pub const SWAP: UberIdentifier = Shard::Swap.uber_identifier();
    // pub const CRESCENT_SHOT_DEPRECATED: UberIdentifier = Shard::CrescentShotDeprecated.uber_identifier();
    // pub const PIERCE: UberIdentifier = Shard::Pierce.uber_identifier();
    pub const MAGNET: UberIdentifier = Shard::Magnet.uber_identifier();
    pub const SPLINTER: UberIdentifier = Shard::Splinter.uber_identifier();
    // pub const BLAZE_DEPRECATED: UberIdentifier = Shard::BlazeDeprecated.uber_identifier();
    // pub const FROST_DEPRECATED: UberIdentifier = Shard::FrostDeprecated.uber_identifier();
    // pub const LIFE_LEECH_DEPRECATED: UberIdentifier = Shard::LifeLeechDeprecated.uber_identifier();
    pub const RECKLESS: UberIdentifier = Shard::Reckless.uber_identifier();
    pub const QUICKSHOT: UberIdentifier = Shard::Quickshot.uber_identifier();
    // pub const EXPLOSIVE_DEPRECATED: UberIdentifier = Shard::ExplosiveDeprecated.uber_identifier();
    // pub const RICOCHET: UberIdentifier = Shard::Ricochet.uber_identifier();
    // pub const CLIMB_DEPRECATED: UberIdentifier = Shard::ClimbDeprecated.uber_identifier();
    pub const RESILIENCE: UberIdentifier = Shard::Resilience.uber_identifier();
    pub const SPIRIT_LIGHT_HARVEST: UberIdentifier = Shard::SpiritLightHarvest.uber_identifier();
    // pub const COMPASS_DEPRECATED: UberIdentifier = Shard::CompassDeprecated.uber_identifier();
    // pub const WATERBREATHING_DEPRECATED: UberIdentifier = Shard::WaterbreathingDeprecated.uber_identifier();
    pub const VITALITY: UberIdentifier = Shard::Vitality.uber_identifier();
    pub const LIFE_HARVEST: UberIdentifier = Shard::LifeHarvest.uber_identifier();
    // pub const SPIRIT_WELL_SHIELD_DEPRECATED: UberIdentifier = Shard::SpiritWellShieldDeprecated.uber_identifier();
    pub const ENERGY_HARVEST: UberIdentifier = Shard::EnergyHarvest.uber_identifier();
    pub const ENERGY_SHARD: UberIdentifier = Shard::Energy.uber_identifier();
    pub const LIFE_PACT: UberIdentifier = Shard::LifePact.uber_identifier();
    pub const LAST_STAND: UberIdentifier = Shard::LastStand.uber_identifier();
    // pub const HARVEST_OF_LIGHT_DEPRECATED: UberIdentifier = Shard::HarvestOfLightDeprecated.uber_identifier();
    pub const SENSE: UberIdentifier = Shard::Sense.uber_identifier();
    // pub const UNDERWATER_EFFICIENCY_DEPRECATED: UberIdentifier = Shard::UnderwaterEfficiencyDeprecated.uber_identifier();
    pub const ULTRA_BASH: UberIdentifier = Shard::UltraBash.uber_identifier();
    pub const ULTRA_GRAPPLE: UberIdentifier = Shard::UltraGrapple.uber_identifier();
    pub const OVERFLOW: UberIdentifier = Shard::Overflow.uber_identifier();
    pub const THORN: UberIdentifier = Shard::Thorn.uber_identifier();
    pub const CATALYST: UberIdentifier = Shard::Catalyst.uber_identifier();
    // pub const SUPRESSOR: UberIdentifier = Shard::Supressor.uber_identifier();
    pub const TURMOIL: UberIdentifier = Shard::Turmoil.uber_identifier();
    pub const STICKY: UberIdentifier = Shard::Sticky.uber_identifier();
    pub const FINESSE: UberIdentifier = Shard::Finesse.uber_identifier();
    pub const SPIRIT_SURGE: UberIdentifier = Shard::SpiritSurge.uber_identifier();
    // pub const OVERCHARGE_DEPRECATED: UberIdentifier = Shard::OverchargeDeprecated.uber_identifier();
    pub const LIFEFORCE: UberIdentifier = Shard::Lifeforce.uber_identifier();
    pub const DEFLECTOR: UberIdentifier = Shard::Deflector.uber_identifier();
    // pub const STINGER: UberIdentifier = Shard::Stinger.uber_identifier();
    pub const FRACTURE: UberIdentifier = Shard::Fracture.uber_identifier();
    pub const ARCING: UberIdentifier = Shard::Arcing.uber_identifier();
    pub const MARSH: UberIdentifier = Teleporter::Inkwater.uber_identifier();
    pub const DEN: UberIdentifier = Teleporter::Den.uber_identifier();
    pub const HOLLOW: UberIdentifier = Teleporter::Hollow.uber_identifier();
    pub const GLADES: UberIdentifier = Teleporter::Glades.uber_identifier();
    pub const WELLSPRING: UberIdentifier = Teleporter::Wellspring.uber_identifier();
    pub const BURROWS: UberIdentifier = Teleporter::Burrows.uber_identifier();
    pub const WEST_WOODS: UberIdentifier = Teleporter::WoodsEntrance.uber_identifier();
    pub const EAST_WOODS: UberIdentifier = Teleporter::WoodsExit.uber_identifier();
    pub const REACH: UberIdentifier = Teleporter::Reach.uber_identifier();
    pub const DEPTHS: UberIdentifier = Teleporter::Depths.uber_identifier();
    pub const EAST_LUMA: UberIdentifier = Teleporter::CentralLuma.uber_identifier();
    pub const WEST_LUMA: UberIdentifier = Teleporter::LumaBoss.uber_identifier();
    pub const FEEDING_GROUNDS: UberIdentifier = Teleporter::FeedingGrounds.uber_identifier();
    pub const EAST_WASTES: UberIdentifier = Teleporter::CentralWastes.uber_identifier();
    pub const OUTER_RUINS: UberIdentifier = Teleporter::OuterRuins.uber_identifier();
    pub const INNER_RUINS: UberIdentifier = Teleporter::InnerRuins.uber_identifier();
    pub const WILLOW: UberIdentifier = Teleporter::Willow.uber_identifier();
    pub const SHRIEK: UberIdentifier = Teleporter::Shriek.uber_identifier();
    pub const EXPLODING_SPEAR: UberIdentifier = WeaponUpgrade::ExplodingSpear.uber_identifier();
    pub const SHOCK_HAMMER: UberIdentifier = WeaponUpgrade::ShockHammer.uber_identifier();
    pub const STATIC_SHURIKEN: UberIdentifier = WeaponUpgrade::StaticShuriken.uber_identifier();
    pub const CHARGE_BLAZE: UberIdentifier = WeaponUpgrade::ChargeBlaze.uber_identifier();
    pub const RAPID_SENTRY: UberIdentifier = WeaponUpgrade::RapidSentry.uber_identifier();

    pub const fn new(group: i32, member: i32) -> Self {
        Self { group, member }
    }

    pub const fn is_shop(self) -> bool {
        matches!(self.group, 1 | 2 | 15)
    }
}
impl Display for UberIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}", self.group, self.member)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Position {
    pub x: R32,
    pub y: R32,
}
impl Position {
    pub fn new<F: Into<R32>>(x: F, y: F) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// TODO so why is spirit light not a resource now?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum Resource {
    HealthFragment = 0,
    EnergyFragment = 1,
    GorlekOre = 2,
    Keystone = 3,
    ShardSlot = 4,
}
impl Resource {
    pub const fn uber_identifier(self) -> UberIdentifier {
        UberIdentifier::new(6, self as i32 + 3000)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
// TODO rename to ability?
pub enum Skill {
    Bash = 0,
    // ChargeFlame = 2,
    WallJump = 3,
    // Stomp = 4,
    DoubleJump = 5,
    Launch = 8,
    // Magnet = 10,
    // UltraMagnet = 11,
    // Climb = 12,
    Glide = 14,
    SpiritFlame = 15,
    // RapidFlame = 17,
    // SplitFlameUpgrade = 18,
    // SoulEfficiency = 22,
    WaterBreath = 23,
    // ChargeFlameBlast = 27,
    // ChargeFlameBurn = 28,
    // DoubleJumpUpgrade = 29,
    // BashBuff = 30,
    // UltraDefense = 31,
    // HealthEfficiency = 32,
    // Sense = 33,
    // UltraStomp = 34,
    // SparkFlame = 36,
    // QuickFlame = 37,
    // MapMarkers = 38,
    // EnergyEfficiency = 39,
    // HealthMarkers = 40,
    // EnergyMarkers = 41,
    // AbilityMarkers = 42,
    // Rekindle = 43,
    // Regroup = 44,
    // ChargeFlameEfficiency = 45,
    // UltraSoulFlame = 46,
    // SoulFlameEfficiency = 47,
    // CinderFlame = 48,
    // UltraSplitFlame = 49,
    // Dash = 50,
    Grenade = 51,
    // GrenadeUpgrade = 52,
    // ChargeDash = 53,
    // AirDash = 54,
    // GrenadeEfficiency = 55,
    // Bounce = 56,
    Grapple = 57,
    // SpiritSlash = 58,
    // HeavySpiritSlash = 59,
    // FireBurstSpell = 60,
    // FireWhirlSpell = 61,
    Flash = 62,
    // LockOnSpell = 63,
    // TimeWarpSpell = 64,
    // ShieldSpell = 65,
    // EnergyWallSpell = 66,
    // InvisibilitySpell = 67,
    // TrapSpell = 68,
    // WarpSpell = 69,
    // LightSpell = 70,
    // MindControlSpell = 71,
    // MirageSpell = 72,
    // StickyMineSpell = 73,
    Spear = 74,
    // LightSpearSpell = 75,
    // LifeAbsorbSpell = 76,
    Regenerate = 77,
    // ChargeShotSpell = 78,
    // SpiritShardsSpell = 79,
    // SpiritSentrySpell = 80,
    // PowerslideSpell = 81,
    // CounterstrikeSpell = 82,
    // EarthShatterSpell = 83,
    // JumpShotSpell = 84,
    // RoundupLeashSpell = 85,
    // BurrowSpell = 86,
    // PowerOfFriendshipSpell = 87,
    // LightningSpell = 88,
    // SpiritFlareSpell = 89,
    // EntanglingRootsSpell = 90,
    // MarkOfTheWildsSpell = 91,
    // HomingMissileSpell = 92,
    // SpiritCrescentSpell = 93,
    // MineSpell = 94,
    // Pinned = 95,
    // Leached = 96,
    Bow = 97,
    Hammer = 98,
    // Torch = 99,
    Sword = 100,
    Burrow = 101,
    Dash = 102,
    // Launch = 103,
    WaterDash = 104,
    // TeleportSpell = 105,
    Shuriken = 106,
    // Drill = 107,
    Seir = 108,
    BowCharge = 109,
    // Swordstaff = 110,
    // Chainsword = 111,
    Magnet = 112,
    // SwordCharge = 113, // TODO add an uberstate?
    // HammerCharge = 114, // TODO add an uberstate?
    Blaze = 115,
    Sentry = 116,
    // Regenerate = 117,
    Flap = 118,
    WeaponCharge = 119, // TODO what is this and why does it have an uberstate
    GladesAncestralLight = 120,
    InkwaterAncestralLight = 121,
}
impl Skill {
    pub const fn uber_identifier(self) -> UberIdentifier {
        UberIdentifier::new(6, self as i32 + 1000)
    }
    pub const fn energy_cost(self) -> f32 {
        match self {
            Skill::Bow => 0.25,
            Skill::Shuriken => 0.5,
            Skill::Grenade | Skill::Flash | Skill::Regenerate | Skill::Blaze | Skill::Sentry => 1.0,
            Skill::Spear => 2.0,
            _ => 0.0,
        }
    }
    pub const fn damage(self, charge_grenade: bool) -> f32 {
        match self {
            Skill::Bow | Skill::Sword => 4.0,
            Skill::Launch => 5.0,
            Skill::Hammer | Skill::Flash => 12.0,
            Skill::Shuriken => 7.0,
            Skill::Grenade => {
                if charge_grenade {
                    8.0
                } else {
                    4.0
                }
            }
            Skill::Spear => 20.0,
            Skill::Blaze => 3.0,
            Skill::Sentry => 8.8,
            _ => 0.0,
        }
    }
    pub const fn burn_damage(self) -> f32 {
        match self {
            Skill::Grenade => 9.0,
            Skill::Blaze => 10.8,
            _ => 0.0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum Shard {
    Overcharge = 1,
    TripleJump = 2,
    Wingclip = 3,
    Bounty = 4,
    Swap = 5,
    // CrescentShotDeprecated = 6,
    // Pierce = 7,
    Magnet = 8,
    Splinter = 9,
    // BlazeDeprecated = 10,
    // FrostDeprecated = 11,
    // LifeLeechDeprecated = 12,
    Reckless = 13,
    Quickshot = 14,
    // ExplosiveDeprecated = 15,
    // Ricochet = 16,
    // ClimbDeprecated = 17,
    Resilience = 18,
    SpiritLightHarvest = 19,
    // CompassDeprecated = 20,
    // WaterbreathingDeprecated = 21,
    Vitality = 22,
    LifeHarvest = 23,
    // SpiritWellShieldDeprecated = 24,
    EnergyHarvest = 25,
    Energy = 26,
    LifePact = 27,
    LastStand = 28,
    // HarvestOfLightDeprecated = 29,
    Sense = 30,
    // UnderwaterEfficiencyDeprecated = 31,
    UltraBash = 32,
    UltraGrapple = 33,
    Overflow = 34,
    Thorn = 35,
    Catalyst = 36,
    // Supressor = 37,
    Turmoil = 38,
    Sticky = 39,
    Finesse = 40,
    SpiritSurge = 41,
    // OverchargeDeprecated = 42,
    Lifeforce = 43,
    Deflector = 44,
    // Stinger = 45,
    Fracture = 46,
    Arcing = 47,
}
impl Shard {
    pub const fn uber_identifier(self) -> UberIdentifier {
        UberIdentifier::new(6, self as i32 + 4000)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
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
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum WeaponUpgrade {
    ExplodingSpear = 0,
    ShockHammer = 1,
    StaticShuriken = 2,
    ChargeBlaze = 3,
    RapidSentry = 4,
}
impl WeaponUpgrade {
    pub const fn uber_identifier(self) -> UberIdentifier {
        UberIdentifier::new(6, self as i32 + 5000)
    }
}
// TODO parse-display has garbage error messages, lets make this our own again
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u16)]
pub enum Equipment {
    Hammer = 1000,
    Bow = 1001,
    Sword = 1002,
    Torch = 1003,
    Swordstaff = 1004,
    Chainsword = 1005,
    Shot = 2000,
    HomingMissiles = 2001,
    Wave = 2002,
    Whirl = 2003,
    Glow = 2004,
    LockOn = 2005,
    Shield = 2006,
    Invisibility = 2007,
    LifeAbsorb = 2008,
    Shards = 2009,
    Grenade = 2010,
    Sentry = 2011,
    Spear = 2012,
    Regenerate = 2013,
    Teleport = 2014,
    Shuriken = 2015,
    Blaze = 2016,
    Turret = 2017,
    Sein = 2018,
    Launch = 2019,
    Bash = 3000,
    Grapple = 3001,
    Burrow = 3002,
    Drill = 3003,
    DoubleJump = 3004,
    Flap = 3005,
    Dash = 4000,
    Bounce = 4001,
    Glide = 4002,
    ChargeJump = 4003,
    WaterDash = 4004,
    Climb = 4005,
    WeaponCharge = 4006,
    DamageUpgradeA = 4007,
    DamageUpgradeB = 4008,
    WaterBreath = 4009,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum Zone {
    Marsh = 0,
    Hollow = 1,
    Glades = 2,
    Wellspring = 3,
    Woods = 7,
    Reach = 6,
    Depths = 8,
    Pools = 4,
    Wastes = 9,
    Ruins = 10,
    Willow = 11,
    Burrows = 5,
    Spawn = 14,
    Shop = 12,
    Void = 13,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum OpherIcon {
    Sentry = 0,
    SentryUpgrade = 1,
    Hammer = 2,
    HammerUpgrade = 3,
    Shuriken = 4,
    ShurikenUpgrade = 5,
    Spear = 6,
    SpearUpgrade = 7,
    Blaze = 8,
    BlazeUpgrade = 9,
    WaterBreath = 10,
    FastTravel = 11,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum LupoIcon {
    EnergyFragmentsMap = 0,
    HealthFragmentsMap = 1,
    ShardsMap = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum GromIcon {
    RepairTheSpiritWell = 0,
    DwellingRepairs = 1,
    RoofsOverHeads = 2,
    OnwardsAndUpwards = 3,
    ClearTheCaveEntrance = 4,
    ThornySituation = 5,
    TheGorlekTouch = 6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum EquipSlot {
    Ability1 = 0,
    Ability2 = 1,
    Ability3 = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum WheelItemPosition {
    Top = 0,
    TopRight = 1,
    RightTop = 2,
    Right = 3,
    RightBottom = 4,
    BottomRight = 5,
    Bottom = 6,
    BottomLeft = 7,
    LeftBottom = 8,
    Left = 9,
    LeftTop = 10,
    TopLeft = 11,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize_repr, Serialize_repr))]
#[cfg_attr(feature = "parse_display", derive(FromStr, Display))]
#[cfg_attr(feature = "try_from_number", derive(TryFromPrimitive))]
#[repr(u8)]
pub enum WheelBind {
    All = 0,
    Ability1 = 1,
    Ability2 = 2,
    Ability3 = 3,
}
