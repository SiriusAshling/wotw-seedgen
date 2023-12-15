use super::{uber_states::UberStateValue, World};
use decorum::R32;
use std::ops::{Add, Div, Mul, Sub};
use wotw_seedgen_data::{Resource, Shard, UberIdentifier, Zone};
use wotw_seedgen_seed::{
    ArithmeticOperator, CommandZone, Comparator, EqualityComparator, Icon, LogicOperator, Operation,
};
use wotw_seedgen_seed_language::output::{
    Action, ActionCondition, Command, CommandBoolean, CommandFloat, CommandIcon, CommandInteger,
    CommandString, CommandVoid, CommonItem, CompilerOutput, StringOrPlaceholder, Trigger,
};

pub trait Simulate {
    type Return;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return;
}
impl Simulate for Action {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            Action::Command(command) => command.simulate(world, output),
            Action::Condition(condition) => condition.simulate(world, output),
            Action::Multi(multi) => multi.simulate(world, output),
        }
    }
}
impl Simulate for ActionCondition {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        if self.condition.simulate(world, output) {
            self.action.simulate(world, output);
        }
    }
}
impl<T: Simulate> Simulate for Vec<T> {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        for t in self {
            t.simulate(world, output);
        }
    }
}
impl Simulate for Command {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            Command::Boolean(command) => {
                command.simulate(world, output);
            }
            Command::Integer(command) => {
                command.simulate(world, output);
            }
            Command::Float(command) => {
                command.simulate(world, output);
            }
            Command::String(command) => {
                command.simulate(world, output);
            }
            Command::Zone(command) => {
                command.simulate(world, output);
            }
            Command::Icon(command) => {
                command.simulate(world, output);
            }
            Command::Void(command) => {
                command.simulate(world, output);
            }
            Command::Custom(command) => {
                command.simulate(world, output);
            }
        }
    }
}
impl<Item: Simulate> Simulate for Operation<Item, EqualityComparator>
where
    Item::Return: PartialEq,
{
    type Return = bool;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        let left = self.left.simulate(world, output);
        let right = self.right.simulate(world, output);
        match self.operator {
            EqualityComparator::Equal => left == right,
            EqualityComparator::NotEqual => left != right,
        }
    }
}
impl<Item: Simulate> Simulate for Operation<Item, Comparator>
where
    Item::Return: PartialEq + PartialOrd,
{
    type Return = bool;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        let left = self.left.simulate(world, output);
        let right = self.right.simulate(world, output);
        match self.operator {
            Comparator::Equal => left == right,
            Comparator::NotEqual => left != right,
            Comparator::Less => left < right,
            Comparator::LessOrEqual => left <= right,
            Comparator::Greater => left > right,
            Comparator::GreaterOrEqual => left >= right,
        }
    }
}
impl<Item: Simulate<Return = bool>> Simulate for Operation<Item, LogicOperator> {
    type Return = bool;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        let left = self.left.simulate(world, output);
        let right = self.right.simulate(world, output);
        match self.operator {
            LogicOperator::And => left && right,
            LogicOperator::Or => left || right,
        }
    }
}
impl<Item: Simulate> Simulate for Operation<Item, ArithmeticOperator>
where
    Item::Return: Add<Output = Item::Return>
        + Sub<Output = Item::Return>
        + Mul<Output = Item::Return>
        + Div<Output = Item::Return>,
{
    type Return = Item::Return;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        let left = self.left.simulate(world, output);
        let right = self.right.simulate(world, output);
        match self.operator {
            ArithmeticOperator::Add => left + right,
            ArithmeticOperator::Subtract => left - right,
            ArithmeticOperator::Multiply => left * right,
            ArithmeticOperator::Divide => left / right,
        }
    }
}
impl Simulate for CommandBoolean {
    type Return = bool;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommandBoolean::Constant { value } => *value,
            CommandBoolean::CompareBoolean { operation } => operation.simulate(world, output),
            CommandBoolean::CompareInteger { operation } => operation.simulate(world, output),
            CommandBoolean::CompareFloat { operation } => operation.simulate(world, output),
            CommandBoolean::CompareString { operation } => operation.simulate(world, output),
            CommandBoolean::CompareZone { operation } => operation.simulate(world, output),
            CommandBoolean::LogicOperation { operation } => operation.simulate(world, output),
            CommandBoolean::FetchBoolean { uber_identifier } => {
                world.uber_states.get(*uber_identifier).as_boolean()
            }
            CommandBoolean::GetBoolean { id } => world.variables.get_boolean(id),
            CommandBoolean::IsInHitbox { .. } => false,
            CommandBoolean::RandomSpiritLightNames {} => true,
        }
    }
}
impl Simulate for CommandInteger {
    type Return = i32;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommandInteger::Constant { value } => *value,
            CommandInteger::Arithmetic { operation } => operation.simulate(world, output),
            CommandInteger::FetchInteger { uber_identifier } => {
                world.uber_states.get(*uber_identifier).as_integer()
            }
            CommandInteger::GetInteger { id } => world.variables.get_integer(id),
        }
    }
}
impl Simulate for CommandFloat {
    type Return = R32;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommandFloat::Constant { value } => *value,
            CommandFloat::Arithmetic { operation } => operation.simulate(world, output),
            CommandFloat::FetchFloat { uber_identifier } => {
                world.uber_states.get(*uber_identifier).as_float()
            }
            CommandFloat::GetFloat { id } => world.variables.get_float(id),
            CommandFloat::ToFloat { integer } => (integer.simulate(world, output) as f32).into(),
        }
    }
}
impl Simulate for CommandString {
    type Return = String;

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommandString::Constant { value } => match value {
                StringOrPlaceholder::Value(value) => value.clone(),
                _ => Default::default(),
            },
            CommandString::Concatenate { left, right } => {
                left.simulate(world, output) + &right.simulate(world, output)
            }
            CommandString::GetString { id } => world.variables.get_string(id),
            CommandString::WorldName { .. } => Default::default(),
            CommandString::ToString { .. } => todo!(),
        }
    }
}
impl Simulate for CommandZone {
    type Return = Zone;

