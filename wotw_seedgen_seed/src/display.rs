use itertools::Itertools;

use crate::{
    Action, ActionCondition, Command, CommandBoolean, CommandFloat, CommandIcon, CommandInteger,
    CommandString, CommandVoid, CommandZone, Event, Icon, LiteralTypes, Operation, Trigger,
};
use std::fmt::{self, Display};

impl Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Shard(shard) => write!(f, "{shard} icon"),
            Icon::Equipment(equipment) => write!(f, "{equipment} icon"),
            Icon::Opher(opher_icon) => write!(f, "{opher_icon} icon"),
            Icon::Lupo(lupo_icon) => write!(f, "{lupo_icon} icon"),
            Icon::Grom(grom_icon) => write!(f, "{grom_icon} icon"),
            Icon::Tuley(tuley_icon) => write!(f, "{tuley_icon} icon"),
        }
    }
}

impl<T: LiteralTypes> Display for Event<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "on {} {}", self.trigger, self.action)
    }
}

impl<T: LiteralTypes> Display for Trigger<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trigger::Pseudo(pseudo) => pseudo.fmt(f),
            // TODO not sure I decided on this syntax yet
            Trigger::Binding(uber_identifier) => write!(f, "[{uber_identifier}]"),
            Trigger::Condition(condition) => condition.fmt(f),
        }
    }
}

impl<T: LiteralTypes> Display for Action<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Command(action) => action.fmt(f),
            Action::Condition(action) => action.fmt(f),
            Action::Multi(action) => write!(f, "[{}]", action.iter().format(", ")),
        }
    }
}

impl<T: LiteralTypes> Display for ActionCondition<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if ({}) {{ {} }}", self.condition, self.action)
    }
}

impl<Item: Display, Operator: Display> Display for Operation<Item, Operator> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl<T: LiteralTypes> Display for Command<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Boolean(command) => command.fmt(f),
            Command::Integer(command) => command.fmt(f),
            Command::Float(command) => command.fmt(f),
            Command::String(command) => command.fmt(f),
            Command::Zone(command) => command.fmt(f),
            Command::Icon(command) => command.fmt(f),
            Command::Void(command) => command.fmt(f),
            Command::Custom(command) => command.fmt(f),
        }
    }
}

impl<T: LiteralTypes> Display for CommandBoolean<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandBoolean::Constant { value } => value.fmt(f),
            CommandBoolean::CompareBoolean { operation } => operation.fmt(f),
            CommandBoolean::CompareInteger { operation } => operation.fmt(f),
            CommandBoolean::CompareFloat { operation } => operation.fmt(f),
            CommandBoolean::CompareString { operation } => operation.fmt(f),
            CommandBoolean::CompareZone { operation } => operation.fmt(f),
            CommandBoolean::LogicOperation { operation } => operation.fmt(f),
            CommandBoolean::FetchBoolean { uber_identifier } => {
                write!(f, "fetch({uber_identifier})")
            }
            CommandBoolean::GetBoolean { id } => write!(f, "get_boolean({id})"),
            CommandBoolean::IsInHitbox { x1, y1, x2, y2 } => {
                write!(f, "is_in_hitbox({x1}, {y1}, {x2}, {y2})")
            }
            CommandBoolean::RandomSpiritLightNames {} => write!(f, "random_spirit_light_names()"),
        }
    }
}

impl<T: LiteralTypes> Display for CommandInteger<T>
where
    T::UberIdentifier: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandInteger::Constant { value } => value.fmt(f),
            CommandInteger::Arithmetic { operation } => operation.fmt(f),
            CommandInteger::FetchInteger { uber_identifier } => {
                write!(f, "fetch({uber_identifier})")
            }
            CommandInteger::GetInteger { id } => write!(f, "get_integer({id})"),
        }
    }
}

impl<T: LiteralTypes> Display for CommandFloat<T>
where
    T::UberIdentifier: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandFloat::Constant { value } => value.fmt(f),
            CommandFloat::Arithmetic { operation } => operation.fmt(f),
            CommandFloat::FetchFloat { uber_identifier } => write!(f, "fetch({uber_identifier})"),
            CommandFloat::GetFloat { id } => write!(f, "get_float({id})"),
            CommandFloat::ToFloat { integer } => write!(f, "to_float({integer})"),
        }
    }
}

impl<T: LiteralTypes> Display for CommandString<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandString::Constant { value } => write!(f, "\"{value}\""),
            CommandString::Concatenate { left, right } => write!(f, "{left} + {right}"),
            CommandString::GetString { id } => write!(f, "get_string({id})"),
            CommandString::WorldName { index } => write!(f, "world_name({index})"),
            CommandString::ToString { command } => write!(f, "to_string({command})"),
        }
    }
}

impl Display for CommandZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandZone::Constant { value } => value.fmt(f),
            CommandZone::CurrentZone {} => write!(f, "current_zone()"),
        }
    }
}

impl<T: LiteralTypes> Display for CommandIcon<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandIcon::Constant { value } => value.fmt(f),
            CommandIcon::ReadIcon { path } => write!(f, "read_icon({path})"),
        }
    }
}

