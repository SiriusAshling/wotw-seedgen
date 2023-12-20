#[cfg(test)]
mod tests;

use crate::{
    logical_difficulty,
    orbs::{self, OrbVariants, Orbs},
};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::{
    fmt::{self, Display},
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use wotw_seedgen_data::{Resource, Shard, Skill, Teleporter, WeaponUpgrade};
use wotw_seedgen_logic_language::output::RefillValue;
use wotw_seedgen_settings::{Difficulty, WorldSettings};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub spirit_light: i32,
    pub resources: FxHashMap<Resource, i32>,
    pub skills: FxHashSet<Skill>,
    pub shards: FxHashSet<Shard>,
    pub teleporters: FxHashSet<Teleporter>,
    pub clean_water: bool,
    pub weapon_upgrades: FxHashSet<WeaponUpgrade>,
}
impl Inventory {
    pub fn spawn() -> Self {
        Inventory {
            resources: [
                (Resource::HealthFragment, 6),
                (Resource::EnergyFragment, 6),
                (Resource::ShardSlot, 3),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        }
    }

    pub fn get_resource(&self, resource: Resource) -> i32 {
        self.resources.get(&resource).copied().unwrap_or_default()
    }
    pub fn add_resource(&mut self, resource: Resource, amount: i32) {
        *self.resources.entry(resource).or_default() += amount;
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn item_count(&self) -> usize {
        // Note that requirement::solutions has logic based on this formula (check_slot_limits)
        ((self.spirit_light + 39) / 40).max(0) as usize // this will usually demand more than necessary, but with the placeholder system that shouldn't be a problem
        + self.world_item_count()
    }
    pub fn world_item_count(&self) -> usize {
        self.resources
            .values()
            .map(|&amount| amount.max(0) as usize)
            .sum::<usize>()
            + self.skills.len()
            + self.shards.len()
            + self.teleporters.len()
            + self.clean_water as usize
    }
    pub fn is_empty(&self) -> bool {
        self.spirit_light == 0 && self.world_item_count() == 0
    }

    pub fn contains(&self, other: &Inventory) -> bool {
        self.spirit_light >= other.spirit_light
            && self
                .resources
                .iter()
                .all(|(resource, amount)| *amount >= other.get_resource(*resource))
            && other.skills.is_subset(&self.skills)
            && other.shards.is_subset(&self.shards)
            && other.teleporters.is_subset(&self.teleporters)
            && (!self.clean_water || other.clean_water)
    }

    pub fn max_health(&self, difficulty: Difficulty) -> f32 {
        let mut health = (self.get_resource(Resource::HealthFragment) * 5) as f32;
        if difficulty >= logical_difficulty::VITALITY && self.shards.contains(&Shard::Vitality) {
            health += 10.0;
        }
        health
    }
    pub fn max_energy(&self, difficulty: Difficulty) -> f32 {
        let mut energy = self.get_resource(Resource::EnergyFragment) as f32 * 0.5;
        if difficulty >= logical_difficulty::ENERGY_SHARD && self.shards.contains(&Shard::Energy) {
            energy += 1.0;
        }
        energy
    }
    pub fn max_orbs(&self, difficulty: Difficulty) -> Orbs {
        Orbs {
            energy: self.max_energy(difficulty),
            health: self.max_health(difficulty),
        }
    }
    pub fn cap_orbs(&self, orbs: &mut Orbs, checkpoint: bool, difficulty: Difficulty) {
        // checkpoints don't refill health given by the Vitality shard
        let max_health = if checkpoint {
            (self.get_resource(Resource::HealthFragment) * 5) as f32
        } else {
            self.max_health(difficulty)
        };
        // (but they do refill energy from the Energy shard...)
        let max_energy = self.max_energy(difficulty);

        if difficulty >= logical_difficulty::OVERFLOW && self.shards.contains(&Shard::Overflow) {
            if orbs.health > max_health {
                orbs.energy += orbs.health - max_health;
            } else if orbs.energy > max_energy {
                orbs.health += orbs.energy - max_energy;
            }
        }

        orbs.health = orbs.health.min(max_health);
        orbs.energy = orbs.energy.min(max_energy);
    }
    pub fn checkpoint_orbs(&self, difficulty: Difficulty) -> Orbs {
        let health_refill = (self.max_health(difficulty) * 0.3).ceil().max(40.0);
        let energy_refill = (self.max_energy(difficulty) * 0.2).ceil().max(1.0);

        let mut orbs = Orbs {
            health: health_refill,
            energy: energy_refill,
        };

        self.cap_orbs(&mut orbs, true, difficulty);
        orbs
    }
    pub fn health_plant_drops(&self, difficulty: Difficulty) -> f32 {
        let value = self.max_health(difficulty) / 30.0;
        // the game rounds to even
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
            clippy::float_cmp
        )]
        if value % 1. == 0.5 && value as u8 % 2 == 0 {
            value.floor()
        } else {
            value.round()
        }
    }

