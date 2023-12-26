#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Identifier for an UberState to store values in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UberIdentifier {
    pub group: i32,
    pub member: i32,
}
impl UberIdentifier {
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

pub mod uber_identifier {
    use crate::UberIdentifier;

    pub const SPIRIT_LIGHT: UberIdentifier = UberIdentifier::new(5, 0);
    pub const GORLEK_ORE: UberIdentifier = UberIdentifier::new(5, 1);
    pub const KEYSTONES: UberIdentifier = UberIdentifier::new(5, 2);
    pub const SHARD_SLOTS: UberIdentifier = UberIdentifier::new(5, 3); // TODO client needs to add this
    pub const CLEAN_WATER: UberIdentifier = UberIdentifier::new(6, 2000);
    pub const MAX_HEALTH: UberIdentifier = UberIdentifier::new(5, 10);
    pub const HEALTH: UberIdentifier = UberIdentifier::new(5, 11);
    pub const MAX_ENERGY: UberIdentifier = UberIdentifier::new(5, 12);
    pub const ENERGY: UberIdentifier = UberIdentifier::new(5, 13);

    pub mod skill {
        use crate::{Skill, UberIdentifier};

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
    }

    pub mod shard {
        use crate::{Shard, UberIdentifier};

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
        pub const SPIRIT_LIGHT_HARVEST: UberIdentifier =
            Shard::SpiritLightHarvest.uber_identifier();
        // pub const COMPASS_DEPRECATED: UberIdentifier = Shard::CompassDeprecated.uber_identifier();
        // pub const WATERBREATHING_DEPRECATED: UberIdentifier = Shard::WaterbreathingDeprecated.uber_identifier();
        pub const VITALITY: UberIdentifier = Shard::Vitality.uber_identifier();
        pub const LIFE_HARVEST: UberIdentifier = Shard::LifeHarvest.uber_identifier();
        // pub const SPIRIT_WELL_SHIELD_DEPRECATED: UberIdentifier = Shard::SpiritWellShieldDeprecated.uber_identifier();
        pub const ENERGY_HARVEST: UberIdentifier = Shard::EnergyHarvest.uber_identifier();
        pub const ENERGY: UberIdentifier = Shard::Energy.uber_identifier();
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
    }

    pub mod teleporter {
        use crate::{Teleporter, UberIdentifier};

        pub const INKWATER: UberIdentifier = Teleporter::Inkwater.uber_identifier();
        pub const DEN: UberIdentifier = Teleporter::Den.uber_identifier();
        pub const HOLLOW: UberIdentifier = Teleporter::Hollow.uber_identifier();
        pub const GLADES: UberIdentifier = Teleporter::Glades.uber_identifier();
        pub const WELLSPRING: UberIdentifier = Teleporter::Wellspring.uber_identifier();
        pub const BURROWS: UberIdentifier = Teleporter::Burrows.uber_identifier();
        pub const WOODS_ENTRANCE: UberIdentifier = Teleporter::WoodsEntrance.uber_identifier();
        pub const WOODS_EXIT: UberIdentifier = Teleporter::WoodsExit.uber_identifier();
        pub const REACH: UberIdentifier = Teleporter::Reach.uber_identifier();
        pub const DEPTHS: UberIdentifier = Teleporter::Depths.uber_identifier();
        pub const CENTRAL_LUMA: UberIdentifier = Teleporter::CentralLuma.uber_identifier();
        pub const LUMA_BOSS: UberIdentifier = Teleporter::LumaBoss.uber_identifier();
        pub const FEEDING_GROUNDS: UberIdentifier = Teleporter::FeedingGrounds.uber_identifier();
        pub const CENTRAL_WASTES: UberIdentifier = Teleporter::CentralWastes.uber_identifier();
        pub const OUTER_RUINS: UberIdentifier = Teleporter::OuterRuins.uber_identifier();
        pub const INNER_RUINS: UberIdentifier = Teleporter::InnerRuins.uber_identifier();
        pub const WILLOW: UberIdentifier = Teleporter::Willow.uber_identifier();
        pub const SHRIEK: UberIdentifier = Teleporter::Shriek.uber_identifier();
    }

    pub mod weapon_upgrade {
        use crate::{UberIdentifier, WeaponUpgrade};

        pub const EXPLODING_SPEAR: UberIdentifier = WeaponUpgrade::ExplodingSpear.uber_identifier();
        pub const SHOCK_HAMMER: UberIdentifier = WeaponUpgrade::HammerShockwave.uber_identifier();
        pub const STATIC_SHURIKEN: UberIdentifier = WeaponUpgrade::StaticShuriken.uber_identifier();
        pub const CHARGE_BLAZE: UberIdentifier = WeaponUpgrade::ChargeBlaze.uber_identifier();
        pub const RAPID_SENTRY: UberIdentifier = WeaponUpgrade::RapidSentry.uber_identifier();
    }
}