    fn simulate(&self, _world: &mut World, _output: &CompilerOutput) -> Self::Return {
        match self {
            CommandZone::Constant { value } => *value,
            CommandZone::CurrentZone {} => Zone::Void,
        }
    }
}
impl Simulate for CommandIcon {
    type Return = Icon;

    fn simulate(&self, _world: &mut World, _output: &CompilerOutput) -> Self::Return {
        match self {
            CommandIcon::Constant { value } => *value,
            CommandIcon::ReadIcon { .. } => Icon::Shard(Shard::Overcharge),
        }
    }
}
impl Simulate for CommandVoid {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommandVoid::StoreBoolean {
                uber_identifier,
                value,
                check_triggers,
            } => {
                let value = UberStateValue::Boolean(value.simulate(world, output));
                set_uber_state(world, output, *uber_identifier, value, *check_triggers);
            }
            CommandVoid::StoreInteger {
                uber_identifier,
                value,
                check_triggers,
            } => {
                let value = UberStateValue::Integer(value.simulate(world, output));
                set_uber_state(world, output, *uber_identifier, value, *check_triggers);
            }
            CommandVoid::StoreFloat {
                uber_identifier,
                value,
                check_triggers,
            } => {
                let value = UberStateValue::Float(value.simulate(world, output));
                set_uber_state(world, output, *uber_identifier, value, *check_triggers);
            }
            CommandVoid::SetBoolean { id, value } => {
                let value = value.simulate(world, output);
                world.variables.set_boolean(*id, value);
            }
            CommandVoid::SetInteger { id, value } => {
                let value = value.simulate(world, output);
                world.variables.set_integer(*id, value);
            }
            CommandVoid::SetFloat { id, value } => {
                let value = value.simulate(world, output);
                world.variables.set_float(*id, value);
            }
            CommandVoid::SetString { id, value } => {
                let value = value.simulate(world, output);
                world.variables.set_string(*id, value);
            }
            // TODO simluate more maybe?
            CommandVoid::DefineTimer { .. }
            | CommandVoid::SetKwolokStatueEnabled { .. }
            | CommandVoid::CreateWarpIcon { .. }
            | CommandVoid::DestroyWarpIcon { .. }
            | CommandVoid::Lookup { .. }
            | CommandVoid::ItemMessage { .. }
            | CommandVoid::ItemMessageWithTimeout { .. }
            | CommandVoid::PriorityMessage { .. }
            | CommandVoid::ControlledMessage { .. }
            | CommandVoid::SetMessageText { .. }
            | CommandVoid::SetMessageTimeout { .. }
            | CommandVoid::DestroyMessage { .. }
            | CommandVoid::Save { .. }
            | CommandVoid::Checkpoint { .. }
            | CommandVoid::Warp { .. }
            | CommandVoid::Equip { .. }
            | CommandVoid::Unequip { .. }
            | CommandVoid::TriggerKeybind { .. }
            | CommandVoid::EnableServerSync { .. }
            | CommandVoid::DisableServerSync { .. }
            | CommandVoid::SetSpoilerMapIcon { .. }
            | CommandVoid::SetWarpIconLabel { .. }
            | CommandVoid::SetShopItemPrice { .. }
            | CommandVoid::SetShopItemName { .. }
            | CommandVoid::SetShopItemDescription { .. }
            | CommandVoid::SetShopItemIcon { .. }
            | CommandVoid::SetShopItemHidden { .. }
            | CommandVoid::SetWheelItemName { .. }
            | CommandVoid::SetWheelItemDescription { .. }
            | CommandVoid::SetWheelItemIcon { .. }
            | CommandVoid::SetWheelItemColor { .. }
            | CommandVoid::SetWheelItemAction { .. }
            | CommandVoid::DestroyWheelItem { .. }
            | CommandVoid::SwitchWheel { .. }
            | CommandVoid::SetWheelPinned { .. }
            | CommandVoid::ClearAllWheels { .. } => {}
        }
    }
}
impl Simulate for CommonItem {
    type Return = ();

