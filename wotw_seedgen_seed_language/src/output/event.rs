pub use crate::ast::PseudoTrigger;

use super::{Command, CommandBoolean};
use wotw_seedgen_data::UberIdentifier;

/// The main event (:badumtsss:)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    /// The Trigger defines when to give the Action
    pub trigger: Trigger,
    /// The Action defines what to do when the Trigger happens
    pub action: Action,
}

/// Trigger for an [`Event`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trigger {
    /// Pseudo triggers are tied to specific events
    Pseudo(PseudoTrigger),
    /// Trigger on every change to an UberIdentifier
    Binding(UberIdentifier),
    /// Trigger when the condition changes from `false` to `true`
    Condition(CommandBoolean),
}

// TODO these could be variants of command?
/// Action performed in an [`Event`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    /// Execute the Command
    Command(Command),
    /// Check a Condition
    Condition(Box<ActionCondition>),
    /// Perform all the contained Actions
    Multi(Vec<Action>),
}

/// A conditional [`Action`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionCondition {
    /// If the expression fails to evaluate, the action should not be performed
    pub condition: CommandBoolean,
    /// Action to perform if the condition evaluated to `true`
    pub action: Action,
}
