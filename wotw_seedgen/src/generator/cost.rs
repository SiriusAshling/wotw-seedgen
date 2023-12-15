use std::collections::HashSet;
use wotw_seedgen_data::{Resource, Shard, Skill, Teleporter};
use wotw_seedgen_seed_language::output::{Action, Command, CommonItem};

use crate::inventory::Inventory;

pub trait Cost {
    fn cost(&self) -> usize;
}
impl Cost for Resource {
    fn cost(&self) -> usize {
        match self {
            Resource::GorlekOre => 20,
            Resource::EnergyFragment | Resource::HealthFragment => 120,
            Resource::Keystone => 320,
            Resource::ShardSlot => 480,
        }
    }
}
impl Cost for Skill {
    fn cost(&self) -> usize {
        match self {
            Skill::Regenerate | Skill::WaterBreath => 200, // Quality-of-Life Skills
            Skill::WallJump | Skill::Dash // Essential Movement
            | Skill::Flap // Counteracting a bias because Flap unlocks rather little
             => 1200,
            Skill::Glide | Skill::Grapple => 1400,         // Feel-Good Finds
            Skill::Sword | Skill::Hammer | Skill::Bow | Skill::Shuriken => 1600, // Basic Weapons
            Skill::Burrow | Skill::WaterDash | Skill::Grenade | Skill::Flash => 1800, // Key Skills
            Skill::DoubleJump => 2000, // Good to find, but this is already biased for by being powerful
            Skill::Blaze | Skill::Sentry => 2800, // Tedious Weapons
            Skill::Bash => 3000, // Counteracting a bias because Bash unlocks a lot
            Skill::Spear => 4000, // No
            Skill::Launch => 40000, // Absolutely Broken
            Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => 1000,
            Skill::SpiritFlame | Skill::Seir | Skill::BowCharge | Skill::Magnet | Skill::WeaponCharge => 0 // ?
        }
    }
}
impl Cost for Shard {
    fn cost(&self) -> usize {
        1000
    }
}
impl Cost for Teleporter {
    fn cost(&self) -> usize {
        match self {
            Teleporter::Inkwater => 30000,
            _ => 25000,
        }
    }
}
impl Cost for Inventory {
    fn cost(&self) -> usize {
        self.spirit_light.max(0) as usize
            + self
                .resources
                .iter()
                .map(|(resource, amount)| resource.cost() * (*amount).max(0) as usize)
                .sum::<usize>()
            + self.skills.cost()
            + self.shards.cost()
            + self.teleporters.cost()
            + self.clean_water as usize * 1800
    }
}
impl Cost for Action {
    fn cost(&self) -> usize {
        match self {
            Action::Command(command) => command.cost(),
            Action::Condition(condition) => condition.action.cost(),
            Action::Multi(multi) => multi.cost(),
        }
    }
}
impl Cost for Command {
    fn cost(&self) -> usize {
        match self {
            Command::Custom(common_item) => match common_item {
                CommonItem::SpiritLight(amount) => (*amount).max(0) as usize,
                CommonItem::Resource(resource) => resource.cost(),
                CommonItem::Skill(skill) => skill.cost(),
                CommonItem::Shard(shard) => shard.cost(),
                CommonItem::Teleporter(teleporter) => teleporter.cost(),
                _ => 0,
            },
            _ => 0,
        }
    }
}
impl<C: Cost> Cost for [C] {
    fn cost(&self) -> usize {
        self.iter().map(|c| c.cost()).sum()
    }
}
// TODO check if used
impl<C: Cost, S> Cost for HashSet<C, S> {
    fn cost(&self) -> usize {
        self.iter().map(|c| c.cost()).sum()
    }
}
