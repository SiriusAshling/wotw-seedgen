use super::{
    cost::Cost, item_pool::ItemPool, spirit_light::SpiritLightProvider, Seed, SEED_FAILED_MESSAGE,
};
use crate::{
    common_item::CommonItem,
    constants::{KEYSTONE_DOORS, PREFERRED_SPAWN_SLOTS, SPAWN_SLOTS, UNSHARED_ITEMS},
    filter_redundancies,
    inventory::Inventory,
    log::{trace, warning},
    node_condition, node_trigger,
    orbs::OrbVariants,
    SeedSpoiler, World,
};
use itertools::Itertools;
#[cfg(any(feature = "log", test))]
use ordered_float::OrderedFloat;
use rand::{
    distributions::Uniform,
    prelude::Distribution,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashMap;
use std::{iter, mem, ops::RangeFrom};
use wotw_seedgen_assembly::{compile_intermediate_output, ClientEvent, Icon, Spawn};
use wotw_seedgen_data::{uber_identifier, Equipment, MapIcon, OpherIcon, Skill, UberIdentifier};
use wotw_seedgen_logic_language::output::{Node, Requirement};
use wotw_seedgen_seed_language::{
    compile,
    output::{
        CommandInteger, CommandString, CommandVoid, CompilerOutput, Event, StringOrPlaceholder,
        Trigger,
    },
};

pub(crate) fn generate_placements(
    rng: &mut Pcg64Mcg,
    worlds: Vec<(World, CompilerOutput)>,
) -> Result<Seed, String> {
    let mut context = Context::new(rng, worlds);

    context.preplacements();

    #[cfg(any(feature = "log", test))]
    let mut step = 1usize..;
    loop {
        trace!("Placement step #{}", step.next().unwrap_or_default());
        context.update_reached();
        if context.everything_reached() {
            trace!("All locations reached");
            context.place_remaining();
            break;
        }
        context.force_keystones();
        if !context.place_random() {
            trace!("Placing forced progression");
            if let Some((target_world_index, progression)) = context.choose_progression() {
                context.place_forced(target_world_index, progression);
            }
        }
    }

    Ok(context.finish())
}

struct Context<'graph, 'settings> {
    rng: Pcg64Mcg,
    worlds: Vec<WorldContext<'graph, 'settings>>,
    /// next multiworld uberState id to use
    multiworld_state_index: RangeFrom<i32>,
}
struct WorldContext<'graph, 'settings> {
    rng: Pcg64Mcg,
    world: World<'graph, 'settings>,
    output: CompilerOutput,
    /// world index of this world
    #[cfg_attr(not(any(feature = "log", test)), allow(unused))]
    index: usize,
    /// remaining items to place
    item_pool: ItemPool,
    /// generates appropriate spirit light amounts
    spirit_light_provider: SpiritLightProvider,
    /// all remaining nodes which need to be assigned random placements
    needs_placement: Vec<&'graph Node>,
    /// nodes which have been reached but explicitely haven't been asigned a placement yet to leave space for later progressions
    placeholders: Vec<&'graph Node>,
    /// reached nodes at the start of the current placement step
    reached: Vec<&'graph Node>,
    /// unmet requirements at the start of the current placement step
    progressions: Vec<(&'graph Requirement, OrbVariants)>,
    /// indices into `needs_placement` for nodes that are reachable and may be used for placements in this step
    reached_needs_placement: Vec<usize>,
    /// indices into `needs_placement` for nodes that have received a placement and should be removed before the next placement step
    received_placement: Vec<usize>,
    /// number of nodes in `reached` that can give items
    reached_item_locations: usize,
    /// number of remaining allowed placements on spawn
    spawn_slots: usize,
    // TODO is this still needed for multiworld quality?
    /// number of remaining placements that should not be placed outside of the own world
    unshared_items: usize,
    /// generates random factors to modify shop prices with
    price_distribution: Uniform<f32>,
}

impl<'graph, 'settings> Context<'graph, 'settings> {
    fn new(rng: &mut Pcg64Mcg, worlds: Vec<(World<'graph, 'settings>, CompilerOutput)>) -> Self {
        Self {
            rng: Pcg64Mcg::from_rng(&mut *rng).expect(SEED_FAILED_MESSAGE),
            worlds: worlds
                .into_iter()
                .enumerate()
                .map(|(index, (world, output))| WorldContext::new(rng, world, output, index))
                .collect(),
            multiworld_state_index: 0..,
        }
    }

    fn preplacements(&mut self) {
        trace!("Generating preplacements");

        for world_context in &mut self.worlds {
            world_context.preplacements();
        }
    }

    fn update_reached(&mut self) {
        trace!("Checking reached locations");
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
            let owned_keystones = world_context.world.inventory().keystones;
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
                .sum::<usize>();
            if required_keystones <= owned_keystones {
                continue;
            }

            trace!(
                "Placing {} keystones for World {world_index} to avoid keylocks",
                required_keystones - owned_keystones
            );
            for _ in owned_keystones..required_keystones {
                self.place_command(compile::keystone(), world_index);
            }
        }
    }

    fn place_remaining(&mut self) {
        trace!("Placing remaining items");
        for target_world_index in 0..self.worlds.len() {
            for command in self.worlds[target_world_index]
                .item_pool
                .drain_random(&mut self.rng)
                .collect::<Vec<_>>()
            {
                self.place_command(command, target_world_index);
            }
        }
        for world_context in &mut self.worlds {
            world_context.fill_remaining();
        }
    }

    fn place_random(&mut self) -> bool {
        let mut any_placed = false;
        for origin_world_index in 0..self.worlds.len() {
            let needs_random_placement = self.worlds[origin_world_index].reserve_placeholders();
            for node in needs_random_placement {
                any_placed = true;
                let origin_world = &mut self.worlds[origin_world_index];
                let placements_remaining = origin_world.placements_remaining();
                let place_spirit_light = self.rng.gen_bool(f64::max(
                    1. - origin_world.item_pool.len() as f64 / placements_remaining as f64,
                    0.,
                ));

                let (target_world_index, command) = if place_spirit_light {
                    let batch = origin_world
                        .spirit_light_provider
                        .take(placements_remaining);
                    (
                        origin_world_index,
                        compile::spirit_light(
                            CommandInteger::Constant {
                                value: batch as i32,
                            },
                            &mut self.rng,
                        ),
                    )
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

                let name = self.name(&command, origin_world_index, target_world_index);
                self.place_command_at(command, name, node, origin_world_index, target_world_index);
            }
        }
        any_placed
    }

    fn choose_progression(&mut self) -> Option<(usize, Inventory)> {
        let slots = self.progression_slots();
        let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
        world_indices.sort_by_key(|index| self.worlds[*index].placements_remaining());

        for target_world_index in world_indices {
            if let Some(progression) = self.worlds[target_world_index].choose_progression(slots) {
                return Some((target_world_index, progression));
            }
        }

        trace!(
            "Unable to find any possible forced progression. Unreached locations:\n{}",
            self.worlds
                .iter()
                .map(|world_context| format!(
                    "World {}: {}",
                    world_context.index,
                    world_context
                        .needs_placement
                        .iter()
                        .map(|node| node.identifier())
                        .format(", ")
                ))
                .format("\n")
        );

        self.flush_item_pool();
        None
    }

    fn progression_slots(&self) -> usize {
        self.worlds
            .iter()
            .map(|world_context| world_context.progression_slots())
            .sum()
    }

    fn flush_item_pool(&mut self) {
        trace!("Placing items which modify uberStates to attempt recovery");

        todo!()
    }

    fn place_forced(&mut self, target_world_index: usize, progression: Inventory) {
        let Inventory {
            spirit_light,
            gorlek_ore,
            keystones,
            shard_slots,
            health,
            energy,
            skills,
            shards,
            teleporters,
            clean_water,
            weapon_upgrades,
        } = progression;

        self.worlds[target_world_index].place_spirit_light(spirit_light);
        iter::repeat_with(compile::gorlek_ore)
            .take(gorlek_ore)
            .chain(iter::repeat_with(compile::keystone).take(keystones))
            .chain(iter::repeat_with(compile::shard_slot).take(shard_slots))
            .chain(iter::repeat_with(compile::health_fragment).take(health / 5))
            .chain(iter::repeat_with(compile::energy_fragment).take((energy * 2.) as usize))
            .chain(skills.into_iter().map(compile::skill))
            .chain(shards.into_iter().map(compile::shard))
            .chain(teleporters.into_iter().map(compile::teleporter))
            .chain(clean_water.then(compile::clean_water))
            .chain(weapon_upgrades.into_iter().map(compile::weapon_upgrade))
            .for_each(|command| self.place_command(command, target_world_index));
    }

    fn place_command(&mut self, command: CommandVoid, target_world_index: usize) {
        trace!("Placing {command} for World {target_world_index}");
        let origin_world_index = self.choose_origin_world(&command, target_world_index);
        let name = self.name(&command, origin_world_index, target_world_index);
        let origin_world = &mut self.worlds[origin_world_index];
        match origin_world.choose_placement_node(&command) {
            None => {
                if origin_world.spawn_slots > 0 {
                    origin_world.spawn_slots -= 1;
                    self.push_command(
                        Trigger::ClientEvent(ClientEvent::Spawn),
                        command,
                        name,
                        origin_world_index,
                        target_world_index,
                    );
                } else {
                    panic!("Not enough space to place item"); // TODO this can actually happen, should maybe be a warning
                }
            }
            Some(node) => {
                self.place_command_at(command, name, node, origin_world_index, target_world_index);
            }
        }
    }

    // TODO might be worth to do some single-world happy paths?
    fn choose_origin_world(&mut self, command: &CommandVoid, target_world_index: usize) -> usize {
        trace!("Choosing origin World for {command}");

        if is_spirit_light(command) {
            trace!("{command} is a spirit light item, chose origin World {target_world_index} to avoid sharing");
            return target_world_index;
        }

        if self.worlds[target_world_index].unshared_items > 0 {
            trace!("World {target_world_index} is not allowed to share items yet, chose origin World {target_world_index}");
            self.worlds[target_world_index].unshared_items -= 1;
            target_world_index
        } else {
            let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
            world_indices.shuffle(&mut self.rng);
            let origin_world_index = world_indices
                .iter()
                .find(|index| !self.worlds[**index].reached_needs_placement.is_empty())
                .copied()
                .or_else(|| {
                    world_indices
                        .into_iter()
                        .find(|index| self.worlds[*index].spawn_slots > 0)
                })
                .unwrap(); // TODO handle
            trace!("Chose origin world {origin_world_index} randomly");
            origin_world_index
        }
    }

    fn choose_target_world(&mut self, origin_world_index: usize) -> usize {
        trace!("Choosing target World for item from World {origin_world_index}");
        if self.worlds[origin_world_index].unshared_items > 0 {
            trace!("World {origin_world_index} is not allowed to share items yet, chose target World {origin_world_index}");
            self.worlds[origin_world_index].unshared_items -= 1;
            origin_world_index
        } else {
            let mut world_indices = (0..self.worlds.len()).collect::<Vec<_>>();
            world_indices.shuffle(&mut self.rng);
            let target_world_index = world_indices
                .into_iter()
                .find_or_last(|index| !self.worlds[*index].item_pool.is_empty())
                .unwrap(); // TODO handle

            trace!("Chose target World {target_world_index}");
            target_world_index
        }
    }

    fn name(
        &self,
        command: &CommandVoid,
        origin_world_index: usize,
        target_world_index: usize,
    ) -> CommandString {
        let name = self.worlds[target_world_index].name(command);
        if origin_world_index == target_world_index {
            CommandString::Constant { value: name }
        } else {
            let right = match name {
                StringOrPlaceholder::Value(value) => CommandString::Constant {
                    value: format!("'s {value}").into(),
                },
                placeholder => CommandString::Concatenate {
                    left: Box::new(CommandString::Constant { value: "'s".into() }),
                    right: Box::new(CommandString::Constant { value: placeholder }),
                },
            };

            CommandString::Concatenate {
                left: Box::new(CommandString::WorldName {
                    index: target_world_index,
                }),
                right: Box::new(right),
            }
        }
    }

    fn place_command_at(
        &mut self,
        command: CommandVoid,
        name: CommandString,
        node: &Node,
        origin_world_index: usize,
        target_world_index: usize,
    ) {
        trace!(
            "Placing {command} for World {target_world_index} at {} in World {origin_world_index}",
            node.identifier()
        );

        self.worlds[origin_world_index].map_icon(node, &command, name.clone());

        let uber_identifier = node.uber_identifier().unwrap();
        if uber_identifier.is_shop() {
            self.worlds[origin_world_index].shop_item_data(&command, uber_identifier, name.clone());
        }

        self.push_command(
            node_trigger(node).unwrap(),
            command,
            name,
            origin_world_index,
            target_world_index,
        );
    }

    fn push_command(
        &mut self,
        trigger: Trigger,
        command: CommandVoid,
        name: CommandString,
        origin_world_index: usize,
        target_world_index: usize,
    ) {
        if origin_world_index == target_world_index {
            self.worlds[origin_world_index].push_command(trigger, command);
        } else {
            let uber_identifier = self.multiworld_state();
            self.worlds[origin_world_index].push_command(
                trigger,
                CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::QueuedMessage {
                            id: None,
                            priority: false,
                            message: name,
                            timeout: None,
                        },
                        compile::set_boolean_value(uber_identifier, true),
                    ],
                },
            );
            self.worlds[target_world_index].push_command(
                Trigger::Binding(uber_identifier), // this is server synced and can't change to false
                command,
            );
        }
    }

    fn multiworld_state(&mut self) -> UberIdentifier {
        UberIdentifier {
            group: 12,
            member: self.multiworld_state_index.next().unwrap(),
        }
    }

    fn finish(self) -> Seed {
        Seed {
            worlds: self
                .worlds
                .into_iter()
                .map(|world_context| {
                    let (mut seed, icons) = compile_intermediate_output(world_context.output);
                    assert!(icons.is_empty(), "custom icons in seedgen aren't supported"); // TODO
                    let spawn = &world_context.world.graph.nodes[world_context.world.spawn];
                    seed.spawn = Spawn {
                        position: *spawn.position().unwrap(),
                        identifier: spawn.identifier().to_string(),
                    };
                    seed
                })
                .collect(),
            spoiler: SeedSpoiler {
                // TODO spoiler
                spawns: vec![],
                groups: vec![],
            },
        }
    }
}