    fn simulate(&self, world: &mut World, output: &CompilerOutput) -> Self::Return {
        match self {
            CommonItem::SpiritLight(amount) => {
                world.player.inventory.spirit_light += *amount;
                world.modify_integer(UberIdentifier::SPIRIT_LIGHT, *amount, output);
            }
            CommonItem::RemoveSpiritLight(amount) => {
                world.player.inventory.spirit_light -= *amount;
                world.modify_integer(UberIdentifier::SPIRIT_LIGHT, -amount, output);
            }
            CommonItem::Resource(resource) => {
                world.player.inventory.add_resource(*resource, 1);
                world.modify_integer(resource.uber_identifier(), 1, output);
            }
            CommonItem::RemoveResource(resource) => {
                world.player.inventory.add_resource(*resource, -1);
                world.modify_integer(resource.uber_identifier(), -1, output);
            }
            CommonItem::Skill(skill) => {
                world.player.inventory.skills.insert(*skill);
                world.set_boolean(skill.uber_identifier(), true, output);
            }
            CommonItem::RemoveSkill(skill) => {
                world.player.inventory.skills.remove(skill);
                world.set_boolean(skill.uber_identifier(), false, output);
            }
            CommonItem::Shard(shard) => {
                world.player.inventory.shards.insert(*shard);
                world.set_boolean(shard.uber_identifier(), true, output);
            }
            CommonItem::RemoveShard(shard) => {
                world.player.inventory.shards.remove(shard);
                world.set_boolean(shard.uber_identifier(), false, output);
            }
            CommonItem::Teleporter(teleporter) => {
                world.player.inventory.teleporters.insert(*teleporter);
                world.set_boolean(teleporter.uber_identifier(), true, output);
            }
            CommonItem::RemoveTeleporter(teleporter) => {
                world.player.inventory.teleporters.remove(teleporter);
                world.set_boolean(teleporter.uber_identifier(), false, output);
            }
            CommonItem::CleanWater => {
                world.player.inventory.clean_water = true;
                world.set_boolean(UberIdentifier::CLEAN_WATER, true, output);
            }
            CommonItem::RemoveCleanWater => {
                world.player.inventory.clean_water = false;
                world.set_boolean(UberIdentifier::CLEAN_WATER, false, output);
            }
            CommonItem::WeaponUpgrade(weapon_upgrade) => {
                world
                    .player
                    .inventory
                    .weapon_upgrades
                    .insert(*weapon_upgrade);
                world.set_boolean(weapon_upgrade.uber_identifier(), true, output);
            }
            CommonItem::RemoveWeaponUpgrade(weapon_upgrade) => {
                world
                    .player
                    .inventory
                    .weapon_upgrades
                    .remove(weapon_upgrade);
                world.set_boolean(weapon_upgrade.uber_identifier(), false, output);
            }
        }
    }
}

fn set_uber_state(
    world: &mut World,
    output: &CompilerOutput,
    uber_identifier: UberIdentifier,
    value: UberStateValue,
    check_triggers: bool,
) {
    // TODO virtual uberstate simulation?
    if prevent_uber_state_change(world, uber_identifier, value) {
        return;
    }
    if check_triggers {
        let events = world.uber_states.set(uber_identifier, value).collect();
        uber_state_side_effects(world, output, uber_identifier, value, check_triggers);
        process_triggers(world, output, events);
    } else {
        world.uber_states.set(uber_identifier, value);
    }
}
fn process_triggers(world: &mut World, output: &CompilerOutput, events: Vec<usize>) {
    for index in events {
        let event = &output.events[index];
        if match &event.trigger {
            Trigger::Pseudo(_) => false,
            Trigger::Binding(_) => true,
            Trigger::Condition(condition) => condition.simulate(world, output),
        } {
            event.action.simulate(world, output);
        }
    }
}

