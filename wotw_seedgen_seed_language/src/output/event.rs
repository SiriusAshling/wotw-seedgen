pub use crate::ast::ClientEvent;

use super::{Command, CommandBoolean};
use wotw_seedgen_data::UberIdentifier;

/// The main event (:badumtsss:)
// TODO improve documentation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    /// The Trigger defines when to give the Action
    pub trigger: Trigger,
    /// The Command defines what to do when the Trigger happens
    pub command: Command,
}

/// Trigger for an [`Event`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trigger {
    /// Specific client events
    ClientEvent(ClientEvent),
    /// Trigger on every change to an UberIdentifier
    Binding(UberIdentifier),
    /// Trigger when the condition changes from `false` to `true`
    Condition(CommandBoolean),
}