impl<'graph, 'settings> WorldContext<'graph, 'settings> {
    fn new(
        rng: &mut Pcg64Mcg,
        world: World<'graph, 'settings>,
        mut output: CompilerOutput,
        index: usize,
    ) -> Self {
        let mut item_pool = ItemPool::default();

        for (command, amount) in mem::take(&mut output.item_pool_changes) {
            item_pool.change(command, amount);
        }

        let mut needs_placement = total_reach_check(&world, &output, &item_pool);
        // TODO optimize based on shape of events, many of which can't possibly be loc_data events
        needs_placement.retain(|node| node.can_place()
            && !output.events.iter().any(|event|
                matches!(&event.trigger, Trigger::Condition(condition) if Some(condition) == node_condition(node).as_ref())
            )
        );

        Self {
            rng: Pcg64Mcg::from_rng(&mut *rng).expect(SEED_FAILED_MESSAGE),
            world,
            output,
            index,
            item_pool,
            spirit_light_provider: SpiritLightProvider::new(20000, rng), // TODO how should !add(spirit_light(100)) behave?
            needs_placement,
            placeholders: Default::default(),
            reached: Default::default(),
            progressions: Default::default(),
            reached_needs_placement: Default::default(),
            received_placement: Default::default(),
            reached_item_locations: Default::default(),
            spawn_slots: SPAWN_SLOTS,
            unshared_items: UNSHARED_ITEMS,
            price_distribution: Uniform::new_inclusive(0.75, 1.25),
        }
    }