    /// Replenish health, but don't exceed the inventory's maximum health
    pub fn heal(&self, orbs: &mut Orbs, amount: f32, difficulty: Difficulty) {
        orbs.health = (orbs.health + amount).min(self.max_health(difficulty));
    }
    /// Replenish energy, but don't exceed the inventory's maximum energy
    pub fn recharge(&self, orbs: &mut Orbs, amount: f32, difficulty: Difficulty) {
        orbs.energy = (orbs.energy + amount).min(self.max_energy(difficulty));
    }
    /// Apply the refill from a [`RefillValue`] to a set of [`OrbVariants`]
    pub(crate) fn refill(
        &self,
        refill: RefillValue,
        orb_variants: &mut OrbVariants,
        difficulty: Difficulty,
    ) {
        debug_assert!(!orb_variants.is_empty());
        match refill {
            RefillValue::Full => *orb_variants = smallvec![self.max_orbs(difficulty)],
            RefillValue::Checkpoint => {
                *orb_variants = orbs::either_single(orb_variants, self.checkpoint_orbs(difficulty))
            }
            RefillValue::Health(amount) => {
                let amount = amount * self.health_plant_drops(difficulty);
                orb_variants
                    .iter_mut()
                    .for_each(|orbs| self.heal(orbs, amount, difficulty));
            }
            RefillValue::Energy(amount) => {
                orb_variants
                    .iter_mut()
                    .for_each(|orbs| self.recharge(orbs, amount, difficulty));
            }
        }
    }

    pub fn damage_mod(&self, flying_target: bool, bow: bool, settings: &WorldSettings) -> f32 {
        let mut damage_mod = 1.0;

        if settings.difficulty >= logical_difficulty::DAMAGE_BUFFS {
            if self.skills.contains(&Skill::GladesAncestralLight) {
                damage_mod += 0.25;
            }
            if self.skills.contains(&Skill::InkwaterAncestralLight) {
                damage_mod += 0.25;
            }

            let mut slots = self.get_resource(Resource::ShardSlot);
            let mut splinter = false;

            if flying_target && slots > 0 && self.shards.contains(&Shard::Wingclip) {
                damage_mod += 1.0;
                slots -= 1;
            }
            if slots > 0 && bow && self.shards.contains(&Shard::Splinter) {
                splinter = true;
                slots -= 1;
            }
            if slots > 0 && self.shards.contains(&Shard::SpiritSurge) {
                damage_mod += (self.spirit_light / 10000) as f32; // TODO but this is capped right
                slots -= 1;
            }
            if slots > 0 && self.shards.contains(&Shard::LastStand) {
                damage_mod += 0.2;
                slots -= 1;
            }
            if slots > 0 && self.shards.contains(&Shard::Reckless) {
                damage_mod += 0.15;
                slots -= 1;
            }
            if slots > 0 && self.shards.contains(&Shard::Lifeforce) {
                damage_mod += 0.1;
                slots -= 1;
            }
            if slots > 0 && self.shards.contains(&Shard::Finesse) {
                damage_mod += 0.05;
            }
            if splinter {
                damage_mod *= 1.5;
            } // Splinter stacks multiplicatively where other buffs stack additively
        }

        damage_mod
    }
    pub fn energy_mod(&self, settings: &WorldSettings) -> f32 {
        let mut energy_mod = 1.0;
        if settings.difficulty < Difficulty::Unsafe {
            energy_mod *= 2.0;
        } else if self.shards.contains(&Shard::Overcharge) {
            energy_mod *= 0.5;
        }
        energy_mod
    }
    pub fn defense_mod(&self, settings: &WorldSettings) -> f32 {
        let mut defense_mod = if settings.difficulty >= logical_difficulty::RESILIENCE
            && self.shards.contains(&Shard::Resilience)
        {
            0.9
        } else {
            1.0
        };
        if settings.hard {
            defense_mod *= 2.0;
        }
        defense_mod
    }

