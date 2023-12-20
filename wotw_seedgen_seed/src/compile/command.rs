use super::{args::Args, compile_into_lookup, Compile};
use crate::Command;
use wotw_seedgen_data::UberIdentifier;
use wotw_seedgen_seed_language::output::{
    self as input, CommandVoid, Comparator, EqualityComparator, StringOrPlaceholder,
};

fn unwrap_string_placeholder(value: StringOrPlaceholder) -> String {
    match value {
        StringOrPlaceholder::Value(value) => value,
        _ => panic!("Unresolved string placeholder during compilation"),
    }
}

impl Compile for input::Command {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Boolean(command) => command.compile(command_lookup),
            Self::Integer(command) => command.compile(command_lookup),
            Self::Float(command) => command.compile(command_lookup),
            Self::String(command) => command.compile(command_lookup),
            Self::Zone(command) => command.compile(command_lookup),
            Self::Void(command) => command.compile(command_lookup),
        }
    }
}

impl Compile for input::CommandBoolean {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Constant { value } => vec![Command::SetBoolean { value }],
            Self::Multi { commands, last } => multi(commands, *last, command_lookup),
            Self::CompareBoolean { operation } => Args::new(2, command_lookup)
                .bool(operation.left)
                .bool(operation.right)
                .call(Command::CompareBoolean {
                    operator: operation.operator,
                }),
            Self::CompareInteger { operation } => Args::new(2, command_lookup)
                .int(operation.left)
                .int(operation.right)
                .call(Command::CompareInteger {
                    operator: operation.operator,
                }),
            Self::CompareFloat { operation } => Args::new(2, command_lookup)
                .float(operation.left)
                .float(operation.right)
                .call(Command::CompareFloat {
                    operator: operation.operator,
                }),
            Self::CompareString { operation } => Args::new(2, command_lookup)
                .string(operation.left)
                .string(operation.right)
                .call(Command::CompareString {
                    operator: operation.operator,
                }),
            Self::CompareZone { operation } => Args::new(2, command_lookup)
                .zone(operation.left)
                .zone(operation.right)
                .call(Command::CompareInteger {
                    operator: match operation.operator {
                        EqualityComparator::Equal => Comparator::Equal,
                        EqualityComparator::NotEqual => Comparator::NotEqual,
                    },
                }),
            Self::LogicOperation { operation } => Args::new(2, command_lookup)
                .bool(operation.left)
                .bool(operation.right)
                .call(Command::LogicOperation {
                    operator: operation.operator,
                }),
            Self::FetchBoolean { uber_identifier } => {
                vec![Command::FetchBoolean { uber_identifier }]
            }
            Self::GetBoolean { id } => vec![Command::CopyBoolean { from: id, to: 0 }],
            Self::IsInHitbox { x1, y1, x2, y2 } => Args::new(4, command_lookup)
                .float(*x1)
                .float(*y1)
                .float(*x2)
                .float(*y2)
                .call(Command::IsInHitbox),
            Self::RandomSpiritLightNames {} => vec![Command::RandomSpiritLightNames],
        }
    }
}

impl Compile for input::CommandInteger {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Constant { value } => vec![Command::SetInteger { value }],
            Self::Multi { commands, last } => multi(commands, *last, command_lookup),
            Self::Arithmetic { operation } => Args::new(2, command_lookup)
                .int(operation.left)
                .int(operation.right)
                .call(Command::ArithmeticInteger {
                    operator: operation.operator,
                }),
            Self::FetchInteger { uber_identifier } => {
                vec![Command::FetchInteger { uber_identifier }]
            }
            Self::GetInteger { id } => vec![Command::CopyInteger { from: id, to: 0 }],
        }
    }
}

impl Compile for input::CommandFloat {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Constant { value } => vec![Command::SetFloat {
                value: value.into(),
            }],
            Self::Multi { commands, last } => multi(commands, *last, command_lookup),
            Self::Arithmetic { operation } => Args::new(2, command_lookup)
                .float(operation.left)
                .float(operation.right)
                .call(Command::ArithmeticFloat {
                    operator: operation.operator,
                }),
            Self::FetchFloat { uber_identifier } => {
                vec![Command::FetchFloat { uber_identifier }]
            }
            Self::GetFloat { id } => vec![Command::CopyFloat { from: id, to: 0 }],
            Self::FromInteger { integer } => Args::new(1, command_lookup)
                .int(*integer)
                .call(Command::IntegerToFloat),
        }
    }
}