    fn preplacements(&mut self) {
        trace!("[World {}] Generating preplacements", self.index);

        self.hi_sigma();

        let mut zone_needs_placement = FxHashMap::default();
        for (command, zone) in mem::take(&mut self.output.preplacements) {
            let nodes = zone_needs_placement.entry(zone).or_insert_with(|| {
                self.needs_placement
                    .iter()
                    .enumerate()
                    .filter(|(_, node)| node.zone() == Some(zone))
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>()
            });
            if nodes.is_empty() {
                warning!(
                    "[World {}] Failed to preplace {command} in {zone} since no free placement location was available",
                    self.index,
                );
            }
            // We prefer generating indices over shuffling the nodes because usually there aren't many zone preplacements (relics)
            let node_index = nodes.swap_remove(self.rng.gen_range(0..nodes.len()));
            // TODO shouldn't this remove the node from needs_placement?
            let node = self.needs_placement[node_index];
            trace!(
                "[World {}] Preplaced {command} at {}",
                self.index,
                node.identifier()
            );
            self.push_command(node_trigger(node).unwrap(), command);
            self.received_placement.push(node_index);
        }
    }

    fn hi_sigma(&mut self) {
        let node = self
            .needs_placement
            .swap_remove(self.rng.gen_range(0..self.needs_placement.len())); // TODO handle empty
        trace!(
            "[World {}] Placed something very important at {}",
            self.index,
            node.identifier()
        );
        let command = compile::spirit_light(CommandInteger::Constant { value: 1 }, &mut self.rng);
        self.push_command(node_trigger(node).unwrap(), command);
    }

