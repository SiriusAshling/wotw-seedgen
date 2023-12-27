mod command;
// TODO remove
// mod display;
mod event;
pub(crate) mod intermediate;
mod operation;

pub use command::{
    Command, CommandBoolean, CommandFloat, CommandInteger, CommandString, CommandVoid, CommandZone,
};
pub use event::{ClientEvent, Event, Trigger};
pub use operation::{ArithmeticOperator, Comparator, EqualityComparator, LogicOperator, Operation};

use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use wotw_seedgen_data::{
    Equipment, GromIcon, LupoIcon, MapIcon, OpherIcon, Position, Shard, TuleyIcon,
};

// TODO check all the public derives
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilerOutput {
    pub spawn: Option<Position>,
    pub events: Vec<Event>,
    pub command_lookup: Vec<Command>,
    pub icons: Vec<(String, Vec<u8>)>, // TODO poor memory
    pub flags: FxHashSet<StringOrPlaceholder>,
    pub item_pool_changes: FxHashMap<CommandVoid, i32>,
    pub item_metadata: FxHashMap<CommandVoid, ItemMetadata>,
    pub logical_state_sets: FxHashSet<String>,
    pub preplacements: Vec<(CommandVoid, wotw_seedgen_data::Zone)>,
    pub success: bool,
    pub debug: Option<DebugOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DebugOutput {
    pub snippets: FxHashMap<String, SnippetDebugOutput>,
    pub callbacks: FxHashMap<String, FxHashMap<String, usize>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SnippetDebugOutput {
    pub variables: FxHashMap<String, String>,
    pub function_indices: FxHashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemMetadata {
    /// Generic name used when sending the item to another world and in the spoiler
    pub name: Option<StringOrPlaceholder>,
    /// Base price used when placed in a shop
    pub price: Option<CommandInteger>,
    /// Description used when placed in a shop
    pub description: Option<CommandString>,
    /// Icon used when placed in a shop
    pub icon: Option<Icon>,
    /// Map Icon used in the spoiler map
    pub map_icon: Option<MapIcon>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringOrPlaceholder {
    Value(String),
    ZoneOfPlaceholder(Box<Command>),
    ItemOnPlaceholder(Box<Trigger>),
    CountInZonePlaceholder(Vec<Command>, wotw_seedgen_data::Zone),
}
impl From<String> for StringOrPlaceholder {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}
impl From<&str> for StringOrPlaceholder {
    fn from(value: &str) -> Self {
        Self::Value(value.to_string())
    }
}

/// Descriptor for an icon
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Icon {
    Shard(Shard),
    Equipment(Equipment),
    Opher(OpherIcon),
    Lupo(LupoIcon),
    Grom(GromIcon),
    Tuley(TuleyIcon),
    File(String),
    Bundle(String),
}
