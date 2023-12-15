// TODO pub everything :/
mod graph;
mod player;
mod reached;
mod simulate;
mod uber_states;

pub use player::Player;
pub use simulate::Simulate;
pub use uber_states::UberStates;

pub(crate) use graph::{node_condition, node_trigger};
pub(crate) use reached::ReachedLocations;
// TODO remove maybe
pub(crate) use player::filter_redundancies;

#[cfg(test)]
mod tests;

use crate::inventory::Inventory;

use self::reached::ReachContext;
use decorum::R32;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::smallvec;
use wotw_seedgen_data::{Resource, Shard, Skill, Teleporter, UberIdentifier, WeaponUpgrade};
use wotw_seedgen_logic_language::output::{Graph, Node};
use wotw_seedgen_seed::{ArithmeticOperator, Operation};
use wotw_seedgen_seed_language::output::{
    CommandBoolean, CommandFloat, CommandInteger, CommandVoid, CompilerOutput,
};
use wotw_seedgen_settings::WorldSettings;

// TODO A stateful reach check would have some advantages, for instance currently seedgen would not correctly account for "Grant Launch on breaking this Wall"

// TODO design interfaces instead of spamming pub(crate)?
#[derive(Debug, Clone)]
pub struct World<'graph, 'settings> {
    pub(crate) graph: &'graph Graph,
    spawn: usize,
    // TODO technically the entire inventory is already contained in the uber_states?
    pub(crate) player: Player<'settings>,
    pub(crate) uber_states: UberStates,
    pub(crate) logic_states: FxHashSet<usize>, // TODO implement
    logic_state_map: FxHashMap<UberIdentifier, Vec<usize>>,
    variables: Variables,
}
impl<'graph, 'settings> World<'graph, 'settings> {
    /// Creates a new world with the given [`Graph`] and [`WorldSettings`]
    ///
    /// Note that the player will start with an empty inventory, use [`new_spawn`] if you want the player to start with the vanilla inventory.
    ///
    /// [`new_spawn`]: World::new_spawn
    pub fn new(
        graph: &'graph Graph,
        spawn: usize,
        settings: &'settings WorldSettings,
        uber_states: UberStates,
    ) -> Self {
        World {
            graph,
            spawn,
            player: Player::new(settings),
            uber_states,
            logic_states: Default::default(),
            logic_state_map: Default::default(),
            variables: Default::default(),
        }
    }
    /// Creates a new world with the given [`Graph`] and [`WorldSettings`]
    ///
    /// Note that the player will start with the vanilla inventory of 3 energy, 30 health and 3 shard slots, use [`new`] if you want the player to start with an empty inventory.
    ///
    /// [`new`]: World::new
    pub fn new_spawn(
        graph: &'graph Graph,
        spawn: usize,
        settings: &'settings WorldSettings,
        uber_states: UberStates,
    ) -> Self {
        World {
            player: Player::new_spawn(settings),
            ..World::new(graph, spawn, settings, uber_states)
        }
    }

    pub(crate) fn reached(&mut self) -> Vec<&'graph Node> {
        let mut context = ReachContext::default();

        self.reach_recursion(self.spawn, smallvec![self.player.max_orbs()], &mut context);
        self.reached_by_teleporter(&mut context);