    fn update_reached(&mut self) {
        trace!("[World {}] Checking reached locations", self.index);

        let mut received_placement = mem::take(&mut self.received_placement);
        received_placement.sort();
        for node_index in received_placement.into_iter().rev() {
            self.needs_placement.swap_remove(node_index);
        }

        let reached_locations = self.world.reached_and_progressions();
        self.reached = reached_locations.reached;
        self.progressions = reached_locations.progressions;
        self.reached_needs_placement = self
            .needs_placement
            .iter()
            .enumerate()
            .filter(|(_, node)| self.reached.contains(node))
            .map(|(index, _)| index)
            .collect();
        self.reached_item_locations = self.reached.iter().filter(|node| node.can_place()).count();
        trace!(
            "[World {}] Reached locations that need placements: {}",
            self.index,
            self.reached_needs_placement
                .iter()
                .map(|index| self.needs_placement[*index].identifier())
                .join(", ")
        );
    }

    fn placements_remaining(&self) -> usize {
        self.needs_placement.len() - self.received_placement.len()
    }

    fn reserve_placeholders(&mut self) -> Vec<&'graph Node> {
        self.received_placement
            .extend(self.reached_needs_placement.clone());
        let desired_placeholders = usize::max(
            usize::max(3, self.placeholders.len()),
            self.reached_needs_placement.len() / 3,
        );
        let new_placeholders = usize::min(desired_placeholders, self.reached_needs_placement.len());
        let kept_placeholders = usize::min(
            desired_placeholders - new_placeholders,
            self.placeholders.len(),
        );
        let released_placeholders = self.placeholders.split_off(kept_placeholders);
        let placeholders = self
            .reached_needs_placement
            .split_off(self.reached_needs_placement.len() - new_placeholders)
            .into_iter()
            .map(|index| self.needs_placement[index]);
        self.placeholders.extend(placeholders);
        self.placeholders.shuffle(&mut self.rng);
        trace!(
            "[World {}] Keeping {} placeholders: {}",
            self.index,
            self.placeholders.len(),
            self.placeholders
                .iter()
                .map(|node| node.identifier())
                .format(", ")
        );
        mem::take(&mut self.reached_needs_placement)
            .into_iter()
            .map(|index| self.needs_placement[index])
            .chain(released_placeholders)
            .collect()
    }

    fn progression_slots(&self) -> usize {
        self.reached_needs_placement.len() + self.placeholders.len()
    }

    fn choose_progression(&mut self, slots: usize) -> Option<Inventory> {
        trace!("[World {}] Attempting forced progression", self.index);

        let world_slots = self.progression_slots();
        let mut progressions = mem::take(&mut self.progressions)
            .into_iter()
            .flat_map(|(requirement, best_orbs)| {
                self.world.player.solutions(
                    requirement,
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

        #[cfg_attr(not(any(feature = "log", test)), allow(unused_mut))]
        let mut weights = progressions
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

                let begrudgingly_used_slots = (inventory.item_count()
                    + (SPAWN_SLOTS - PREFERRED_SPAWN_SLOTS))
                    .saturating_sub(slots);
                if begrudgingly_used_slots > 0 {
                    weight *= (0.3_f32).powf(begrudgingly_used_slots as f32);
                }

                (index, weight)
            })
            .collect::<Vec<_>>();

        #[cfg(any(feature = "log", test))]
        {
            weights.sort_unstable_by(|(_, a), (_, b)| OrderedFloat(*b).cmp(&OrderedFloat(*a)));
            let weight_sum = weights.iter().map(|(_, weight)| weight).sum::<f32>();
            let options = weights.iter().map(|(index, weight)| {
                let inventory = &progressions[*index];
                let chance = ((*weight / weight_sum) * 1000.).round() * 0.1;
                format!("{chance}%: {inventory}")
            });
            trace!(
                "[World {}] Options for forced progression:\n{}",
                self.index,
                options.format("\n")
            );
        }

        let index = weights
            .choose_weighted(&mut self.rng, |(_, weight)| *weight)
            .ok()?
            .0; // TODO handle
        let progression = progressions.swap_remove(index);
        trace!(
            "[World {}] Chose forced progression: {progression}",
            self.index
        );

        Some(progression)
    }

    fn place_spirit_light(&mut self, mut amount: usize) {
        while amount > 0 {
            let batch = self.spirit_light_provider.take(self.placements_remaining());
            amount -= batch;
            let command = compile::spirit_light(
                CommandInteger::Constant {
                    value: batch as i32,
                },
                &mut self.rng,
            );
            let node = self.choose_placement_node(&command).unwrap();
            trace!(
                "[World {}] Placing {command} at {}",
                self.index,
                node.identifier()
            );
            self.push_command(node_trigger(node).unwrap(), command);
        }
    }

    fn choose_placement_node(&mut self, command: &CommandVoid) -> Option<&'graph Node> {
        let is_spirit_light = is_spirit_light(command);
        if is_spirit_light {
            self.reached_needs_placement
                .iter()
                .enumerate()
                .filter(|(_, node_index)| {
                    !self.needs_placement[**node_index]
                        .uber_identifier()
                        .unwrap()
                        .is_shop()
                })
                .map(|(index, _)| index)
                .choose(&mut self.rng) // TODO shuffle instead?
        } else {
            (!self.reached_needs_placement.is_empty())
                .then(|| self.rng.gen_range(0..self.reached_needs_placement.len()))
        }
        .map(|index| {
            let node_index = self.reached_needs_placement.swap_remove(index);
            trace!(
                "[World {}] Choose {} as placement location for {command}",
                self.index,
                self.needs_placement[node_index].identifier()
            );
            self.received_placement.push(node_index);
            self.needs_placement[node_index]
        })
        .or_else(|| {
            if is_spirit_light {
                let (index, _) = self
                    .placeholders
                    .iter()
                    .enumerate()
                    .find(|(_, node)| !node.uber_identifier().unwrap().is_shop())?;
                Some(self.placeholders.swap_remove(index))
            } else {
                self.placeholders.pop()
            }
        })
    }

    fn map_icon(&mut self, node: &Node, command: &CommandVoid, label: CommandString) {
        let icon = self
            .output
            .item_metadata
            .get(command)
            .and_then(|metadata| metadata.map_icon)
            .unwrap_or_else(|| {
                CommonItem::from_command(command)
                    .into_iter()
                    .next()
                    .map_or(MapIcon::QuestItem, |common_item| common_item.map_icon())
            });

        self.on_load(CommandVoid::SetSpoilerMapIcon {
            location: node.identifier().to_string(),
            icon,
            label,
        });
    }

    fn name(&self, command: &CommandVoid) -> StringOrPlaceholder {
        self.output
            .item_metadata
            .get(command)
            .and_then(|metadata| metadata.name.clone())
            .unwrap_or_else(|| command.to_string().into()) // TODO to_string usages in here do not seem reasonable?
    }

    fn on_load(&mut self, command: CommandVoid) {
        self.push_command(Trigger::ClientEvent(ClientEvent::Reload), command);
    }

    fn shop_item_data(
        &mut self,
        command: &CommandVoid,
        uber_identifier: UberIdentifier,
        name: CommandString,
    ) {
        let (price, description, icon) = self
            .output
            .item_metadata
            .get(command)
            .cloned()
            .map_or((None, None, None), |metadata| {
                (metadata.price, metadata.description, metadata.icon)
            });

        let price = price.unwrap_or_else(|| CommandInteger::Constant {
            value: self.shop_price(command),
        });
        let icon = icon.or_else(|| default_icon(command));

        let mut commands = vec![
            CommandVoid::SetShopItemPrice {
                uber_identifier,
                price,
            },
            CommandVoid::SetShopItemName {
                uber_identifier,
                name,
            },
        ];
        if let Some(description) = description {
            commands.push(CommandVoid::SetShopItemDescription {
                uber_identifier,
                description,
            })
        }
        if let Some(icon) = icon {
            commands.push(CommandVoid::SetShopItemIcon {
                uber_identifier,
                icon,
            })
        }

        self.on_load(CommandVoid::Multi { commands });
    }

    fn shop_price(&mut self, command: &CommandVoid) -> i32 {
        let common_items = CommonItem::from_command(command);

        if matches!(common_items.as_slice(), [CommonItem::Skill(Skill::Blaze)]) {
            return 420;
        }

        let base_price = if common_items.is_empty() {
            200.
        } else {
            common_items
                .into_iter()
                .map(|common_item| common_item.shop_price())
                .sum()
        };

        (base_price * self.price_distribution.sample(&mut self.rng)).round() as i32
    }

    fn fill_remaining(&mut self) {
        trace!(
            "[World {}] Filling remaining locations with spirit light",
            self.index
        );

        let mut needs_placement = mem::take(&mut self.needs_placement);
        needs_placement.extend(mem::take(&mut self.placeholders));
        needs_placement.shuffle(&mut self.rng);

        for (placements_remaining, node) in needs_placement.into_iter().enumerate().rev() {
            let uber_identifier = node.uber_identifier().unwrap();
            let command = if uber_identifier.is_shop() {
                // TODO warn and also try to avoid
                compile::gorlek_ore()
            } else {
                compile::spirit_light(
                    CommandInteger::Constant {
                        value: self.spirit_light_provider.take(placements_remaining) as i32,
                    },
                    &mut self.rng,
                )
            };
            trace!(
                "[World {}] Placing {command} at {}",
                self.index,
                node.identifier()
            );
            let name = CommandString::Constant {
                value: self.name(&command),
            };
            self.shop_item_data(&command, uber_identifier, name.clone());
            self.push_command(node_trigger(node).unwrap(), command)
        }
        // TODO unreachable items that should be filled
    }

    fn push_command(&mut self, trigger: Trigger, command: CommandVoid) {
        self.world.uber_states.register_trigger(&trigger);
        self.world.simulate(&command, &self.output);
        self.output.events.push(Event { trigger, command });
    }
}

