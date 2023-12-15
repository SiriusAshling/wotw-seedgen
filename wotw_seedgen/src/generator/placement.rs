use super::{
    cost::Cost, item_pool::ItemPool, spirit_light::SpiritLightProvider, Seed, SEED_FAILED_MESSAGE,
};
use crate::{
    constants::{KEYSTONE_DOORS, PREFERRED_SPAWN_SLOTS, SPAWN_SLOTS, UNSHARED_ITEMS},
    filter_redundancies,
    inventory::Inventory,
    node_condition, node_trigger,
    orbs::OrbVariants,
    ReachedLocations, World,
};
use decorum::R32;
use itertools::Itertools;
use rand::{
    distributions::Uniform,
    prelude::Distribution,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashMap;
use std::{iter, mem, ops::RangeFrom};
use wotw_seedgen_data::{Equipment, MapIcon, OpherIcon, Resource, Skill, UberIdentifier};
use wotw_seedgen_logic_language::output::{Node, Requirement};
use wotw_seedgen_seed::{Icon, PseudoTrigger};
use wotw_seedgen_seed_language::output::{
    Action, Command, CommandBoolean, CommandFloat, CommandIcon, CommandInteger, CommandString,
    CommandVoid, CommonItem, CompilerOutput, Event, StringOrPlaceholder, Trigger,
};

pub(crate) fn generate_placements(
    rng: &mut Pcg64Mcg,
    worlds: Vec<(World, CompilerOutput)>,
) -> Result<Seed, String> {
    let mut context = Context::new(rng, worlds);

    context.preplacements();

    loop {
        context.update_reached();
        if context.everything_reached() {
            context.place_remaining();
            todo!();
        }
        context.force_keystones();
        if !context.place_random() {
            let (target_world_index, progression) = context.choose_progression();
            context.place_forced(target_world_index, progression);
        }
    }
}

struct Context<'graph, 'settings> {
    rng: Pcg64Mcg,
    worlds: Vec<WorldContext<'graph, 'settings>>,
    multiworld_state_index: RangeFrom<i32>,
}
impl<'graph, 'settings> Context<'graph, 'settings> {
    fn new(rng: &mut Pcg64Mcg, worlds: Vec<(World<'graph, 'settings>, CompilerOutput)>) -> Self {
        Self {
            rng: Pcg64Mcg::from_rng(&mut *rng).expect(SEED_FAILED_MESSAGE),
            worlds: worlds
                .into_iter()
                .map(|(world, output)| WorldContext::new(rng, world, output))
                .collect(),
            multiworld_state_index: 0..,
        }
    }

    fn preplacements(&mut self) {
        for world_context in &mut self.worlds {
            world_context.preplacements();
        }
    }

    fn update_reached(&mut self) {
        for world_context in &mut self.worlds {
            world_context.update_reached();
        }
    }

    fn everything_reached(&self) -> bool {
        self.worlds
            .iter()
            .all(|world| world.reached_needs_placement.len() == world.needs_placement.len())
    }

    fn force_keystones(&mut self) {
        for world_index in 0..self.worlds.len() {
            let world_context = &mut self.worlds[world_index];
            let owned_keystones = world_context
                .world
                .inventory()
                .get_resource(Resource::Keystone);
            if owned_keystones < 2 {
                continue;
            }

            let required_keystones = KEYSTONE_DOORS
                .iter()
                .filter_map(|(identifier, amount)| {
                    world_context
                        .reached
                        .iter()
                        .any(|node| node.identifier() == *identifier)
                        .then_some(*amount)
                })
                .sum::<i32>();
            if required_keystones <= owned_keystones {
                continue;
            }

            for _ in owned_keystones..required_keystones {
                self.place_action(
                    common(CommonItem::Resource(Resource::Keystone)),
                    world_index,
                );
            }
        }
    }

    fn place_remaining(&mut self) {
        for target_world_index in 0..self.worlds.len() {
            for action in self.worlds[target_world_index]
                .item_pool
                .drain(&mut self.rng)
                .collect::<Vec<_>>()
            {
                self.place_action(action, target_world_index);
            }
        }
        for world_context in &mut self.worlds {
            world_context.fill_remaining();
        }
    }

    fn place_random(&mut self) -> bool {
        let mut any_placed = false;
        // TODO placeholders
        for origin_world_index in 0..self.worlds.len() {
            let reached_needs_placement =
                mem::take(&mut self.worlds[origin_world_index].reached_needs_placement);
            for node_index in reached_needs_placement.iter().copied() {
                any_placed = true;
                let origin_world = &mut self.worlds[origin_world_index];
                let slots_remaining = origin_world.slots_remaining();
                let place_spirit_light = self.rng.gen_bool(f64::max(
                    1. - origin_world.item_pool.len() as f64 / slots_remaining as f64,
                    0.,
                ));

                let (target_world_index, action) = if place_spirit_light {
                    let batch = origin_world.spirit_light_provider.take(slots_remaining);
                    (origin_world_index, common(CommonItem::SpiritLight(batch)))
                } else {
                    let target_world_index = self.choose_target_world(origin_world_index);
                    (
                        target_world_index,
                        self.worlds[target_world_index]
                            .item_pool
                            .choose_random(&mut self.rng)
                            .unwrap(),
                    )
                };

                let node = self.worlds[origin_world_index].needs_placement[node_index];
                let name = self.name(&action, origin_world_index, target_world_index);
                self.place_action_at(action, name, node, origin_world_index, target_world_index);
            }
            self.worlds[origin_world_index]
                .received_placement
                .extend(reached_needs_placement);
        }
        any_placed
    }

    fn choose_progression(&mut self) -> (usize, Inventory) {
        let slots = self
            .worlds
            .iter()
            .map(|world_context| world_context.reached_needs_placement.len())
            .sum::<usize>();

        let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
        world_indices.sort_by_key(|index| self.worlds[*index].slots_remaining());

        for target_world_index in world_indices {
            if let Some(progression) = self.worlds[target_world_index].choose_progression(slots) {
                return (target_world_index, progression);
            }
        }
        // TODO flush item pool
        todo!()
    }

    fn place_forced(&mut self, target_world_index: usize, progression: Inventory) {
        let Inventory {
            spirit_light,
            resources,
            skills,
            shards,
            teleporters,
            clean_water,
            weapon_upgrades,
        } = progression;

        self.worlds[target_world_index].place_spirit_light(spirit_light);

        resources
            .into_iter()
            .flat_map(|(resource, amount)| {
                iter::repeat(common(CommonItem::Resource(resource))).take(amount as usize)
            })
            .chain(
                skills
                    .into_iter()
                    .map(|skill| common(CommonItem::Skill(skill))),
            )
            .chain(
                shards
                    .into_iter()
                    .map(|shard| common(CommonItem::Shard(shard))),
            )
            .chain(
                teleporters
                    .into_iter()
                    .map(|teleporter| common(CommonItem::Teleporter(teleporter))),
            )
            .chain(clean_water.then_some(common(CommonItem::CleanWater)))
            .chain(
                weapon_upgrades
                    .into_iter()
                    .map(|weapon_upgrade| common(CommonItem::WeaponUpgrade(weapon_upgrade))),
            )
            .for_each(|action| self.place_action(action, target_world_index));
    }

    fn place_action(&mut self, action: Action, target_world_index: usize) {
        let origin_world_index = self.choose_origin_world(&action, target_world_index);
        let name = self.name(&action, origin_world_index, target_world_index);
        let origin_world = &mut self.worlds[origin_world_index];
        match origin_world.choose_placement_node(&action) {
            None => {
                if origin_world.spawn_slots > 0 {
                    origin_world.spawn_slots -= 1;
                    self.push_action(
                        Trigger::Pseudo(PseudoTrigger::Spawn),
                        action,
                        name,
                        origin_world_index,
                        target_world_index,
                    );
                } else {
                    panic!("Not enough space to place item"); // TODO this can actually happen, should maybe be a warning
                }
            }
            Some(node) => {
                self.place_action_at(action, name, node, origin_world_index, target_world_index);
            }
        }
    }

    fn choose_origin_world(&mut self, action: &Action, target_world_index: usize) -> usize {
        match action {
            Action::Command(Command::Custom(CommonItem::SpiritLight(_))) => target_world_index,
            _ => {
                if self.worlds[target_world_index].unshared_items > 0 {
                    self.worlds[target_world_index].unshared_items -= 1;
                    target_world_index
                } else {
                    let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
                    world_indices.shuffle(&mut self.rng);
                    world_indices
                        .iter()
                        .find(|index| !self.worlds[**index].reached_needs_placement.is_empty())
                        .copied()
                        .or_else(|| {
                            world_indices
                                .into_iter()
                                .find(|index| self.worlds[*index].spawn_slots > 0)
                        })
                        .unwrap() // TODO handle
                }
            }
        }
    }

    fn choose_target_world(&mut self, origin_world_index: usize) -> usize {
        if self.worlds[origin_world_index].unshared_items > 0 {
            self.worlds[origin_world_index].unshared_items -= 1;
            origin_world_index
        } else {
            let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
            world_indices.shuffle(&mut self.rng);
            world_indices
                .into_iter()
                .find_or_last(|index| !self.worlds[*index].item_pool.is_empty())
                .unwrap()
        }
    }

    fn name(
        &self,
        action: &Action,
        origin_world_index: usize,
        target_world_index: usize,
    ) -> CommandString {
        let name = self.worlds[target_world_index].name(action);
        if origin_world_index == target_world_index {
            string(name)
        } else {
            CommandString::Concatenate {
                left: Box::new(CommandString::WorldName {
                    index: target_world_index,
                }),
                right: Box::new(string(format!("'s {name}"))),
            }
        }
    }

    fn place_action_at(
        &mut self,
        action: Action,
        name: CommandString,
        node: &Node,
        origin_world_index: usize,
        target_world_index: usize,
    ) {
        self.worlds[origin_world_index].map_icon(node, &action, name.clone());

        let uber_identifier = node.uber_identifier().unwrap();
        if uber_identifier.is_shop() {
            self.worlds[origin_world_index].shop_item_data(&action, uber_identifier, name.clone());
        }

        self.push_action(
            node_trigger(node).unwrap(),
            action,
            name,
            origin_world_index,
            target_world_index,
        );
    }

    fn push_action(
        &mut self,
        trigger: Trigger,
        action: Action,
        name: CommandString,
        origin_world_index: usize,
        target_world_index: usize,
    ) {
        if origin_world_index == target_world_index {
            self.worlds[origin_world_index].push_action(trigger, action);
        } else {
            let uber_identifier = self.multiworld_state();
            self.worlds[origin_world_index].push_action(
                trigger,
                Action::Multi(vec![
                    Action::Command(Command::Void(CommandVoid::ItemMessage { message: name })),
                    Action::Command(Command::Void(CommandVoid::StoreBoolean {
                        uber_identifier,
                        value: boolean(true),
                        check_triggers: true,
                    })),
                ]),
            );
            self.worlds[target_world_index].push_action(
                Trigger::Binding(uber_identifier), // this is server synced and can't change to false
                action,
            );
        }
    }

    fn multiworld_state(&mut self) -> UberIdentifier {
        UberIdentifier {
            group: 12,
            member: self.multiworld_state_index.next().unwrap(),
        }
    }
}
struct WorldContext<'graph, 'settings> {
    rng: Pcg64Mcg,
    world: World<'graph, 'settings>,
    output: CompilerOutput,
    item_pool: ItemPool,
    spirit_light_provider: SpiritLightProvider,
    needs_placement: Vec<&'graph Node>,
    reached: Vec<&'graph Node>,
    progressions: Vec<(&'graph Requirement, OrbVariants)>,
    reached_needs_placement: Vec<usize>,
    received_placement: Vec<usize>,
    reached_item_locations: usize,
    spawn_slots: usize,
    unshared_items: usize,
    on_load_index: usize,
    price_distribution: Uniform<f32>,
}
impl<'graph, 'settings> WorldContext<'graph, 'settings> {
    fn new(
        rng: &mut Pcg64Mcg,
        world: World<'graph, 'settings>,
        mut output: CompilerOutput,
    ) -> Self {
        let mut item_pool = ItemPool::default();

        for (action, amount) in mem::take(&mut output.item_pool_changes) {
            item_pool.change(action, amount);
        }

        let needs_placement = world
            .graph
            .nodes
            .iter()
            // TODO optimize based on shape of events, many of which can't possibly be loc_data events
            .filter(|node| {
                node.can_place()
                    && !output.events.iter().any(|event|
                        matches!(&event.trigger, Trigger::Condition(condition) if Some(condition) == node_condition(node).as_ref())
                    )
            })
            .collect::<Vec<_>>();
        // TODO filter out unreachable locations

        let on_load_index = match output
            .events
            .iter_mut()
            .enumerate()
            .find(|(_, event)| matches!(event.trigger, Trigger::Pseudo(PseudoTrigger::Reload)))
        {
            None => {
                let index = output.events.len();
                output.events.push(Event {
                    trigger: Trigger::Pseudo(PseudoTrigger::Reload),
                    action: Action::Multi(vec![]),
                });
                index
            }
            Some((index, event)) => {
                let action = mem::replace(&mut event.action, Action::Multi(vec![]));
                if let Action::Multi(actions) = &mut event.action {
                    actions.push(action);
                }
                index
            }
        };

        Self {
            rng: Pcg64Mcg::from_rng(&mut *rng).expect(SEED_FAILED_MESSAGE),
            world,
            output,
            item_pool,
            spirit_light_provider: SpiritLightProvider::new(20000, rng), // TODO how should !add(spirit_light(100)) behave?
            needs_placement,
            reached: Default::default(),
            progressions: Default::default(),
            reached_needs_placement: Default::default(),
            received_placement: Default::default(),
            reached_item_locations: Default::default(),
            spawn_slots: SPAWN_SLOTS,
            unshared_items: UNSHARED_ITEMS,
            on_load_index,
            price_distribution: Uniform::new_inclusive(0.75, 1.25),
        }
    }

    fn preplacements(&mut self) {
        self.hi_sigma();

        let mut zone_needs_placement = FxHashMap::default();
        for (action, zone) in mem::take(&mut self.output.preplacements) {
            let nodes = zone_needs_placement.entry(zone).or_insert_with(|| {
                self.needs_placement
                    .iter()
                    .enumerate()
                    .filter(|(_, node)| node.zone() == Some(zone))
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>()
            });
            if nodes.is_empty() {
                todo!()
            }
            // We prefer generating indices over shuffling the nodes because usually there aren't many zone preplacements (relics)
            let node_index = nodes.swap_remove(self.rng.gen_range(0..nodes.len()));
            let node = self.needs_placement[node_index];
            self.push_action(node_trigger(node).unwrap(), action);
            self.received_placement.push(node_index);
        }
    }

    fn hi_sigma(&mut self) {
        let node = self
            .needs_placement
            .swap_remove(self.rng.gen_range(0..self.needs_placement.len())); // TODO handle empty
        self.push_action(
            node_trigger(node).unwrap(),
            common(CommonItem::SpiritLight(1)),
        );
    }

    fn update_reached(&mut self) {
        let mut received_placement = mem::take(&mut self.received_placement);
        received_placement.sort();
        for node_index in received_placement.into_iter().rev() {
            self.needs_placement.swap_remove(node_index);
        }

        let reached_locations = self.world.reached_and_progressions();
        self.reached = reached_locations.reached;
        self.progressions = reached_locations.progressions;
        self.reached_needs_placement = self
            .reached
            .iter()
            .enumerate()
            .filter(|(_, node)| self.needs_placement.contains(node))
            .map(|(index, _)| index)
            .collect();
        self.reached_item_locations = self.reached.iter().filter(|node| node.can_place()).count();
    }

    fn slots_remaining(&self) -> usize {
        self.needs_placement.len() - self.received_placement.len()
    }

    fn choose_progression(&mut self, slots: usize) -> Option<Inventory> {
        let world_slots = self.reached_needs_placement.len();
        let mut progressions = mem::take(&mut self.progressions)
            .into_iter()
            .flat_map(|(requirement, best_orbs)| {
                self.world.player.solutions(
                    &requirement,
                    &self.world.logic_states,
                    best_orbs,
                    slots,
                    world_slots,
                )
            })
            .filter(|solution| self.item_pool.contains(solution))
            .collect();
        // TODO is it desirable to filter here again? they have already been filterer per-solutions-call
        filter_redundancies(&mut progressions);

        let weights = progressions
            .iter()
            .enumerate()
            .map(|(index, inventory)| {
                self.world.player.inventory += inventory.clone();
                let mut lookahead_reachable = self.world.reached();
                self.world.player.inventory -= inventory;
                lookahead_reachable.retain(|&node| node.can_place());

                // Resource tracking can result in reaching less locations with an added teleporter, so prevent any overflows.
                // This is very rare and usually means the granted teleporter doesn't actually lead anywhere new, so 0 newly reached is accurate enough.
                let newly_reached = lookahead_reachable
                    .len()
                    .saturating_sub(self.reached_item_locations);

                let mut weight = 1.0 / inventory.cost() as f32 * (newly_reached + 1) as f32;

                let begrudgingly_used_slots = (inventory.item_count() as usize
                    + (SPAWN_SLOTS - PREFERRED_SPAWN_SLOTS))
                    .saturating_sub(slots);
                if begrudgingly_used_slots > 0 {
                    weight *= (0.3_f32).powf(begrudgingly_used_slots as f32);
                }

                (index, weight)
            })
            .collect::<Vec<_>>();

        let index = weights
            .choose_weighted(&mut self.rng, |(_, weight)| *weight)
            .ok()?
            .0; // TODO handle

        Some(progressions.swap_remove(index))
    }

    fn place_spirit_light(&mut self, mut amount: i32) {
        while amount > 0 {
            let batch = self.spirit_light_provider.take(self.slots_remaining());
            amount -= batch;
            let action = common(CommonItem::SpiritLight(batch));
            let node = self.choose_placement_node(&action).unwrap();
            self.push_action(node_trigger(node).unwrap(), action);
        }
    }

    fn choose_placement_node(&mut self, action: &Action) -> Option<&'graph Node> {
        match action {
            Action::Command(Command::Custom(CommonItem::SpiritLight(_))) => self
                .reached_needs_placement
                .iter()
                .enumerate()
                .filter(|(_, node_index)| {
                    !self.needs_placement[**node_index]
                        .uber_identifier()
                        .unwrap()
                        .is_shop()
                })
                .map(|(index, _)| index)
                .choose(&mut self.rng),
            _ => (!self.reached_needs_placement.is_empty())
                .then(|| self.rng.gen_range(0..self.reached_needs_placement.len())),
        }
        .map(|index| {
            let node_index = self.reached_needs_placement.swap_remove(index);
            self.received_placement.push(node_index);
            self.needs_placement[node_index]
        })
    }

    fn map_icon(&mut self, node: &Node, action: &Action, label: CommandString) {
        let icon = self
            .output
            .item_metadata
            .get(action)
            .and_then(|metadata| metadata.map_icon.clone())
            .unwrap_or_else(|| match action {
                Action::Command(Command::Custom(common_item)) => match common_item {
                    CommonItem::SpiritLight(_) => MapIcon::Experience,
                    CommonItem::Resource(resource) => match resource {
                        Resource::HealthFragment => MapIcon::HealthFragment,
                        Resource::EnergyFragment => MapIcon::EnergyFragment,
                        Resource::GorlekOre => MapIcon::Ore,
                        Resource::Keystone => MapIcon::Keystone,
                        Resource::ShardSlot => MapIcon::ShardSlotUpgrade,
                    },
                    CommonItem::Skill(_) => MapIcon::AbilityPedestal,
                    CommonItem::Shard(_) => MapIcon::SpiritShard,
                    CommonItem::Teleporter(_) => MapIcon::Teleporter,
                    _ => MapIcon::QuestItem,
                },
                _ => MapIcon::QuestItem,
            });

        self.on_load(Action::Command(Command::Void(
            CommandVoid::SetSpoilerMapIcon {
                location: string(node.identifier().to_string()),
                icon,
                label,
            },
        )));
    }

    fn name(&self, action: &Action) -> String {
        self.output
            .item_metadata
            .get(action)
            .and_then(|metadata| metadata.name.clone())
            .unwrap_or_else(|| action.to_string())
    }

    fn on_load(&mut self, action: Action) {
        // This will be true because we forced it in the constructor
        if let Action::Multi(actions) = &mut self.output.events[self.on_load_index].action {
            // we only use this for metadata stuff so no need to simulate
            actions.push(action);
        }
    }

    fn shop_item_data(
        &mut self,
        action: &Action,
        uber_identifier: UberIdentifier,
        name: CommandString,
    ) {
        let (price, description, icon) = self
            .output
            .item_metadata
            .get(action)
            .cloned()
            .map_or((None, None, None), |metadata| {
                (metadata.price, metadata.description, metadata.icon)
            });

        let price = price.unwrap_or_else(|| integer(self.shop_price(action)));
        let icon = icon.or_else(|| default_icon(action));

        let mut actions = vec![
            Action::Command(Command::Void(CommandVoid::SetShopItemPrice {
                uber_identifier,
                price,
            })),
            Action::Command(Command::Void(CommandVoid::SetShopItemName {
                uber_identifier,
                name,
            })),
        ];
        if let Some(description) = description {
            actions.push(Action::Command(Command::Void(
                CommandVoid::SetShopItemDescription {
                    uber_identifier,
                    description,
                },
            )))
        }
        if let Some(icon) = icon {
            actions.push(Action::Command(Command::Void(
                CommandVoid::SetShopItemIcon {
                    uber_identifier,
                    icon,
                },
            )))
        }

        self.on_load(Action::Multi(actions));
    }

    fn shop_price(&mut self, action: &Action) -> i32 {
        let base_price = match action {
            Action::Command(Command::Custom(common_item)) => match common_item {
                CommonItem::Resource(Resource::HealthFragment) => 200.,
                CommonItem::Resource(Resource::EnergyFragment) => 150.,
                CommonItem::Resource(Resource::GorlekOre | Resource::Keystone) => 100.,
                CommonItem::Resource(Resource::ShardSlot) => 250.,
                CommonItem::Skill(skill) => match skill {
                    Skill::WaterBreath | Skill::Regenerate | Skill::Seir => 200.,
                    Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => 300.,
                    Skill::Blaze => return 420,
                    Skill::Launch => 800.,
                    _ => 500.,
                },
                CommonItem::CleanWater => 500.,
                CommonItem::Teleporter(_) | CommonItem::Shard(_) => 250.,
                _ => 200.,
            },
            _ => 200.,
        };

        (base_price * self.price_distribution.sample(&mut self.rng)).round() as i32
    }

    fn fill_remaining(&mut self) {
        let mut needs_placement = mem::take(&mut self.needs_placement);
        needs_placement.shuffle(&mut self.rng);

        for (slots_remaining, node) in needs_placement.into_iter().enumerate().rev() {
            let uber_identifier = node.uber_identifier().unwrap();
            let action = common(if uber_identifier.is_shop() {
                // TODO warn and also try to avoid
                CommonItem::Resource(Resource::GorlekOre)
            } else {
                CommonItem::SpiritLight(self.spirit_light_provider.take(slots_remaining))
            });
            let name = string(self.name(&action));
            self.shop_item_data(&action, uber_identifier, name.clone());
            self.push_action(node_trigger(node).unwrap(), action)
        }
        // TODO unreachable items that should be filled
    }

    fn push_action(&mut self, trigger: Trigger, action: Action) {
        self.world.uber_states.register_trigger(&trigger);
        self.world.simulate(&action, &self.output);
        self.output.events.push(Event { trigger, action });
    }
}

fn common(item: CommonItem) -> Action {
    Action::Command(Command::Custom(item))
}
fn boolean(value: bool) -> CommandBoolean {
    CommandBoolean::Constant { value }
}
fn integer(value: i32) -> CommandInteger {
    CommandInteger::Constant { value }
}
fn float(value: R32) -> CommandFloat {
    CommandFloat::Constant { value }
}
fn string(value: String) -> CommandString {
    CommandString::Constant {
        value: StringOrPlaceholder::Value(value),
    }
}
fn icon(value: Icon) -> CommandIcon {
    CommandIcon::Constant { value }
}
fn read_icon(path: String) -> CommandIcon {
    CommandIcon::ReadIcon {
        path: Box::new(CommandString::Constant {
            value: StringOrPlaceholder::Value(path),
        }),
    }
}

fn default_icon(action: &Action) -> Option<CommandIcon> {
    match action {
        Action::Command(Command::Custom(common_item)) => match common_item {
            CommonItem::SpiritLight(_) => {
                Some(read_icon("assets/icons/game/experience.png".to_string()))
            }
            CommonItem::Resource(resource) => {
                let mut filename = resource.to_string();
                filename.make_ascii_lowercase();
                Some(read_icon(format!("assets/icons/game/{filename}.png",)))
            }
            CommonItem::Skill(skill) => match skill {
                Skill::Bash => Some(icon(Icon::Equipment(Equipment::Bash))),
                Skill::DoubleJump => Some(icon(Icon::Equipment(Equipment::Bounce))),
                Skill::Launch => Some(icon(Icon::Equipment(Equipment::Launch))),
                Skill::Glide => Some(icon(Icon::Equipment(Equipment::Glide))),
                Skill::WaterBreath => Some(icon(Icon::Opher(OpherIcon::WaterBreath))),
                Skill::Grenade => Some(icon(Icon::Equipment(Equipment::Grenade))),
                Skill::Grapple => Some(icon(Icon::Equipment(Equipment::Grapple))),
                Skill::Flash => Some(icon(Icon::Equipment(Equipment::Glow))),
                Skill::Spear => Some(icon(Icon::Opher(OpherIcon::Spear))),
                Skill::Regenerate => Some(icon(Icon::Equipment(Equipment::Regenerate))),
                Skill::Bow => Some(icon(Icon::Equipment(Equipment::Bow))),
                Skill::Hammer => Some(icon(Icon::Opher(OpherIcon::Hammer))),
                Skill::Sword => Some(icon(Icon::Equipment(Equipment::Sword))),
                Skill::Burrow => Some(icon(Icon::Equipment(Equipment::Burrow))),
                Skill::Dash => Some(icon(Icon::Equipment(Equipment::Dash))),
                Skill::WaterDash => Some(icon(Icon::Equipment(Equipment::WaterDash))),
                Skill::Shuriken => Some(icon(Icon::Opher(OpherIcon::Shuriken))),
                Skill::Seir => Some(icon(Icon::Equipment(Equipment::Sein))),
                Skill::Blaze => Some(icon(Icon::Opher(OpherIcon::Blaze))),
                Skill::Sentry => Some(icon(Icon::Opher(OpherIcon::Sentry))),
                Skill::Flap => Some(icon(Icon::Equipment(Equipment::Flap))),
                Skill::GladesAncestralLight => Some(read_icon(
                    "assets/icons/game/ancestrallight1.png".to_string(),
                )),
                Skill::InkwaterAncestralLight => Some(read_icon(
                    "assets/icons/game/ancestrallight2.png".to_string(),
                )),
                _ => None,
            },
            CommonItem::Shard(shard) => Some(icon(Icon::Shard(*shard))),
            CommonItem::Teleporter(_) => {
                Some(read_icon("assets/icons/game/teleporter.png".to_string()))
            }
            CommonItem::CleanWater => Some(read_icon("assets/icons/game/water.png".to_string())),
            _ => None,
        },
        _ => None,
    }
}