impl Compile for input::CommandString {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Constant { value } => vec![Command::SetString {
                value: unwrap_string_placeholder(value),
            }],
            Self::Multi { commands, last } => multi(commands, *last, command_lookup),
            Self::Concatenate { left, right } => Args::new(2, command_lookup)
                .string(*left)
                .string(*right)
                .call(Command::Concatenate),
            Self::GetString { id } => vec![Command::CopyString { from: id, to: 0 }],
            Self::WorldName { index } => vec![Command::WorldName { index }],
            Self::FromBoolean { boolean } => Args::new(1, command_lookup)
                .bool(*boolean)
                .call(Command::BooleanToString),
            Self::FromInteger { integer } => Args::new(1, command_lookup)
                .int(*integer)
                .call(Command::IntegerToString),
            Self::FromFloat { float } => Args::new(1, command_lookup)
                .float(*float)
                .call(Command::FloatToString),
        }
    }
}

impl Compile for input::CommandZone {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Constant { value } => vec![Command::SetInteger {
                value: value as i32,
            }],
            Self::Multi { commands, last } => multi(commands, *last, command_lookup),
            Self::CurrentZone {} => vec![Command::FetchInteger {
                uber_identifier: UberIdentifier {
                    group: 5,
                    member: 50,
                },
            }],
        }
    }
}