fn total_reach_check<'graph>(
    world: &World<'graph, '_>,
    output: &CompilerOutput,
    item_pool: &ItemPool,
) -> Vec<&'graph Node> {
    let mut world = world.clone();
    for command in item_pool.clone().drain() {
        world.simulate(&command, output);
    }
    world.reached()
}

fn is_spirit_light(command: &CommandVoid) -> bool {
    matches!(
        command,
        CommandVoid::StoreInteger {
            uber_identifier: uber_identifier::SPIRIT_LIGHT,
            ..
        }
    )
}

fn default_icon(command: &CommandVoid) -> Option<Icon> {
    CommonItem::from_command(command)
        .into_iter()
        .next()
        .and_then(|common_item| match common_item {
            CommonItem::SpiritLight(_) => {
                Some(Icon::File("assets/icons/game/experience.png".to_string()))
            }
            CommonItem::GorlekOre => {
                Some(Icon::File("assets/icons/game/gorlekore.png".to_string()))
            }
            CommonItem::Keystone => Some(Icon::File("assets/icons/game/keystone.png".to_string())),
            CommonItem::ShardSlot => {
                Some(Icon::File("assets/icons/game/shardslot.png".to_string()))
            }
            CommonItem::HealthFragment => Some(Icon::File(
                "assets/icons/game/healthfragment.png".to_string(),
            )),
            CommonItem::EnergyFragment => Some(Icon::File(
                "assets/icons/game/energyfragment.png".to_string(),
            )),
            CommonItem::Skill(skill) => match skill {
                Skill::Bash => Some(Icon::Equipment(Equipment::Bash)),
                Skill::DoubleJump => Some(Icon::Equipment(Equipment::Bounce)),
                Skill::Launch => Some(Icon::Equipment(Equipment::Launch)),
                Skill::Glide => Some(Icon::Equipment(Equipment::Glide)),
                Skill::WaterBreath => Some(Icon::Opher(OpherIcon::WaterBreath)),
                Skill::Grenade => Some(Icon::Equipment(Equipment::Grenade)),
                Skill::Grapple => Some(Icon::Equipment(Equipment::Grapple)),
                Skill::Flash => Some(Icon::Equipment(Equipment::Glow)),
                Skill::Spear => Some(Icon::Opher(OpherIcon::Spear)),
                Skill::Regenerate => Some(Icon::Equipment(Equipment::Regenerate)),
                Skill::Bow => Some(Icon::Equipment(Equipment::Bow)),
                Skill::Hammer => Some(Icon::Opher(OpherIcon::Hammer)),
                Skill::Sword => Some(Icon::Equipment(Equipment::Sword)),
                Skill::Burrow => Some(Icon::Equipment(Equipment::Burrow)),
                Skill::Dash => Some(Icon::Equipment(Equipment::Dash)),
                Skill::WaterDash => Some(Icon::Equipment(Equipment::WaterDash)),
                Skill::Shuriken => Some(Icon::Opher(OpherIcon::Shuriken)),
                Skill::Seir => Some(Icon::Equipment(Equipment::Sein)),
                Skill::Blaze => Some(Icon::Opher(OpherIcon::Blaze)),
                Skill::Sentry => Some(Icon::Opher(OpherIcon::Sentry)),
                Skill::Flap => Some(Icon::Equipment(Equipment::Flap)),
                Skill::GladesAncestralLight => Some(Icon::File(
                    "assets/icons/game/ancestrallight1.png".to_string(),
                )),
                Skill::InkwaterAncestralLight => Some(Icon::File(
                    "assets/icons/game/ancestrallight2.png".to_string(),
                )),
                _ => None,
            },
            CommonItem::Shard(shard) => Some(Icon::Shard(shard)),
            CommonItem::Teleporter(_) => {
                Some(Icon::File("assets/icons/game/teleporter.png".to_string()))
            }
            CommonItem::CleanWater => Some(Icon::File("assets/icons/game/water.png".to_string())),
            _ => None,
        })
}