impl<T: LiteralTypes> Display for CommandVoid<T>
where
    T::CustomCommand: Display,
    T::UberIdentifier: Display,
    T::String: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandVoid::ItemMessage { message } => write!(f, "item_message({message})"),
            CommandVoid::ItemMessageWithTimeout { message, timeout } => {
                write!(f, "item_message_with_timeout({message}, {timeout})")
            }
            CommandVoid::PriorityMessage { message, timeout } => {
                write!(f, "priority_message({message}, {timeout})")
            }
            CommandVoid::ControlledMessage { id, message } => {
                write!(f, "controlled_message({id}, {message})")
            }
            CommandVoid::SetMessageText { id, message } => {
                write!(f, "set_message_text({id}, {message})")
            }
            CommandVoid::SetMessageTimeout { id, timeout } => {
                write!(f, "set_message_timeout({id}, {timeout})")
            }
            CommandVoid::DestroyMessage { id } => write!(f, "destroy_message({id})"),
            CommandVoid::StoreBoolean {
                uber_identifier,
                value,
                check_triggers,
            } => write!(
                f,
                "store_boolean{}({uber_identifier}, {value})",
                if *check_triggers {
                    ""
                } else {
                    "_without_triggers"
                }
            ),
            CommandVoid::StoreInteger {
                uber_identifier,
                value,
                check_triggers,
            } => write!(
                f,
                "store_integer{}({uber_identifier}, {value})",
                if *check_triggers {
                    ""
                } else {
                    "_without_triggers"
                }
            ),
            CommandVoid::StoreFloat {
                uber_identifier,
                value,
                check_triggers,
            } => write!(
                f,
                "store_float{}({uber_identifier}, {value})",
                if *check_triggers {
                    ""
                } else {
                    "_without_triggers"
                }
            ),
            CommandVoid::SetBoolean { id, value } => write!(f, "set_boolean({id}, {value})"),
            CommandVoid::SetInteger { id, value } => write!(f, "set_integer({id}, {value})"),
            CommandVoid::SetFloat { id, value } => write!(f, "set_float({id}, {value})"),
            CommandVoid::SetString { id, value } => write!(f, "set_string({id}, {value})"),
            CommandVoid::DefineTimer { toggle, timer } => {
                write!(f, "define_timer({toggle}, {timer})")
            }
            CommandVoid::Save {} => write!(f, "save()"),
            CommandVoid::Checkpoint {} => write!(f, "checkpoint()"),
            CommandVoid::Warp { x, y } => write!(f, "warp({x}, {y})"),
            CommandVoid::Equip { slot, equipment } => write!(f, "equip({slot}, {equipment})"),
            CommandVoid::Unequip { equipment } => write!(f, "unequip({equipment})"),
            CommandVoid::TriggerKeybind { bind } => write!(f, "trigger_keybind({bind})"),
            CommandVoid::EnableServerSync { uber_identifier } => {
                write!(f, "enable_server_sync({uber_identifier})")
            }
            CommandVoid::DisableServerSync { uber_identifier } => {
                write!(f, "disable_server_sync({uber_identifier})")
            }
            CommandVoid::SetKwolokStatueEnabled { enabled } => {
                write!(f, "set_kwolok_statue_enabled({enabled})")
            }
            CommandVoid::SetSpoilerMapIcon {
                location,
                icon,
                label,
            } => write!(f, "set_map_icon({location}, {icon}, {label})"),
            CommandVoid::CreateWarpIcon { id, x, y } => {
                write!(f, "create_warp_icon({id}, {x}, {y})")
            }
            CommandVoid::SetWarpIconLabel { id, label } => {
                write!(f, "set_warp_icon_label({id}, {label})")
            }
            CommandVoid::DestroyWarpIcon { id } => write!(f, "destroy_warp_icon({id})"),
            CommandVoid::SetShopItemPrice {
                uber_identifier,
                price,
            } => write!(f, "set_shop_item_price({uber_identifier}, {price})"),
            CommandVoid::SetShopItemName {
                uber_identifier,
                name,
            } => write!(f, "set_shop_item_name({uber_identifier}, {name})"),
            CommandVoid::SetShopItemDescription {
                uber_identifier,
                description,
            } => write!(
                f,
                "set_shop_item_description({uber_identifier}, {description})"
            ),
            CommandVoid::SetShopItemIcon {
                uber_identifier,
                icon,
            } => write!(f, "set_shop_item_icon({uber_identifier}, {icon})"),
            CommandVoid::SetShopItemHidden {
                uber_identifier,
                hidden,
            } => write!(f, "set_shop_item_hidden({uber_identifier}, {hidden})"),
            CommandVoid::SetWheelItemName {
                wheel,
                position,
                name,
            } => write!(f, "set_wheel_item_name({wheel}, {position}, {name})"),
            CommandVoid::SetWheelItemDescription {
                wheel,
                position,
                description,
            } => write!(
                f,
                "set_wheel_item_description({wheel}, {position}, {description})"
            ),
            CommandVoid::SetWheelItemIcon {
                wheel,
                position,
                icon,
            } => write!(f, "set_wheel_item_icon({wheel}, {position}, {icon})"),
            CommandVoid::SetWheelItemColor {
                wheel,
                position,
                red,
                green,
                blue,
                alpha,
            } => write!(
                f,
                "set_wheel_item_color({wheel}, {position}, {red}, {green}, {blue}, {alpha})"
            ),
            CommandVoid::SetWheelItemAction {
                wheel,
                position,
                bind,
                action,
            } => write!(
                f,
                "set_wheel_item_action({wheel}, {position}, {bind}, {action})"
            ),
            CommandVoid::DestroyWheelItem { wheel, position } => {
                write!(f, "destroy_wheel_item({wheel}, {position})")
            }
            CommandVoid::SwitchWheel { wheel } => write!(f, "switch_wheel({wheel})"),
            CommandVoid::SetWheelPinned { wheel, pinned } => {
                write!(f, "set_wheel_pinned({wheel}, {pinned})")
            }
            CommandVoid::ClearAllWheels {} => write!(f, "clear_all_wheels()"),
            CommandVoid::Lookup { index } => write!(f, "lookup({index})"),
        }
    }
}