const WELLSPRING_QUEST: UberIdentifier = UberIdentifier::new(937, 34641);
const KU_QUEST: UberIdentifier = UberIdentifier::new(14019, 34504);
const LUMA_FIGHT_ARENA_2: UberIdentifier = UberIdentifier::new(5377, 53480);
const LUMA_FIGHT_ARENA_1: UberIdentifier = UberIdentifier::new(5377, 1373);
const DIAMOND_IN_THE_ROUGH_CUTSCENE: UberIdentifier = UberIdentifier::new(42178, 2654);
const DIAMOND_IN_THE_ROUGH_PICKUP: UberIdentifier = UberIdentifier::new(23987, 14832);
const WELLSPRING_ESCAPE_COMPLETE: UberIdentifier = UberIdentifier::new(37858, 12379);
const TULEY_IN_GLADES: UberIdentifier = UberIdentifier::new(6, 300);
const CAT_AND_MOUSE: UberIdentifier = UberIdentifier::new(58674, 32810);
const WILLOW_STONE_BOSS_HEART: UberIdentifier = UberIdentifier::new(16155, 28478);
const WILLOW_STONE_BOSS_STATE: UberIdentifier = UberIdentifier::new(16155, 12971);
const SWORD_TREE: UberIdentifier = UberIdentifier::new(0, 100);
const RAIN_LIFTED: UberIdentifier = UberIdentifier::new(6, 401);
const VOICE: UberIdentifier = UberIdentifier::new(46462, 59806);
const STRENGTH: UberIdentifier = UberIdentifier::new(945, 49747);
const MEMORY: UberIdentifier = UberIdentifier::new(28895, 25522);
const EYES: UberIdentifier = UberIdentifier::new(18793, 63291);
const HEART: UberIdentifier = UberIdentifier::new(10289, 22102);

fn prevent_uber_state_change(
    world: &World,
    uber_identifier: UberIdentifier,
    value: UberStateValue,
) -> bool {
    match uber_identifier {
        WELLSPRING_QUEST if world.uber_states.get(WELLSPRING_QUEST) >= value => true,
        KU_QUEST if value <= 4 => true,
        _ => false,
    }
}

// This should mirror https://github.com/ori-community/wotw-rando-client/blob/dev/projects/Randomizer/uber_states/misc_handlers.cpp
fn uber_state_side_effects(
    world: &mut World,
    output: &CompilerOutput,
    uber_identifier: UberIdentifier,
    value: UberStateValue,
    check_triggers: bool,
) {
    match uber_identifier {
        LUMA_FIGHT_ARENA_2 if value == 4 => {
            set_uber_state(
                world,
                output,
                LUMA_FIGHT_ARENA_1,
                UberStateValue::Integer(4),
                check_triggers,
            );
        }
        DIAMOND_IN_THE_ROUGH_CUTSCENE if matches!(value.as_integer(), 1 | 2) => {
            set_uber_state(
                world,
                output,
                DIAMOND_IN_THE_ROUGH_CUTSCENE,
                UberStateValue::Integer(3),
                check_triggers,
            );
            set_uber_state(
                world,
                output,
                DIAMOND_IN_THE_ROUGH_PICKUP,
                UberStateValue::Boolean(true),
                check_triggers,
            );
        }
        WELLSPRING_ESCAPE_COMPLETE if value == true => {
            set_uber_state(
                world,
                output,
                WELLSPRING_QUEST,
                UberStateValue::Integer(3),
                check_triggers,
            );
        }
        WELLSPRING_QUEST if value >= 3 => {
            set_uber_state(
                world,
                output,
                TULEY_IN_GLADES,
                UberStateValue::Boolean(true),
                check_triggers,
            );
        }
        CAT_AND_MOUSE if value == 7 => {
            set_uber_state(
                world,
                output,
                CAT_AND_MOUSE,
                UberStateValue::Integer(8),
                check_triggers,
            );
        }
        WILLOW_STONE_BOSS_HEART if value == true => {
            set_uber_state(
                world,
                output,
                WILLOW_STONE_BOSS_STATE,
                UberStateValue::Integer(4),
                check_triggers,
            );
        }
        SWORD_TREE if value == true => {
            set_uber_state(
                world,
                output,
                RAIN_LIFTED,
                UberStateValue::Boolean(true),
                check_triggers,
            );
        }
        VOICE | STRENGTH | MEMORY | EYES | HEART if value == true => {
            // TODO not strictly correct but not sure what else to do
            world.modify_resource(Resource::HealthFragment, 2, output);
            world.modify_resource(Resource::EnergyFragment, 2, output);
        }
        _ => {}
    }
}