    pub fn use_cost(&self, weapon: Skill, settings: &WorldSettings) -> f32 {
        weapon.energy_cost() * self.energy_mod(settings)
    }
    /// Returns the damage and cost of the weapon after all modifiers
    pub fn weapon_stats(
        &self,
        weapon: Skill,
        flying_target: bool,
        settings: &WorldSettings,
    ) -> (f32, f32) {
        let damage_mod = self.damage_mod(flying_target, matches!(weapon, Skill::Bow), settings);
        let damage = weapon.damage(settings.difficulty >= Difficulty::Unsafe) * damage_mod
            + weapon.burn_damage();
        let cost = weapon.energy_cost() * self.energy_mod(settings);
        (damage, cost)
    }
    /// Returns the energy required to destroy the target with the given weapon
    pub fn destroy_cost_with(
        &self,
        target_health: f32,
        weapon: Skill,
        flying_target: bool,
        settings: &WorldSettings,
    ) -> f32 {
        let (damage, cost) = self.weapon_stats(weapon, flying_target, settings);
        (target_health / damage).ceil() * cost
    }
    /// Returns the energy required to destroy the target with the given combination of weapons, or `None` if `weapons` is empty
    ///
    /// We optimize based on the assumption that `weapons` has energy-less weapons in front
    fn destroy_cost_with_any_of<const N: usize>(
        &self,
        mut target_health: f32,
        weapons: SmallVec<[Skill; N]>,
        flying_target: bool,
        settings: &WorldSettings,
    ) -> Option<f32> {
        if weapons.first()?.energy_cost() == 0.0 {
            return Some(0.0);
        }

        let weapon_stats = weapons
            .into_iter()
            .map(|weapon| self.weapon_stats(weapon, flying_target, settings))
            .collect::<SmallVec<[_; 9]>>();

        // Use the best weapon as long as it doesn't "waste" any damage
        let ((damage, mut cost), _) = weapon_stats
            .iter()
            .map(|(damage, cost)| ((*damage, *cost), damage / cost))
            .max_by(|(_, dpe_a), (_, dpe_b)| dpe_a.float_cmp(dpe_b))?;
        let optimal_hits = (target_health / damage).floor();
        target_health -= optimal_hits * damage;
        cost *= optimal_hits;

        // Figure out the best weapon to deal the last bit of damage
        cost += weapon_stats
            .into_iter()
            .map(|(damage, cost)| ((target_health / damage).ceil() * cost))
            .min_by(f32::float_cmp)?;

        // On arbitrary energy costs and damage amounts this procedure might choose suboptimal weapons to use, but for the defaults it should be exhaustive

        Some(cost)
    }
    /// Returns the energy required to destroy the target, or `None` if no weapons are available to attack the target
    pub fn destroy_cost<const TARGET_IS_WALL: bool>(
        &self,
        target_health: f32,
        flying_target: bool,
        settings: &WorldSettings,
    ) -> Option<f32> {
        self.destroy_cost_with_any_of(
            target_health,
            self.owned_weapons::<TARGET_IS_WALL>(settings),
            flying_target,
            settings,
        )
    }
    /// Returns the energy required to destroy the target with a ranged weapon, or `None` if no weapons are available to attack the target
    pub fn destroy_cost_ranged(
        &self,
        target_health: f32,
        flying_target: bool,
        settings: &WorldSettings,
    ) -> Option<f32> {
        self.destroy_cost_with_any_of(
            target_health,
            self.owned_ranged_weapons(settings),
            flying_target,
            settings,
        )
    }

    fn owned_weapons_from_fn<const N: usize, F>(
        &self,
        weapons_fn: F,
        settings: &WorldSettings,
    ) -> SmallVec<[Skill; N]>
    where
        F: FnOnce(Difficulty) -> SmallVec<[Skill; N]>,
    {
        let mut weapons = weapons_fn(settings.difficulty);
        weapons.retain(|weapon| self.skills.contains(weapon));
        weapons
    }
    // TODO would it be worth to precompile the resulting slices for all variants?
    pub fn owned_weapons<const TARGET_IS_WALL: bool>(
        &self,
        settings: &WorldSettings,
    ) -> SmallVec<[Skill; 9]> {
        self.owned_weapons_from_fn(logical_difficulty::weapons::<TARGET_IS_WALL>, settings)
    }
    pub fn owned_ranged_weapons(&self, settings: &WorldSettings) -> SmallVec<[Skill; 6]> {
        self.owned_weapons_from_fn(logical_difficulty::ranged_weapons, settings)
    }
    pub fn owned_shield_weapons(&self, settings: &WorldSettings) -> SmallVec<[Skill; 4]> {
        self.owned_weapons_from_fn(|_| logical_difficulty::shield_weapons(), settings)
    }

