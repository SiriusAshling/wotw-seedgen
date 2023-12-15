// TODO why is this in a directory?

use crate::log;
use decorum::R32;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use wotw_seedgen_assets::UberStateData;
use wotw_seedgen_data::UberIdentifier;
use wotw_seedgen_seed::{CommandZone, Operation};
use wotw_seedgen_seed_language::output::{
    CommandBoolean, CommandFloat, CommandIcon, CommandInteger, CommandString, CommandVoid, Trigger,
};

#[derive(Debug, Clone)]
pub struct UberStates {
    states: FxHashMap<UberIdentifier, UberStateEntry>,
    registered_triggers: usize,
    fallback: UberStateEntry,
}
impl UberStates {
    pub fn new(uber_state_data: &UberStateData) -> Self {
        Self {
            states: uber_state_data
                .id_lookup
                .iter()
                .filter_map(|(uber_identifier, data)| {
                    let value = match data.default_value {
                        wotw_seedgen_assets::UberStateValue::Boolean(value) => {
                            UberStateValue::Boolean(value)
                        }
                        wotw_seedgen_assets::UberStateValue::Integer(value) => {
                            UberStateValue::Integer(value)
                        }
                        wotw_seedgen_assets::UberStateValue::Float(value) => {
                            UberStateValue::Float(value.into())
                        }
                    };

                    Some((
                        *uber_identifier,
                        UberStateEntry {
                            value,
                            triggers: Default::default(),
                        },
                    ))
                })
                .collect(),
            registered_triggers: 0,
            fallback: UberStateEntry {
                value: UberStateValue::Boolean(false),
                triggers: Default::default(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct UberStateEntry {
    value: UberStateValue,
    triggers: FxHashSet<usize>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UberStateValue {
    Boolean(bool),
    Integer(i32),
    Float(R32),
}
impl UberStateValue {
    pub fn as_boolean(self) -> bool {
        match self {
            UberStateValue::Boolean(value) => value,
            _ => {
                log::warning!("Attempted to access {} UberState as Boolean", self.kind());
                Default::default()
            }
        }
    }
    pub fn as_integer(self) -> i32 {
        match self {
            UberStateValue::Integer(value) => value,
            _ => {
                log::warning!("Attempted to access {} UberState as Integer", self.kind());
                Default::default()
            }
        }
    }
    pub fn as_float(self) -> R32 {
        match self {
            UberStateValue::Float(value) => value,
            _ => {
                log::warning!("Attempted to access {} UberState as Float", self.kind());
                Default::default()
            }
        }
    }

    fn kind(self) -> &'static str {
        match self {
            UberStateValue::Boolean(_) => "Boolean",
            UberStateValue::Integer(_) => "Integer",
            UberStateValue::Float(_) => "Float",
        }
    }
}
impl PartialEq<bool> for UberStateValue {
    fn eq(&self, other: &bool) -> bool {
        self.as_boolean() == *other
    }
}
impl PartialOrd<bool> for UberStateValue {
    fn partial_cmp(&self, other: &bool) -> Option<Ordering> {
        self.as_boolean().partial_cmp(other)
    }
}
impl PartialEq<i32> for UberStateValue {
    fn eq(&self, other: &i32) -> bool {
        self.as_integer() == *other
    }
}
impl PartialOrd<i32> for UberStateValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.as_integer().partial_cmp(other)
    }
}
impl PartialEq<R32> for UberStateValue {
    fn eq(&self, other: &R32) -> bool {
        self.as_float() == *other
    }
}
impl PartialOrd<R32> for UberStateValue {
    fn partial_cmp(&self, other: &R32) -> Option<Ordering> {
        self.as_float().partial_cmp(other)
    }
}

// impl Default for UberStates {
//     // TODO we ignore the effect of seed_core here
//     fn default() -> Self {
//         Self {
//             states: DEFAULT_UBER_STATES.clone(),
//             registered_triggers: 0,
//             fallback: UberStateEntry {
//                 value: UberStateValue::Boolean(false),
//                 triggers: Default::default(),
//             },
//         }
//     }
// }
impl UberStates {
    pub fn register_trigger(&mut self, trigger: &Trigger) {
        for uber_identifier in contained_uber_identifiers(trigger) {
            match self.states.get_mut(&uber_identifier) {
                None => log::warning!("Trigger contained unknown UberState {uber_identifier}"),
                Some(entry) => {
                    entry.triggers.insert(self.registered_triggers);
                }
            }
        }
        self.registered_triggers += 1;
    }

    pub fn set<'a>(
        &'a mut self,
        uber_identifier: UberIdentifier,
        value: UberStateValue,
    ) -> impl Iterator<Item = usize> + 'a {
        match self.states.get_mut(&uber_identifier) {
            None => {
                log::warning!("Attempted to write to unknown UberState {uber_identifier}");
                self.fallback.triggers.iter().copied()
            }
            Some(entry) => {
                if entry.value != value {
                    // TODO type check maybe?
                    entry.value = value;
                    entry.triggers.iter().copied()
                } else {
                    self.fallback.triggers.iter().copied()
                }
            }
        }
    }
    pub fn get(&self, uber_identifier: UberIdentifier) -> UberStateValue {
        match self.states.get(&uber_identifier) {
            None => {
                log::warning!("Attempted to read from unknown UberState {uber_identifier}");
                self.fallback.value
            }
            Some(entry) => entry.value,
        }
    }
}

fn contained_uber_identifiers<T: ContainedUberIdentifiers>(t: &T) -> Vec<UberIdentifier> {
    let mut output = vec![];
    t.contained_uber_identifiers(&mut output);
    output
}
trait ContainedUberIdentifiers {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>);
}
impl ContainedUberIdentifiers for Trigger {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        match self {
            Trigger::Pseudo(_) => {}
            Trigger::Binding(uber_identifier) => output.push(*uber_identifier),
            Trigger::Condition(condition) => condition.contained_uber_identifiers(output),
        }
    }
}
impl<Item: ContainedUberIdentifiers, Operator> ContainedUberIdentifiers
    for Operation<Item, Operator>
{
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        self.left.contained_uber_identifiers(output);
        self.right.contained_uber_identifiers(output);
    }
}
impl ContainedUberIdentifiers for CommandBoolean {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        match self {
            CommandBoolean::CompareBoolean { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::CompareInteger { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::CompareFloat { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::CompareString { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::CompareZone { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::LogicOperation { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandBoolean::FetchBoolean { uber_identifier } => output.push(*uber_identifier),
            CommandBoolean::Constant { .. }
            | CommandBoolean::GetBoolean { .. }
            | CommandBoolean::IsInHitbox { .. }
            | CommandBoolean::RandomSpiritLightNames { .. } => {}
        }
    }
}
impl ContainedUberIdentifiers for CommandInteger {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        match self {
            CommandInteger::Arithmetic { operation } => {
                operation.contained_uber_identifiers(output)
            }
            CommandInteger::FetchInteger { uber_identifier } => output.push(*uber_identifier),
            CommandInteger::Constant { .. } | CommandInteger::GetInteger { .. } => {}
        }
    }
}
impl ContainedUberIdentifiers for CommandFloat {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        match self {
            CommandFloat::Arithmetic { operation } => operation.contained_uber_identifiers(output),
            CommandFloat::FetchFloat { uber_identifier } => output.push(*uber_identifier),
            CommandFloat::ToFloat { integer } => integer.contained_uber_identifiers(output),
            CommandFloat::Constant { .. } | CommandFloat::GetFloat { .. } => {}
        }
    }
}
impl ContainedUberIdentifiers for CommandString {
    fn contained_uber_identifiers(&self, output: &mut Vec<UberIdentifier>) {
        match self {
            CommandString::Concatenate { left, right } => {
                left.contained_uber_identifiers(output);
                right.contained_uber_identifiers(output);
            }
            CommandString::ToString { .. } => todo!(),
            CommandString::Constant { .. }
            | CommandString::GetString { .. }
            | CommandString::WorldName { .. } => {}
        }
    }
}
impl ContainedUberIdentifiers for CommandZone {
    fn contained_uber_identifiers(&self, _output: &mut Vec<UberIdentifier>) {}
}
impl ContainedUberIdentifiers for CommandIcon {
    fn contained_uber_identifiers(&self, _output: &mut Vec<UberIdentifier>) {}
}
impl ContainedUberIdentifiers for CommandVoid {
    fn contained_uber_identifiers(&self, _output: &mut Vec<UberIdentifier>) {
        // If it doesn't return anything, you can't build a condition out of it
    }
}

// TODO this blocks being generic over the file access and probably bloats the memory footprint
// lazy_static! {
//     static ref DEFAULT_UBER_STATES: FxHashMap<UberIdentifier, UberStateEntry> = UBER_STATE_DATA
//         .id_lookup
//         .iter()
//         .filter_map(|(uber_identifier, data)| {
//             let value = match data.ty {
//                 UberStateType::SerializedBooleanUberState
//                 | UberStateType::BooleanUberState
//                 | UberStateType::SavePedestalUberState
//                 | UberStateType::ConditionUberState => {
//                     UberStateValue::Boolean(data.default_value != 0.)
//                 }
//                 UberStateType::SerializedByteUberState
//                 | UberStateType::ByteUberState
//                 | UberStateType::SerializedIntUberState
//                 | UberStateType::IntUberState
//                 | UberStateType::CountUberState => {
//                     UberStateValue::Integer(data.default_value as i32)
//                 }
//                 UberStateType::SerializedFloatUberState | UberStateType::VirtualUberState => {
//                     UberStateValue::Float(data.default_value.into())
//                 }
//                 UberStateType::PlayerUberStateDescriptor => {
//                     return None;
//                 }
//             };

//             Some((
//                 *uber_identifier,
//                 UberStateEntry {
//                     value,
//                     triggers: Default::default(),
//                 },
//             ))
//         })
//         .collect();
// }
