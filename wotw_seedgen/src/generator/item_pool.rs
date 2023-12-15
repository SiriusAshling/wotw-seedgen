use std::iter;

use crate::{inventory::Inventory, log};
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use wotw_seedgen_data::{Resource, Shard, Skill, WeaponUpgrade};
use wotw_seedgen_seed_language::output::{Action, Command, CommonItem};

use super::cost::Cost;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemPool {
    items: Vec<usize>,
    item_lookup: Vec<Action>,
    inventory: Inventory,
}
impl Default for ItemPool {
    fn default() -> Self {
        Self {
            items: [
                iter::repeat(0).take(24),
                iter::repeat(1).take(24),
                iter::repeat(2).take(40),
                iter::repeat(3).take(34),
                iter::repeat(4).take(5),
            ]
            .into_iter()
            .flatten()
            .chain(5..=63)
            .collect(),
            item_lookup: [
                Action::Command(Command::Custom(CommonItem::Resource(
                    Resource::HealthFragment,
                ))),
                Action::Command(Command::Custom(CommonItem::Resource(
                    Resource::EnergyFragment,
                ))),
                Action::Command(Command::Custom(CommonItem::Resource(Resource::GorlekOre))),
                Action::Command(Command::Custom(CommonItem::Resource(Resource::Keystone))),
                Action::Command(Command::Custom(CommonItem::Resource(Resource::ShardSlot))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Bash))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::DoubleJump))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Launch))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Glide))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::WaterBreath))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Grenade))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Grapple))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Flash))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Spear))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Regenerate))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Bow))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Hammer))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Sword))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Burrow))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Dash))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::WaterDash))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Shuriken))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Blaze))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Sentry))),
                Action::Command(Command::Custom(CommonItem::Skill(Skill::Flap))),
                Action::Command(Command::Custom(CommonItem::Skill(
                    Skill::GladesAncestralLight,
                ))),
                Action::Command(Command::Custom(CommonItem::Skill(
                    Skill::InkwaterAncestralLight,
                ))),
                Action::Command(Command::Custom(CommonItem::CleanWater)),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Overcharge))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::TripleJump))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Wingclip))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Bounty))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Swap))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Magnet))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Splinter))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Reckless))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Quickshot))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Resilience))),
                Action::Command(Command::Custom(CommonItem::Shard(
                    Shard::SpiritLightHarvest,
                ))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Vitality))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::LifeHarvest))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::EnergyHarvest))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Energy))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::LifePact))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::LastStand))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Sense))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::UltraBash))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::UltraGrapple))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Overflow))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Thorn))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Catalyst))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Turmoil))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Sticky))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Finesse))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::SpiritSurge))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Lifeforce))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Deflector))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Fracture))),
                Action::Command(Command::Custom(CommonItem::Shard(Shard::Arcing))),
                Action::Command(Command::Custom(CommonItem::WeaponUpgrade(
                    WeaponUpgrade::ExplodingSpear,
                ))),
                Action::Command(Command::Custom(CommonItem::WeaponUpgrade(
                    WeaponUpgrade::ShockHammer,
                ))),
                Action::Command(Command::Custom(CommonItem::WeaponUpgrade(
                    WeaponUpgrade::StaticShuriken,
                ))),
                Action::Command(Command::Custom(CommonItem::WeaponUpgrade(
                    WeaponUpgrade::ChargeBlaze,
                ))),
                Action::Command(Command::Custom(CommonItem::WeaponUpgrade(
                    WeaponUpgrade::RapidSentry,
                ))),
            ]
            .into_iter()
            .collect(),
            inventory: Inventory {
                spirit_light: 0,
                resources: [
                    (Resource::HealthFragment, 24),
                    (Resource::EnergyFragment, 24),
                    (Resource::Keystone, 40),
                    (Resource::GorlekOre, 34),
                    (Resource::ShardSlot, 5),
                ]
                .into_iter()
                .collect(),
                skills: [
                    Skill::Bash,
                    Skill::DoubleJump,
                    Skill::Launch,
                    Skill::Glide,
                    Skill::WaterBreath,
                    Skill::Grenade,
                    Skill::Grapple,
                    Skill::Flash,
                    Skill::Spear,
                    Skill::Regenerate,
                    Skill::Bow,
                    Skill::Hammer,
                    Skill::Sword,
                    Skill::Burrow,
                    Skill::Dash,
                    Skill::WaterDash,
                    Skill::Shuriken,
                    Skill::Blaze,
                    Skill::Sentry,
                    Skill::Flap,
                    Skill::GladesAncestralLight,
                    Skill::InkwaterAncestralLight,
                ]
                .into_iter()
                .collect(),
                shards: [
                    Shard::Overcharge,
                    Shard::TripleJump,
                    Shard::Wingclip,
                    Shard::Bounty,
                    Shard::Swap,
                    Shard::Magnet,
                    Shard::Splinter,
                    Shard::Reckless,
                    Shard::Quickshot,
                    Shard::Resilience,
                    Shard::SpiritLightHarvest,
                    Shard::Vitality,
                    Shard::LifeHarvest,
                    Shard::EnergyHarvest,
                    Shard::Energy,
                    Shard::LifePact,
                    Shard::LastStand,
                    Shard::Sense,
                    Shard::UltraBash,
                    Shard::UltraGrapple,
                    Shard::Overflow,
                    Shard::Thorn,
                    Shard::Catalyst,
                    Shard::Turmoil,
                    Shard::Sticky,
                    Shard::Finesse,
                    Shard::SpiritSurge,
                    Shard::Lifeforce,
                    Shard::Deflector,
                    Shard::Fracture,
                    Shard::Arcing,
                ]
                .into_iter()
                .collect(),
                teleporters: Default::default(),
                clean_water: true,
                weapon_upgrades: [
                    WeaponUpgrade::ExplodingSpear,
                    WeaponUpgrade::ShockHammer,
                    WeaponUpgrade::StaticShuriken,
                    WeaponUpgrade::ChargeBlaze,
                    WeaponUpgrade::RapidSentry,
                ]
                .into_iter()
                .collect(),
            },
        }
    }
}
impl ItemPool {
    pub fn change(&mut self, action: Action, mut amount: i32) {
        // TODO update the inventory accordingly
        let index = self
            .item_lookup
            .iter()
            .enumerate()
            .find(|(_, a)| *a == &action)
            .map(|(index, _)| index);

        if amount > 0 {
            let index = match index {
                None => {
                    let index = self.item_lookup.len();
                    self.item_lookup.push(action);
                    index
                }
                Some(index) => index,
            };
            self.items.extend(iter::repeat(index).take(amount as usize));
        } else if let Some(index) = index {
            self.items.retain(|i| {
                amount == 0
                    || if *i == index {
                        amount += 1;
                        false
                    } else {
                        true
                    }
            });
        }
    }

