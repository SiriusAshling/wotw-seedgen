use crate::{
    perfect_derive::perfect_derive, ArithmeticOperator, Comparator, EqualityComparator, Icon,
    LiteralTypes, LogicOperator, Operation,
};
use decorum::R32;
use serde::{Deserialize, Serialize};
use wotw_seedgen_data::{EquipSlot, Equipment, MapIcon, WheelBind, WheelItemPosition, Zone};

perfect_derive! {
    /// A Command, which may be used to affect the world, player or client state
    pub enum Command<T: LiteralTypes> {
        /// Commands returning [`bool`]
        Boolean(CommandBoolean<T>),
        /// Commands returning [`i32`]
        Integer(CommandInteger<T>),
        /// Commands returning [`f32`]
        Float(CommandFloat<T>),
        /// Commands returning [`T::String`]
        String(CommandString<T>),
        /// Commands returning [`Zone`]
        Zone(CommandZone),
        /// Commands returning [`Icon`]
        Icon(CommandIcon<T>),
        /// Commands returning nothing
        Void(CommandVoid<T>),
        /// Commands provided by [`LiteralTypes`]
        Custom(T::CustomCommand),
    }
}
perfect_derive! {
    /// Command which returns [`bool`]
    pub enum CommandBoolean<T: LiteralTypes> {
        /// Return `value`
        Constant {
            value: bool,
        },
        /// Return the result of `operation`
        CompareBoolean {
            #[serde(flatten)]
            operation: Box<Operation<CommandBoolean<T>, EqualityComparator>>,
        },
        /// Return the result of `operation`
        CompareInteger {
            #[serde(flatten)]
            operation: Box<Operation<CommandInteger<T>, Comparator>>,
        },
        /// Return the result of `operation`
        CompareFloat {
            #[serde(flatten)]
            operation: Box<Operation<CommandFloat<T>, Comparator>>,
        },
        /// Return the result of `operation`
        CompareString {
            #[serde(flatten)]
            operation: Box<Operation<CommandString<T>, EqualityComparator>>,
        },
        /// Return the result of `operation`
        CompareZone {
            #[serde(flatten)]
            operation: Box<Operation<CommandZone, EqualityComparator>>,
        },
        /// Return the result of `operation`
        LogicOperation {
            #[serde(flatten)]
            operation: Box<Operation<CommandBoolean<T>, LogicOperator>>,
        },
        /// Return the value stored in `uber_identifier`
        // TODO could there be a better naming convention than fetch vs get etc.?
        FetchBoolean {
            uber_identifier: T::UberIdentifier,
        },
        /// Get the value stored under `id`
        GetBoolean {
            id: usize,
        },
        // TODO some kind of lint if things like this appear in trigger conditions
        /// Check if Ori is in the hitbox defined by (`x1`, `y1`) and (`x2`, `y2)
        IsInHitbox {
            x1: Box<CommandFloat<T>>,
            y1: Box<CommandFloat<T>>,
            x2: Box<CommandFloat<T>>,
            y2: Box<CommandFloat<T>>,
        },
        /// Return whether the user wants to see random spirit light names
        RandomSpiritLightNames {},
    }
}
perfect_derive! {
    /// Command which returns [`i32`]
    pub enum CommandInteger<T: LiteralTypes> {
        /// Return `value`
        Constant {
            value: i32,
        },
        /// Return the result of `operation`
        Arithmetic {
            #[serde(flatten)]
            operation: Box<Operation<CommandInteger<T>, ArithmeticOperator>>,
        },
        /// Return the value stored in `uber_identifier`
        // TODO these could just be called "Fetch"? This is redundant with the type name
        FetchInteger {
            uber_identifier: T::UberIdentifier,
        },
        /// Get the value stored under `id`
        GetInteger {
            id: usize,
        },
    }
}
perfect_derive! {
    /// Command which returns [`f32`]
    pub enum CommandFloat<T: LiteralTypes> {
        /// Return `value`
        Constant {
            value: R32,
        },
        /// Return the result of `operation`
        Arithmetic {
            #[serde(flatten)]
            operation: Box<Operation<CommandFloat<T>, ArithmeticOperator>>,
        },
        /// Return the value stored in `uber_identifier`
        FetchFloat {
            uber_identifier: T::UberIdentifier,
        },
        /// Get the value stored under `id`
        GetFloat {
            id: usize,
        },
        /// Convert `integer` to `f32`
        ToFloat {
            integer: Box<CommandInteger<T>>,
        },
    }
}
perfect_derive! {
    /// Command which returns [`T::String`]
    pub enum CommandString<T: LiteralTypes> {
        /// Return `value`
        Constant {
            value: T::String,
        },
        /// Return a String consisting of `left`, then `right`
        Concatenate {
            left: Box<CommandString<T>>,
            right: Box<CommandString<T>>,
        },
        /// Get the value stored under `id`
        GetString {
            id: usize,
        },
        /// Return the name of world number `index`
        WorldName {
            index: usize,
        },
        /// Convert `command` to a `String`
        ///
        /// Only booleans, numbers and strings are required to be converted successfully
        ToString {
            command: Box<Command<T>>,
        },
    }
}
/// Command which returns [`Zone`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CommandZone {
    /// Return `value`
    Constant { value: Zone },
    /// Return the zone Ori is currently in
    CurrentZone {},
}
perfect_derive! {
    /// Command which returns [`Icon`]
    pub enum CommandIcon<T: LiteralTypes> {
        /// Return `value`
        Constant {
            value: Icon,
        },
        /// Read an icon from `path`
        ReadIcon {
            path: Box<CommandString<T>>,
        },
    }
}
perfect_derive! {
    /// Command which returns nothing
    pub enum CommandVoid<T: LiteralTypes> {
        /// Add `message` to the item message queue with a default timeout
        ItemMessage {
            message: CommandString<T>,
        },
        /// Add `message` to the item message queue with `timeout`
        ItemMessageWithTimeout {
            message: CommandString<T>,
            timeout: CommandFloat<T>,
        },
        /// Show `message` immediately as a priority message with `timeout`
        PriorityMessage {
            message: CommandString<T>,
            timeout: CommandFloat<T>,
        },
        /// Show `message` immediately as a priority message and keep `id` as a reference to it
        ControlledMessage {
            id: usize,
            message: CommandString<T>,
        },
        /// If `id` refers to an existing controlled message, change its text to `message`
        SetMessageText {
            id: usize,
            message: CommandString<T>,
        },
        /// If `id` refers to an existing controlled message, set its `timeout`
        SetMessageTimeout {
            id: usize,
            timeout: CommandInteger<T>,
        },
        /// If `id` refers to an existing controlled message, DESTROY it
        DestroyMessage {
            id: usize,
        },
        /// Store `value` in `uber_identifier` and check if any events are triggered
        StoreBoolean {
            uber_identifier: T::UberIdentifier,
            value: CommandBoolean<T>,
            check_triggers: bool,
        },
        /// Store `value` in `uber_identifier` and check if any events are triggered
        StoreInteger {
            uber_identifier: T::UberIdentifier,
            value: CommandInteger<T>,
            check_triggers: bool,
        },
        /// Store `value` in `uber_identifier` and check if any events are triggered
        StoreFloat {
            uber_identifier: T::UberIdentifier,
            value: CommandFloat<T>,
            check_triggers: bool,
        },
        /// Temporarily store `value` under `id`. The value should live at least until the next tick
        SetBoolean {
            id: usize,
            value: CommandBoolean<T>,
        },
        /// Temporarily store `value` under `id`. The value should live at least until the next tick
        SetInteger {
            id: usize,
            value: CommandInteger<T>,
        },
        /// Temporarily store `value` under `id`. The value should live at least until the next tick
        SetFloat {
            id: usize,
            value: CommandFloat<T>,
        },
        /// Temporarily store `value` under `id`. The value should live at least until the next tick
        SetString {
            id: usize,
            value: CommandString<T>,
        },
        /// Until the next reload, on every tick where `toggle` is true, increment `timer` by the amount of seconds passed
        DefineTimer {
            toggle: T::UberIdentifier,
            timer: T::UberIdentifier,
        },
        /// Perform a "hard" save like an autosave
        Save {},
        /// Perform a "soft" checkpoint like a boss fight checkpoint
        Checkpoint {},
        /// Warp the player to (`x`, `y`)
        Warp {
            x: CommandFloat<T>,
            y: CommandFloat<T>,
        },
        /// Equip `equipment` into `slot`
        Equip {
            slot: EquipSlot,
            equipment: Equipment,
        },
        /// Unequip `equipment` from any slot it may be equipped in
        Unequip {
            equipment: Equipment,
        },
        /// Act as though the user would have pressed `bind`
        TriggerKeybind {
            bind: CommandString<T>,
        },
        /// Start syncing `uber_identifier` in co-op
        EnableServerSync {
            uber_identifier: T::UberIdentifier,
        },
        /// Stop syncing `uber_identifier` in co-op
        DisableServerSync {
            uber_identifier: T::UberIdentifier,
        },
        /// If the Kwolok Eyestone statue is not `enabled`, the player should be unable to interact with it
        SetKwolokStatueEnabled {
            enabled: CommandBoolean<T>,
        },
        // TODO how does the client want to identify the map icon?
        /// Set the map icon associated with the `location` identifier from loc_data to `icon`
        SetSpoilerMapIcon {
            location: CommandString<T>,
            icon: MapIcon,
            label: CommandString<T>,
        },
        /// Create a spirit well icon that you can warp to on the map at (`x`, `y`)
        CreateWarpIcon {
            id: usize,
            x: CommandFloat<T>,
            y: CommandFloat<T>,
        },
        /// Set the map label of an existing spirit well icon `id` to `label`
        SetWarpIconLabel {
            id: usize,
            label: CommandString<T>,
        },
        /// DESTROY the spirit well icon `id`
        DestroyWarpIcon {
            id: usize,
        },
        // TODO would seem more efficient to set these at once to save uber_identifier lookups
        // (same for wheel stuff)
        /// Set the price of the shop item at `uber_identifier` to `price`
        SetShopItemPrice {
            uber_identifier: T::UberIdentifier,
            price: CommandInteger<T>,
        },
        /// Set the display name of the shop item at `uber_identifier` to `name`
        SetShopItemName {
            uber_identifier: T::UberIdentifier,
            name: CommandString<T>,
        },
        /// Set the description of the shop item at `uber_identifier` to `description`
        SetShopItemDescription {
            uber_identifier: T::UberIdentifier,
            description: CommandString<T>,
        },
        /// Set the icon of the shop item at `uber_identifier` to `icon`
        SetShopItemIcon {
            uber_identifier: T::UberIdentifier,
            icon: CommandIcon<T>,
        },
        /// Set the shop item at `uber_identifier` to be `hidden`
        SetShopItemHidden {
            uber_identifier: T::UberIdentifier,
            hidden: CommandBoolean<T>,
        },
        /// Set the display name of the wheel item in `wheel` at `position` to `name`
        SetWheelItemName {
            wheel: usize,
            position: WheelItemPosition,
            name: CommandString<T>,
        },
        /// Set the description of the wheel item in `wheel` at `position` to `description`
        SetWheelItemDescription {
            wheel: usize,
            position: WheelItemPosition,
            description: CommandString<T>,
        },
        /// Set the icon of the wheel item in `wheel` at `position` to `icon`
        SetWheelItemIcon {
            wheel: usize,
            position: WheelItemPosition,
            icon: CommandIcon<T>,
        },
        /// Set the rgba color of the wheel item in `wheel` at `position` to `red`, `green`, `blue`, `alpha`
        SetWheelItemColor {
            wheel: usize,
            position: WheelItemPosition,
            red: CommandInteger<T>,
            green: CommandInteger<T>,
            blue: CommandInteger<T>,
            alpha: CommandInteger<T>,
        },
        /// When pressing `bind` with the wheel item in `wheel` at `position` selected, lookup and perform `action`
        SetWheelItemAction {
            wheel: usize,
            position: WheelItemPosition,
            bind: WheelBind,
            action: usize,
        },
        /// Remove the wheel item in `wheel` at `position`
        DestroyWheelItem {
            wheel: usize,
            position: WheelItemPosition,
        },
        /// Switch the active wheel to `wheel`
        SwitchWheel {
            wheel: usize,
        },
        /// If a `wheel` is `pinned`, it should remain the active wheel after closing and reopening the randomizer wheel
        SetWheelPinned {
            wheel: usize,
            pinned: CommandBoolean<T>,
        },
        /// Remove all wheel items
        ClearAllWheels {},
        /// Lookup and perform the action at `index`
        Lookup {
            index: usize,
        }
    }
}
