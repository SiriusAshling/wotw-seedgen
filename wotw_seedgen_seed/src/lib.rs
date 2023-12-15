mod command;
mod display;
mod operation;
mod perfect_derive;

pub use {command::*, operation::*};

use parse_display::{Display, FromStr};
use perfect_derive::perfect_derive;
use serde::{Deserialize, Serialize};
use wotw_seedgen_data::{
    Equipment, GromIcon, LupoIcon, OpherIcon, Position, Shard, TuleyIcon, UberIdentifier,
};

pub trait LiteralTypes {
    type CustomCommand;
    type UberIdentifier;
    type String;

    fn uber_identifier_literal(value: UberIdentifier) -> Self::UberIdentifier;
    fn string_literal(value: String) -> Self::String;
}
pub struct SeedLiteralTypes;
impl LiteralTypes for SeedLiteralTypes {
    type CustomCommand = ();
    type UberIdentifier = UberIdentifier;
    type String = String;

    fn uber_identifier_literal(value: UberIdentifier) -> Self::UberIdentifier {
        value
    }
    fn string_literal(value: String) -> Self::String {
        value
    }
}

/// Seed data for one World
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SeedWorld {
    /// Starting location
    pub spawn: Spawn,
    /// Events from generation and snippets
    pub events: Vec<Event<SeedLiteralTypes>>,
    /// [`Action`]s that may be referenced from elsewhere by index
    pub action_lookup: Vec<Action<SeedLiteralTypes>>,
}
/// Spawn location for a [`SeedWorld`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Spawn {
    /// Where to spawn in world coordinates
    pub position: Position,
    /// Anchor Identifier from the logic source, needed for reach check
    pub identifier: String,
}

perfect_derive! {
    /// The main event (:badumtsss:)
    pub struct Event<T: LiteralTypes> {
        /// The Trigger defines when to give the Action
        pub trigger: Trigger<T>,
        /// The Action defines what to do when the Trigger happens
        pub action: Action<T>,
    }
}

perfect_derive! {
    /// Trigger for an [`Event`]
    pub enum Trigger<T: LiteralTypes> {
        /// Pseudo triggers are tied to specific events
        Pseudo(PseudoTrigger),
        /// Trigger on every change to an UberIdentifier
        Binding(T::UberIdentifier),
        /// Trigger when the condition changes from `false` to `true`
        Condition(CommandBoolean<T>),
    }
}

perfect_derive! {
    /// Action performed in an [`Event`]
    pub enum Action<T: LiteralTypes> {
        /// Execute the Command
        Command(Command<T>),
        /// Check a Condition
        Condition(Box<ActionCondition<T>>),
        /// Perform all the contained Actions
        Multi(Vec<Action<T>>),
    }
}

perfect_derive! {
    /// A conditional [`Action`]
    pub struct ActionCondition<T: LiteralTypes> {
        /// If the expression fails to evaluate, the action should not be performed
        pub condition: CommandBoolean<T>,
        /// Action to perform if the condition evaluated to `true`
        pub action: Action<T>,
    }
}

/// Descriptor for an icon
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Shard(Shard),
    Equipment(Equipment),
    Opher(OpherIcon),
    Lupo(LupoIcon),
    Grom(GromIcon),
    Tuley(TuleyIcon),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, FromStr, Display)]
#[display(style = "snake_case")]
pub enum PseudoTrigger {
    /// Trigger when starting a new file
    Spawn,
    /// Trigger when starting a new file or loading the seed into an active file
    Reload,
    /// Trigger when respawning after death, void etc.
    Respawn,
    /// Trigger on keybind
    Bind1,
    /// Trigger on keybind
    Bind2,
    /// Trigger on keybind
    Bind3,
    /// Trigger on keybind
    Bind4,
    /// Trigger on keybind
    Bind5,
    /// Trigger on Teleport
    Teleport,
    /// Trigger on Jump
    Jump,
    /// Trigger on Double Jump
    DoubleJump,
    /// Trigger on Dash
    Dash,
    /// Trigger on Bash
    Bash,
    /// Trigger on Glide
    Glide,
    /// Trigger on Sword
    Sword,
    /// Trigger on Hammer
    Hammer,
    /// Trigger on Spike
    Spike,
    /// Trigger on Spirit Star
    SpiritStar,
    /// Trigger on Light Burst
    LightBurst,
    /// Trigger on Bow
    Bow,
    /// Trigger on Blaze
    Blaze,
    /// Trigger on Sentry
    Sentry,
    /// Trigger on Flash
    Flash,
    /// Trigger on Launch
    Launch,
    /// Trigger on Wall Jump
    WallJump,
    /// Trigger on Burrow
    Burrow,
    /// Trigger on Water Dash
    WaterDash,
    /// Trigger on Flap
    Flap,
    /// Trigger on Regenerate
    Regenerate,
    /// Trigger on the Show Progress keybind
    ProgressMessage,
    /// Trigger every frame
    Tick,
}
