mod compile;

pub use compile::*;
pub use wotw_seedgen_data::{
    EquipSlot, Equipment, MapIcon, Position, UberIdentifier, WheelBind, WheelItemPosition,
};
pub use wotw_seedgen_seed_language::output::{
    ArithmeticOperator, Comparator, EqualityComparator, Icon, LogicOperator, Operation,
    PseudoTrigger,
};

use serde::{Deserialize, Serialize};

/// Seed data for one World
#[derive(Debug, Serialize, Deserialize)]
pub struct SeedWorld {
    /// Starting location
    pub spawn: Spawn,
    /// Events from generation and snippets
    pub events: Vec<Event>,
    /// [`Command`]s that may be referenced from elsewhere by index
    ///
    /// Each index may store multiple [`Command`]s to execute
    pub command_lookup: Vec<Vec<Command>>,
}

/// Spawn location for a [`SeedWorld`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Spawn {
    /// Where to spawn in world coordinates
    pub position: Position,
    /// Anchor Identifier from the logic source, needed for reach check
    pub identifier: String,
}

/// The main event (:badumtsss:)
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    /// `trigger` defines when to execute `command`
    pub trigger: Trigger,
    /// Index into `command_lookup`
    pub command: usize,
}

/// Trigger for an [`Event`]
#[derive(Debug, Serialize, Deserialize)]
pub enum Trigger {
    /// Pseudo triggers are tied to specific events
    Pseudo(PseudoTrigger),
    /// Trigger on every change to an UberIdentifier
    Binding(UberIdentifier),
    /// Index into `command_lookup`
    ///
    /// After executing the command, Boolean Memory 0 determines whether the condition is met.
    /// The last result of executing the command should be saved, with an initial value of `false`.
    /// The trigger should only fire if the last result was `false` and the current result is `true`
    Condition(usize),
}

