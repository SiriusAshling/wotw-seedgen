mod common_item;
pub use common_item::*;
pub(crate) mod intermediate;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    fmt::{self, Display},
    hash::Hash,
};
use wotw_seedgen_data::{MapIcon, UberIdentifier};
use wotw_seedgen_seed::LiteralTypes;

pub type Event = wotw_seedgen_seed::Event<SnippetLiteralTypes>;
pub type Trigger = wotw_seedgen_seed::Trigger<SnippetLiteralTypes>;
pub type Action = wotw_seedgen_seed::Action<SnippetLiteralTypes>;
pub type Command = wotw_seedgen_seed::Command<SnippetLiteralTypes>;
pub type CommandBoolean = wotw_seedgen_seed::CommandBoolean<SnippetLiteralTypes>;
pub type CommandInteger = wotw_seedgen_seed::CommandInteger<SnippetLiteralTypes>;
pub type CommandFloat = wotw_seedgen_seed::CommandFloat<SnippetLiteralTypes>;
pub type CommandString = wotw_seedgen_seed::CommandString<SnippetLiteralTypes>;
pub type CommandIcon = wotw_seedgen_seed::CommandIcon<SnippetLiteralTypes>;
pub type CommandVoid = wotw_seedgen_seed::CommandVoid<SnippetLiteralTypes>;
pub type ActionCondition = wotw_seedgen_seed::ActionCondition<SnippetLiteralTypes>;
pub use wotw_seedgen_seed::{
    ArithmeticOperator, CommandZone, Comparator, EqualityComparator, Icon, LogicOperator,
    Operation, PseudoTrigger, Spawn,
};

// TODO check all the public derives
#[derive(Debug, Clone, Default)]
pub struct CompilerOutput {
    pub spawn: Option<String>,
    pub events: Vec<Event>,
    pub action_lookup: Vec<Action>,
    pub flags: FxHashSet<String>,
    pub item_pool_changes: FxHashMap<Action, i32>,
    pub item_metadata: FxHashMap<Action, ItemMetadata>,
    pub logical_state_sets: FxHashSet<String>,
    pub preplacements: Vec<(Action, wotw_seedgen_data::Zone)>,
    pub success: bool,
}
#[derive(Debug, Clone, Default)]
pub struct ItemMetadata {
    /// Generic name used when sending the item to another world and in the spoiler
    pub name: Option<String>,
    /// Base price used when placed in a shop
    pub price: Option<CommandInteger>,
    /// Description used when placed in a shop
    pub description: Option<CommandString>,
    /// Icon used when placed in a shop
    pub icon: Option<CommandIcon>,
    /// Map Icon used in the spoiler map
    pub map_icon: Option<MapIcon>,
}
pub struct SnippetLiteralTypes;
impl LiteralTypes for SnippetLiteralTypes {
    type CustomCommand = CommonItem;
    type UberIdentifier = UberIdentifier;
    type String = StringOrPlaceholder;

    fn uber_identifier_literal(value: UberIdentifier) -> Self::UberIdentifier {
        value
    }
    fn string_literal(value: String) -> Self::String {
        StringOrPlaceholder::Value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringOrPlaceholder {
    Value(String),
    ZoneOfPlaceholder(Box<Action>),
    ItemOnPlaceholder(Box<Trigger>),
    CountInZonePlaceholder(Vec<Action>, wotw_seedgen_data::Zone),
}
impl Display for StringOrPlaceholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringOrPlaceholder::Value(string) => string.fmt(f),
            StringOrPlaceholder::ZoneOfPlaceholder(action) => write!(f, "zone_of({action})"),
            StringOrPlaceholder::ItemOnPlaceholder(trigger) => write!(f, "item_on({trigger})"),
            StringOrPlaceholder::CountInZonePlaceholder(actions, zone) => {
                write!(
                    f,
                    "count_in_zone({zone}, [{}])",
                    actions.iter().format(", ")
                )
            }
        }
    }
}