    fn progression_weapons_from_fn<const N: usize, F>(
        &self,
        weapons_fn: F,
        settings: &WorldSettings,
    ) -> SmallVec<[Skill; N]>
    where
        F: FnOnce(Difficulty) -> SmallVec<[Skill; N]>,
    {
        fn damage_per_energy(weapon: Skill, settings: &WorldSettings) -> f32 {
            // (weapon.damage(unsafe_paths) + weapon.burn_damage()) / weapon.energy_cost()
            (10.0
                / (weapon.damage(settings.difficulty >= Difficulty::Unsafe) + weapon.burn_damage()))
            .ceil()
                * weapon.energy_cost()
            // "how much energy do you need to deal 10 damage" leads to a more realistic ordering than pure damage per energy
        }

        let mut weapons = weapons_fn(settings.difficulty);
        // TODO check whether creating this map is even worth it
        let dpe_map = weapons
            .iter()
            .map(|weapon| {
                (
                    *weapon,
                    (damage_per_energy(*weapon, settings) * 10.0) as u16,
                )
            })
            .collect::<FxHashMap<Skill, u16>>();
        weapons.sort_unstable_by_key(|weapon| dpe_map[weapon]);
        if let Some((index, weapon)) = weapons
            .iter()
            .enumerate()
            .find(|(_, weapon)| self.skills.contains(*weapon))
        {
            let dpe = dpe_map[weapon];
            weapons.truncate(index + 1);
            // maybe there are multiple weapons costing the same and we already skipped over a redundant one
            weapons.swap(index, 0); // if we found something before, there must be at least one element
            let remove_after = weapons
                .iter()
                .rposition(|weapon| dpe_map[weapon] != dpe)
                .unwrap_or(0);
            weapons.truncate(remove_after + 1);
            weapons.swap(0, remove_after);
        }
        weapons
    }
    // TODO interesting point https://github.com/servo/rust-smallvec/issues/274
    pub fn progression_weapons<const TARGET_IS_WALL: bool>(
        &self,
        settings: &WorldSettings,
    ) -> SmallVec<[Skill; 9]> {
        self.progression_weapons_from_fn(logical_difficulty::weapons::<TARGET_IS_WALL>, settings)
    }
    pub fn ranged_progression_weapons(&self, settings: &WorldSettings) -> SmallVec<[Skill; 6]> {
        self.progression_weapons_from_fn(logical_difficulty::ranged_weapons, settings)
    }
    pub fn shield_progression_weapons(&self, settings: &WorldSettings) -> SmallVec<[Skill; 4]> {
        self.progression_weapons_from_fn(|_| logical_difficulty::shield_weapons(), settings)
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn comma(f: &mut fmt::Formatter<'_>, first: &mut bool) -> fmt::Result {
            if mem::take(first) {
                Ok(())
            } else {
                write!(f, ", ")
            }
        }

        let mut first = true;

        if self.spirit_light > 0 {
            first = false;
            write!(f, "{} Spirit Light", self.spirit_light)?;
        }
        for (resource, amount) in &self.resources {
            comma(f, &mut first)?;
            write!(f, "{amount} {resource}")?;
        }
        for skill in &self.skills {
            comma(f, &mut first)?;
            skill.fmt(f)?;
        }
        for shard in &self.shards {
            comma(f, &mut first)?;
            shard.fmt(f)?;
        }
        for teleporter in &self.teleporters {
            comma(f, &mut first)?;
            teleporter.fmt(f)?;
        }
        if self.clean_water {
            comma(f, &mut first)?;
            write!(f, "Clean Water")?;
        }

        Ok(())
    }
}

impl Add for Inventory {
    type Output = Inventory;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl AddAssign for Inventory {
    fn add_assign(&mut self, rhs: Self) {
        self.spirit_light += rhs.spirit_light;
        for (resource, amount) in rhs.resources {
            self.add_resource(resource, amount);
        }
        self.skills.extend(rhs.skills);
        self.shards.extend(rhs.shards);
        self.teleporters.extend(rhs.teleporters);
        self.clean_water |= rhs.clean_water;
    }
}
impl Sub for Inventory {
    type Output = Inventory;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl SubAssign for Inventory {
    fn sub_assign(&mut self, rhs: Self) {
        self.spirit_light = self.spirit_light.saturating_sub(rhs.spirit_light);
        for (resource, amount) in rhs.resources {
            self.add_resource(resource, -amount);
        }
        for skill in rhs.skills {
            self.skills.remove(&skill);
        }
        for shard in rhs.shards {
            self.shards.remove(&shard);
        }
        for teleporter in rhs.teleporters {
            self.teleporters.remove(&teleporter);
        }
        self.clean_water &= rhs.clean_water;
    }
}
impl SubAssign<&Self> for Inventory {
    fn sub_assign(&mut self, rhs: &Self) {
        self.spirit_light = self.spirit_light.saturating_sub(rhs.spirit_light);
        for (resource, amount) in &rhs.resources {
            self.add_resource(*resource, -*amount);
        }
        for skill in &rhs.skills {
            self.skills.remove(skill);
        }
        for shard in &rhs.shards {
            self.shards.remove(shard);
        }
        for teleporter in &rhs.teleporters {
            self.teleporters.remove(teleporter);
        }
        self.clean_water &= rhs.clean_water;
    }
}