/// A Command, which may be used to affect the world, player or client state
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    /// Execute the commands at `index` in command_lookup
    Execute { index: usize },
    /// Execute the commands at `index` in command_lookup if Boolean Memory 0 is `true`
    ExecuteIf { index: usize },
    /// Load `value` into Boolean Memory 0
    LoadBoolean { value: bool },
    /// Load `value` into Integer Memory 0
    LoadInteger { value: i32 },
    /// Load `value` into Float Memory 0
    LoadFloat { value: f32 },
    /// Load `value` into String Memory 0
    LoadString { value: String },
    /// Copy address `from` into address `to` in Boolean Memory
    CopyBoolean { from: usize, to: usize },
    /// Copy address `from` into address `to` in Integer Memory
    CopyInteger { from: usize, to: usize },
    /// Copy address `from` into address `to` in Float Memory
    CopyFloat { from: usize, to: usize },
    /// Copy address `from` into address `to` in String Memory
    CopyString { from: usize, to: usize },
    /// Copy the value of `uber_identifier` into Boolean Memory 0
    FetchBoolean { uber_identifier: UberIdentifier },
    /// Copy the value of `uber_identifier` into Integer Memory 0
    FetchInteger { uber_identifier: UberIdentifier },
    /// Copy the value of `uber_identifier` into Float Memory 0
    FetchFloat { uber_identifier: UberIdentifier },
    /// Copy the value of Boolean Memory 0 into `uber_identifier`
    StoreBoolean {
        uber_identifier: UberIdentifier,
        check_triggers: bool,
    },
    /// Copy the value of Integer Memory 0 into `uber_identifier`
    StoreInteger {
        uber_identifier: UberIdentifier,
        check_triggers: bool,
    },
    /// Copy the value of Float Memory 0 into `uber_identifier`
    StoreFloat {
        uber_identifier: UberIdentifier,
        check_triggers: bool,
    },
    /// Perform `operator` on Boolean Memory 1 and Boolean Memory 0 and store the result in Boolean Memory 0
    CompareBoolean { operator: EqualityComparator },
    /// Perform `operator` on Integer Memory 1 and Integer Memory 0 and store the result in Integer Memory 0
    CompareInteger { operator: Comparator },
    /// Perform `operator` on Float Memory 1 and Float Memory 0 and store the result in Float Memory 0
    CompareFloat { operator: Comparator },
    /// Perform `operator` on String Memory 1 and String Memory 0 and store the result in String Memory 0
    CompareString { operator: EqualityComparator },
    /// Perform `operator` on Boolean Memory 1 and Boolean Memory 0 and store the result in Boolean Memory 0
    LogicOperation { operator: LogicOperator },
    /// Perform `operator` on Integer Memory 1 and Integer Memory 0 and store the result in Integer Memory 0
    ArithmeticInteger { operator: ArithmeticOperator },
    /// Perform `operator` on Float Memory 1 and Float Memory 0 and store the result in Float Memory 0
    ArithmeticFloat { operator: ArithmeticOperator },
    /// Concatenate String Memory 1 and String Memory 0 and store the result in String Memory 0
    Concatenate,
    /// Convert Integer Memory 0 to a float and store it in Float Memory 0
    IntegerToFloat,
    /// Convert Boolean Memory 0 to a string and store it in String Memory 0
    BooleanToString,
    /// Convert Integer Memory 0 to a string and store it in String Memory 0
    IntegerToString,
    /// Convert Float Memory 0 to a string and store it in String Memory 0
    FloatToString,
    DefineTimer {
        toggle: UberIdentifier,
        timer: UberIdentifier,
    },
    /// Check if Ori is in the hitbox defined by (Float Memory 1, Float Memory 2) and (Float Memory 3, Float Memory 0) and store the result in Boolean Memory 0
    IsInHitbox,
    /// Store whether the user wants to see random spirit light names in Boolean Memory 0
    RandomSpiritLightNames,
    /// Store the name of world number `index` in String Memory 0
    WorldName { index: usize },
    /// Store Ori's current zone in Integer Memory 0
    CurrentZone,
    /// Queue String Memory 0 as item message with a default timeout
    ItemMessage,
    /// Queue String Memory 0 as item message with Float Memory 0 as timeout
    ItemMessageWithTimeout,
    /// Show String Memory 0 as priority message with Float Memory 0 as timeout
    PriorityMessage,
    /// Show String Memory 0 as priority message and keep `id` as a reference to it
    ControlledMessage { id: usize },
    /// If `id` refers to an existing controlled message, set its text to String Memory 0
    SetMessageText { id: usize },
    /// If `id` refers to an existing controlled message, set its timeout to Float Memory 0
    SetMessageTimeout { id: usize },
    /// If `id` refers to an existing controlled message, DESTROY, OBLITERATE and ANNIHILATE it
    DestroyMessage { id: usize },
    /// Perform a "hard" save like an autosave
    Save,
    /// Perform a "soft" checkpoint like a boss fight checkpoint
    Checkpoint,
    /// Warp the player to (Float Memory 1, Float Memory 0)
    Warp,
    /// Equip `equipment` into `slot`
    Equip {
        slot: EquipSlot,
        equipment: Equipment,
    },
    /// Unequip `equipment` from any slot it may be equipped in
    Unequip { equipment: Equipment },
    /// Act as though the user would have pressed `bind`
    TriggerKeybind { bind: String },
    /// Start syncing `uber_identifier` in co-op
    EnableServerSync { uber_identifier: UberIdentifier },
    /// Stop syncing `uber_identifier` in co-op
    DisableServerSync { uber_identifier: UberIdentifier },
    /// Set whether the Kwolok statue can be interacted with based on Boolean Memory 0
    SetKwolokStatueEnabled,
    /// Set the map icon associated with the `location` identifier from loc_data to `icon` and the label to String Memory 0
    SetSpoilerMapIcon { location: String, icon: MapIcon },
    /// Create a spirit well icon that you can warp to on the map at (Float Memory 1, Float Memory 0)
    CreateWarpIcon { id: usize },
    /// If `id` refers to an existing spirit well icon, set its label to String Memory 0
    SetWarpIconLabel { id: usize },
    /// If `id` refers to an existing spirit well icon, DESTROY, OBLITERATE and ANNIHILATE it
    DestroyWarpIcon { id: usize },
    /// Set the price of the shop item at `uber_identifier` to Integer Memory 0
    SetShopItemPrice { uber_identifier: UberIdentifier },
    /// Set the display name of the shop item at `uber_identifier` to String Memory 0
    SetShopItemName { uber_identifier: UberIdentifier },
    /// Set the description of the shop item at `uber_identifier` to String Memory 0
    SetShopItemDescription { uber_identifier: UberIdentifier },
    /// Set the icon of the shop item at `uber_identifier` to `icon`
    SetShopItemIcon {
        uber_identifier: UberIdentifier,
        icon: Icon,
    },
    /// Set whether the shop item at `uber_identifier` is hidden based on Boolean Memory 0
    SetShopItemHidden { uber_identifier: UberIdentifier },
    /// Set the display name of the wheel item in `wheel` at `position` to String Memory 0
    SetWheelItemName {
        wheel: usize,
        position: WheelItemPosition,
    },
    /// Set the description of the wheel item in `wheel` at `position` to String Memory 0
    SetWheelItemDescription {
        wheel: usize,
        position: WheelItemPosition,
    },
    /// Set the icon of the wheel item in `wheel` at `position` to `icon`
    SetWheelItemIcon {
        wheel: usize,
        position: WheelItemPosition,
        icon: Icon,
    },
    /// Set the rgba color of the wheel item in `wheel` at `position` to Integer Memory 1, Integer Memory 2, Integer Memory 3, Integer Memory 0
    SetWheelItemColor {
        wheel: usize,
        position: WheelItemPosition,
    },
    /// When pressing `bind` with the wheel item in `wheel` at `position` selected, execute `command`
    SetWheelItemCommand {
        wheel: usize,
        position: WheelItemPosition,
        bind: WheelBind,
        command: usize,
    },
    /// Remove the wheel item in `wheel` at `position`
    DestroyWheelItem {
        wheel: usize,
        position: WheelItemPosition,
    },
    /// Switch the active wheel to `wheel`
    SwitchWheel { wheel: usize },
    /// Sets whether `wheel` is pinned based on Boolean Memory 0
    SetWheelPinned { wheel: usize },
    /// Remove all wheel items
    ClearAllWheels,
}