    pub fn choose_random(&mut self, rng: &mut Pcg64Mcg) -> Option<Action> {
        if self.items.is_empty() {
            return None;
        }
        // TODO why not swap_remove? same in other places
        let index = self.items.remove(rng.gen_range(0..self.items.len()));
        let action = self.item_lookup[index].clone();

        let cost = action.cost();
        if cost > 10000 {
            let reroll_chance = -10000.0 / cost as f64 + 1.0;

            if rng.gen_bool(reroll_chance) {
                log::trace!("Rerolling random placement {action}");
                self.items.push(index);
                return self.choose_random(rng);
            }
        }

        Some(action)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[inline]
    pub fn contains(&self, inventory: &Inventory) -> bool {
        self.inventory.contains(inventory)
    }

    #[inline]
    pub fn drain<'pool>(&'pool mut self, rng: &mut Pcg64Mcg) -> Drain<'pool> {
        Drain::new(self, rng)
    }
}

pub struct Drain<'pool> {
    item_pool: &'pool mut ItemPool,
}
impl<'pool> Drain<'pool> {
    fn new(item_pool: &'pool mut ItemPool, rng: &mut Pcg64Mcg) -> Self {
        item_pool.items.shuffle(rng);
        Self { item_pool }
    }
}
impl Iterator for Drain<'_> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.item_pool
            .items
            .pop()
            .map(|index| self.item_pool.item_lookup[index].clone())
    }
}