impl Compile for input::CommandVoid {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Multi { commands } => commands
                .into_iter()
                .flat_map(|command| command.compile(command_lookup))
                .collect(),
            Self::Lookup { index } => vec![Command::Execute { index }],
            Self::If { condition, command } => {
                let index = compile_into_lookup(*command, command_lookup);
                Args::new(1, command_lookup)
                    .bool(condition)
                    .call(Command::ExecuteIf { index })
            }
            Self::ItemMessage { message } => Args::new(1, command_lookup)
                .string(message)
                .call(Command::ItemMessage),
            Self::ItemMessageWithTimeout { message, timeout } => Args::new(2, command_lookup)
                .string(message)
                .float(timeout)
                .call(Command::ItemMessageWithTimeout),
            Self::PriorityMessage { message, timeout } => Args::new(2, command_lookup)
                .string(message)
                .float(timeout)
                .call(Command::PriorityMessage),
            Self::ControlledMessage { id, message } => Args::new(1, command_lookup)
                .string(message)
                .call(Command::ControlledMessage { id }),
            Self::SetMessageText { id, message } => Args::new(1, command_lookup)
                .string(message)
                .call(Command::SetMessageText { id }),
            Self::SetMessageTimeout { id, timeout } => Args::new(1, command_lookup)
                .int(timeout)
                .call(Command::SetMessageTimeout { id }),
            Self::DestroyMessage { id } => vec![Command::DestroyMessage { id }],
            Self::StoreBoolean {
                uber_identifier,
                value,
                check_triggers,
            } => Args::new(1, command_lookup)
                .bool(value)
                .call(Command::StoreBoolean {
                    uber_identifier,
                    check_triggers,
                }),
            Self::StoreInteger {
                uber_identifier,
                value,
                check_triggers,
            } => Args::new(1, command_lookup)
                .int(value)
                .call(Command::StoreInteger {
                    uber_identifier,
                    check_triggers,
                }),
            Self::StoreFloat {
                uber_identifier,
                value,
                check_triggers,
            } => Args::new(1, command_lookup)
                .float(value)
                .call(Command::StoreFloat {
                    uber_identifier,
                    check_triggers,
                }),
            Self::SetBoolean { id, value } => Args::new(1, command_lookup)
                .bool(value)
                .call(Command::CopyBoolean { from: 0, to: id }),
            Self::SetInteger { id, value } => Args::new(1, command_lookup)
                .int(value)
                .call(Command::CopyInteger { from: 0, to: id }),
            Self::SetFloat { id, value } => Args::new(1, command_lookup)
                .float(value)
                .call(Command::CopyFloat { from: 0, to: id }),
            Self::SetString { id, value } => Args::new(1, command_lookup)
                .string(value)
                .call(Command::CopyString { from: 0, to: id }),
            Self::DefineTimer { toggle, timer } => vec![Command::DefineTimer { toggle, timer }],
            Self::Save {} => vec![Command::Save],
            Self::Checkpoint {} => vec![Command::Checkpoint],
            Self::Warp { x, y } => Args::new(2, command_lookup)
                .float(x)
                .float(y)
                .call(Command::Warp),
            Self::Equip { slot, equipment } => vec![Command::Equip { slot, equipment }],
            Self::Unequip { equipment } => vec![Command::Unequip { equipment }],
            Self::TriggerKeybind { bind } => vec![Command::TriggerKeybind {
                bind: unwrap_string_placeholder(bind),
            }],
            Self::EnableServerSync { uber_identifier } => {
                vec![Command::EnableServerSync { uber_identifier }]
            }
            Self::DisableServerSync { uber_identifier } => {
                vec![Command::DisableServerSync { uber_identifier }]
            }
            Self::SetSpoilerMapIcon {
                location,
                icon,
                label,
            } => Args::new(1, command_lookup)
                .string(label)
                .call(Command::SetSpoilerMapIcon { location, icon }),
            Self::CreateWarpIcon { id, x, y } => Args::new(2, command_lookup)
                .float(x)
                .float(y)
                .call(Command::CreateWarpIcon { id }),
            Self::SetWarpIconLabel { id, label } => Args::new(1, command_lookup)
                .string(label)
                .call(Command::SetWarpIconLabel { id }),
            Self::DestroyWarpIcon { id } => vec![Command::DestroyWarpIcon { id }],
            Self::SetShopItemPrice {
                uber_identifier,
                price,
            } => Args::new(1, command_lookup)
                .int(price)
                .call(Command::SetShopItemPrice { uber_identifier }),
            Self::SetShopItemName {
                uber_identifier,
                name,
            } => Args::new(1, command_lookup)
                .string(name)
                .call(Command::SetShopItemName { uber_identifier }),
            Self::SetShopItemDescription {
                uber_identifier,
                description,
            } => Args::new(1, command_lookup)
                .string(description)
                .call(Command::SetShopItemDescription { uber_identifier }),
            Self::SetShopItemIcon {
                uber_identifier,
                icon,
            } => vec![Command::SetShopItemIcon {
                uber_identifier,
                icon,
            }],
            Self::SetShopItemHidden {
                uber_identifier,
                hidden,
            } => Args::new(1, command_lookup)
                .bool(hidden)
                .call(Command::SetShopItemHidden { uber_identifier }),
            Self::SetWheelItemName {
                wheel,
                position,
                name,
            } => Args::new(1, command_lookup)
                .string(name)
                .call(Command::SetWheelItemName { wheel, position }),
            Self::SetWheelItemDescription {
                wheel,
                position,
                description,
            } => Args::new(1, command_lookup)
                .string(description)
                .call(Command::SetWheelItemDescription { wheel, position }),
            Self::SetWheelItemIcon {
                wheel,
                position,
                icon,
            } => vec![Command::SetWheelItemIcon {
                wheel,
                position,
                icon,
            }],
            Self::SetWheelItemColor {
                wheel,
                position,
                red,
                green,
                blue,
                alpha,
            } => Args::new(4, command_lookup)
                .int(red)
                .int(green)
                .int(blue)
                .int(alpha)
                .call(Command::SetWheelItemColor { wheel, position }),
            Self::SetWheelItemAction {
                wheel,
                position,
                bind,
                action,
            } => vec![Command::SetWheelItemCommand {
                wheel,
                position,
                bind,
                command: action,
            }],
            Self::DestroyWheelItem { wheel, position } => {
                vec![Command::DestroyWheelItem { wheel, position }]
            }
            Self::SwitchWheel { wheel } => {
                vec![Command::SwitchWheel { wheel }]
            }
            Self::SetWheelPinned { wheel, pinned } => Args::new(1, command_lookup)
                .bool(pinned)
                .call(Command::SetWheelPinned { wheel }),
            Self::ClearAllWheels {} => {
                vec![Command::ClearAllWheels]
            }
        }
    }
}

fn multi<T: Compile<Output = Vec<Command>>>(
    commands: Vec<CommandVoid>,
    last: T,
    command_lookup: &mut Vec<Vec<Command>>,
) -> Vec<Command> {
    let mut commands = commands
        .into_iter()
        .flat_map(|command| command.compile(command_lookup))
        .collect::<Vec<_>>();
    commands.extend(last.compile(command_lookup));
    commands
}