        context.reached_locations.reached
    }
    pub(crate) fn reached_and_progressions(&mut self) -> ReachedLocations<'graph> {
        let mut context = ReachContext::default();
        context.progression_check = true;

        self.reach_recursion(self.spawn, smallvec![self.player.max_orbs()], &mut context);
        self.reached_by_teleporter(&mut context);

        context.reached_locations
    }

    #[inline]
    pub fn simulate<T: Simulate>(&mut self, t: &T, output: &CompilerOutput) -> T::Return {
        t.simulate(self, output)
    }
    pub fn set_boolean(
        &mut self,
        uber_identifier: UberIdentifier,
        value: bool,
        output: &CompilerOutput,
    ) {
        self.simulate(
            &CommandVoid::StoreBoolean {
                uber_identifier,
                value: CommandBoolean::Constant { value },
                check_triggers: true,
            },
            output,
        );
    }
    pub fn set_integer(
        &mut self,
        uber_identifier: UberIdentifier,
        value: i32,
        output: &CompilerOutput,
    ) {
        self.simulate(
            &CommandVoid::StoreInteger {
                uber_identifier,
                value: CommandInteger::Constant { value },
                check_triggers: true,
            },
            output,
        );
    }
    pub fn set_float(
        &mut self,
        uber_identifier: UberIdentifier,
        value: R32,
        output: &CompilerOutput,
    ) {
        self.simulate(
            &CommandVoid::StoreFloat {
                uber_identifier,
                value: CommandFloat::Constant { value },
                check_triggers: true,
            },
            output,
        );
    }
    pub fn modify_integer(
        &mut self,
        uber_identifier: UberIdentifier,
        add: i32,
        output: &CompilerOutput,
    ) {
        self.simulate(
            &CommandVoid::StoreInteger {
                uber_identifier,
                value: CommandInteger::Arithmetic {
                    operation: Box::new(Operation {
                        left: CommandInteger::FetchInteger { uber_identifier },
                        operator: ArithmeticOperator::Add,
                        right: CommandInteger::Constant { value: add },
                    }),
                },
                check_triggers: true,
            },
            output,
        );
    }
    pub fn modify_float(
        &mut self,
        uber_identifier: UberIdentifier,
        add: R32,
        output: &CompilerOutput,
    ) {
        self.simulate(
            &CommandVoid::StoreFloat {
                uber_identifier,
                value: CommandFloat::Arithmetic {
                    operation: Box::new(Operation {
                        left: CommandFloat::FetchFloat { uber_identifier },
                        operator: ArithmeticOperator::Add,
                        right: CommandFloat::Constant { value: add },
                    }),
                },
                check_triggers: true,
            },
            output,
        );
    }
    // TODO these are confusing because they sidestep the inventory update. I really think it would be worth investigating whether dropping the inventory in favour of direct uberState check would be worthwhile
    #[inline]
    pub fn set_spirit_light(&mut self, value: i32, output: &CompilerOutput) {
        self.set_integer(UberIdentifier::SPIRIT_LIGHT, value, output);
    }
    #[inline]
    pub fn modify_spirit_light(&mut self, add: i32, output: &CompilerOutput) {
        self.modify_integer(UberIdentifier::SPIRIT_LIGHT, add, output);
    }
    #[inline]
    pub fn set_resource(&mut self, resource: Resource, value: i32, output: &CompilerOutput) {
        self.set_integer(resource.uber_identifier(), value, output);
    }
    #[inline]
    pub fn modify_resource(&mut self, resource: Resource, add: i32, output: &CompilerOutput) {
        self.modify_integer(resource.uber_identifier(), add, output);
    }
    #[inline]
    pub fn set_skill(&mut self, skill: Skill, value: bool, output: &CompilerOutput) {
        self.set_boolean(skill.uber_identifier(), value, output);
    }
    #[inline]
    pub fn set_shard(&mut self, shard: Shard, value: bool, output: &CompilerOutput) {
        self.set_boolean(shard.uber_identifier(), value, output);
    }
    #[inline]
    pub fn set_teleporter(&mut self, teleporter: Teleporter, value: bool, output: &CompilerOutput) {
        self.set_boolean(teleporter.uber_identifier(), value, output);
    }
    #[inline]
    pub fn set_clean_water(&mut self, value: bool, output: &CompilerOutput) {
        self.set_boolean(UberIdentifier::CLEAN_WATER, value, output);
    }
    #[inline]
    pub fn set_weapon_upgrade(
        &mut self,
        weapon_upgrade: WeaponUpgrade,
        value: bool,
        output: &CompilerOutput,
    ) {
        self.set_boolean(weapon_upgrade.uber_identifier(), value, output);
    }

    #[inline]
    pub fn inventory(&self) -> &Inventory {
        &self.player.inventory
    }

    // TODO should be possible to use an immutable reference
    // TODO inefficient to do this every time
    // pub(crate) fn collect_states(&mut self, output: &CompilerOutput) -> FxHashSet<usize> {
    //     let mut states = FxHashSet::default();

    //     for (condition, index) in self
    //         .graph
    //         .nodes
    //         .iter()
    //         .filter(|node| matches!(node, Node::State(_) | Node::LogicalState(_)))
    //         .filter_map(|node| node.condition().map(|condition| (condition, node.index())))
    //     {
    //         // TODO conceptually I'd expect any condition contained in loc or state data to never mutate the world,
    //         // or maybe more generally for commands returning values if that's reasonable
    //         if condition.simulate(self, output) {
    //             states.insert(index);
    //         }
    //     }

    //     // TODO !set commands

    //     states
    // }

    // TODO reminder that quest and similar uberStates have to behave strictly incrementally
}

#[derive(Debug, Default, Clone)]
struct Variables {
    booleans: FxHashMap<usize, bool>,
    integers: FxHashMap<usize, i32>,
    floats: FxHashMap<usize, R32>,
    strings: FxHashMap<usize, String>,
}
impl Variables {
    fn set_boolean(&mut self, id: usize, value: bool) {
        self.booleans.insert(id, value);
    }
    fn set_integer(&mut self, id: usize, value: i32) {
        self.integers.insert(id, value);
    }
    fn set_float(&mut self, id: usize, value: R32) {
        self.floats.insert(id, value);
    }
    fn set_string(&mut self, id: usize, value: String) {
        self.strings.insert(id, value);
    }
    fn get_boolean(&self, id: &usize) -> bool {
        self.booleans.get(id).copied().unwrap_or_default()
    }
    fn get_integer(&self, id: &usize) -> i32 {
        self.integers.get(id).copied().unwrap_or_default()
    }
    fn get_float(&self, id: &usize) -> R32 {
        self.floats.get(id).copied().unwrap_or_default()
    }
    fn get_string(&self, id: &usize) -> String {
        self.strings.get(id).cloned().unwrap_or_default()
    }
}
