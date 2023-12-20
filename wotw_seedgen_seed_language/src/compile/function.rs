use super::{expression::CompileInto, Compile, SnippetCompiler};
use crate::{
    ast::{self, UberStateType},
    output::{
        ArithmeticOperator, Command, CommandBoolean, CommandFloat, CommandInteger, CommandString,
        CommandVoid, CommandZone, EqualityComparator, Operation, StringOrPlaceholder,
    },
};
use convert_case::{Case, Casing};
use parse_display::FromStr;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use std::ops::Range;
use wotw_seedgen_data::{
    Resource, Shard, Skill, Teleporter, UberIdentifier, WeaponUpgrade, WheelBind,
};
use wotw_seedgen_parse::{Error, Punctuated, Span, Symbol};

struct ArgContext<'a, 'compiler, 'source, 'uberstates> {
    span: Range<usize>,
    parameters: <Punctuated<ast::Expression<'source>, Symbol<','>> as IntoIterator>::IntoIter,
    compiler: &'a mut SnippetCompiler<'compiler, 'source, 'uberstates>,
}
fn try_next<'source>(
    context: &mut ArgContext<'_, '_, 'source, '_>,
) -> Option<ast::Expression<'source>> {
    let next = context.parameters.next();
    if next.is_none() {
        context.compiler.errors.push(Error::custom(
            "Too few parameters".to_string(),
            context.span.clone(),
        ))
    }
    next
}
fn arg<T: CompileInto>(context: &mut ArgContext) -> Option<T> {
    try_next(context)?.compile_into(context.compiler)
}
fn spanned_arg<T: CompileInto>(context: &mut ArgContext) -> Option<(T, Range<usize>)> {
    let next = try_next(context)?;
    let span = next.span();
    let next = next.compile_into(context.compiler)?;
    Some((next, span))
}
fn boxed_arg<T: CompileInto>(context: &mut ArgContext) -> Option<Box<T>> {
    arg(context).map(Box::new)
}
fn string_literal(context: &mut ArgContext) -> Option<String> {
    let (arg, span) = spanned_arg(context)?;
    match arg {
        CommandString::Constant {
            value: StringOrPlaceholder::Value(value),
        } => Some(value),
        _ => {
            context.compiler.errors.push(Error::custom(
                "Only literals are allowed in this position".to_string(),
                span,
            ));
            None
        }
    }
}
fn boolean_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.boolean_ids.id(id))
}
fn integer_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.integer_ids.id(id))
}
fn float_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.float_ids.id(id))
}
fn string_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.string_ids.id(id))
}
fn message_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.message_ids.id(id))
}
fn wheel_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.wheel_ids.id(id))
}
fn warp_icon_id(context: &mut ArgContext) -> Option<usize> {
    string_literal(context).map(|id| context.compiler.global.warp_icon_ids.id(id))
}

#[derive(FromStr)]
#[display(style = "snake_case")]
pub(crate) enum FunctionIdentifier {
    Fetch,
    IsInHitbox,
    GetBoolean,
    GetInteger,
    ToInteger,
    GetFloat,
    ToFloat,
    GetString,
    ToString,
    SpiritLightString,
    RemoveSpiritLightString,
    ResourceString,
    RemoveResourceString,
    SkillString,
    RemoveSkillString,
    ShardString,
    RemoveShardString,
    TeleporterString,
    RemoveTeleporterString,
    CleanWaterString,
    RemoveCleanWaterString,
    WeaponUpgradeString,
    RemoveWeaponUpgradeString,
    CurrentZone,
    SpiritLight,
    RemoveSpiritLight,
    Resource,
    RemoveResource,
    Skill,
    RemoveSkill,
    Shard,
    RemoveShard,
    Teleporter,
    RemoveTeleporter,
    CleanWater,
    RemoveCleanWater,
    WeaponUpgrade,
    RemoveWeaponUpgrade,
    ItemMessage,
    ItemMessageWithTimeout,
    PriorityMessage,
    ControlledMessage,
    SetMessageText,
    SetMessageTimeout,
    DestroyMessage,
    Store,
    StoreWithoutTriggers,
    SetBoolean,
    SetInteger,
    SetFloat,
    SetString,
    DefineTimer,
    Save,
    Checkpoint,
    Warp,
    Equip,
    Unequip,
    TriggerKeybind,
    EnableServerSync,
    DisableServerSync,
    CreateWarpIcon,
    SetWarpIconLabel,
    DestroyWarpIcon,
    SetShopItemData,
    SetShopItemPrice,
    SetShopItemName,
    SetShopItemDescription,
    SetShopItemIcon,
    SetShopItemHidden,
    SetWheelItemData,
    SetWheelItemName,
    SetWheelItemDescription,
    SetWheelItemIcon,
    SetWheelItemColor,
    SetWheelItemAction,
    DestroyWheelItem,
    SwitchWheel,
    SetWheelPinned,
    ClearAllWheels,
}

impl<'source> Compile<'source> for ast::FunctionCall<'source> {
    type Output = Option<Command>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        if let Some(&index) = compiler.function_indices.get(self.identifier.data.0) {
            if let Ok(parameters) = &self.parameters.content {
                if !parameters.is_empty() {
                    compiler.errors.push(Error::custom(
                    "parameters for custom functions aren't (yet) supported".to_string(),
                    parameters.first().unwrap().span().start..parameters.last.as_ref().unwrap().span().end,
                ).with_help("Use set commands for the values you want to pass and get them again in the function".to_string()))
                }
            }
            return Some(Command::Void(CommandVoid::Lookup { index }));
        }
        let identifier =
            compiler.consume_result(self.identifier.data.0.parse().map_err(|_| {
                Error::custom("Unknown function".to_string(), self.identifier.span)
            }))?;

        let span = self.parameters.span();
        let content = compiler.consume_result(self.parameters.content)?;

        let span = match &content.last {
            Some(last) => content.first().unwrap().span().start..last.span().end,
            None => span,
        };

        let mut context = ArgContext {
            span,
            parameters: content.into_iter(),
            compiler,
        };

        let action = match identifier {
            FunctionIdentifier::Fetch => {
                let uber_identifier = try_next(&mut context)?;
                let span = uber_identifier.span();
                let uber_identifier =
                    uber_identifier.compile_into::<UberIdentifier>(&mut context.compiler)?;
                match context.compiler.uber_state_type(uber_identifier, &span)? {
                    UberStateType::Boolean => {
                        Command::Boolean(CommandBoolean::FetchBoolean { uber_identifier })
                    }
                    UberStateType::Integer => {
                        Command::Integer(CommandInteger::FetchInteger { uber_identifier })
                    }
                    UberStateType::Float => {
                        Command::Float(CommandFloat::FetchFloat { uber_identifier })
                    }
                }
            }
            FunctionIdentifier::IsInHitbox => {
                Command::Boolean(CommandBoolean::IsInHitbox {
                    x1: boxed_arg(&mut context)?, // TODO we short circuit potential error messages here, but this does avoid duplicate "too few arguments" errors, so we'd need a different approach to begin with
                    y1: boxed_arg(&mut context)?,
                    x2: boxed_arg(&mut context)?,
                    y2: boxed_arg(&mut context)?,
                })
            }
            FunctionIdentifier::GetBoolean => Command::Boolean(CommandBoolean::GetBoolean {
                id: boolean_id(&mut context)?,
            }),
            FunctionIdentifier::GetInteger => Command::Integer(CommandInteger::GetInteger {
                id: integer_id(&mut context)?,
            }),
            FunctionIdentifier::ToInteger => {
                let float = arg(&mut context)?;
                let command = match float {
                    CommandFloat::Constant { value } => CommandInteger::Constant {
                        value: value.round() as i32,
                    },
                    _ => CommandInteger::FromFloat {
                        float: Box::new(float),
                    },
                };
                Command::Integer(command)
            }
            FunctionIdentifier::GetFloat => Command::Float(CommandFloat::GetFloat {
                id: float_id(&mut context)?,
            }),
            FunctionIdentifier::ToFloat => {
                let integer = arg(&mut context)?;
                let command = match integer {
                    CommandInteger::Constant { value } => CommandFloat::Constant {
                        value: (value as f32).into(),
                    },
                    _ => CommandFloat::FromInteger {
                        integer: Box::new(integer),
                    },
                };
                Command::Float(command)
            }
            FunctionIdentifier::GetString => Command::String(CommandString::GetString {
                id: string_id(&mut context)?,
            }),
            FunctionIdentifier::ToString => {
                let (arg, span) = spanned_arg(&mut context)?;
                let command = match arg {
                    Command::Boolean(command) => match command {
                        CommandBoolean::Constant { value } => CommandString::Constant {
                            value: value.to_string().into(),
                        },
                        other => CommandString::FromBoolean {
                            boolean: Box::new(other),
                        },
                    },
                    Command::Integer(command) => match command {
                        CommandInteger::Constant { value } => CommandString::Constant {
                            value: value.to_string().into(),
                        },
                        other => CommandString::FromInteger {
                            integer: Box::new(other),
                        },
                    },
                    Command::Float(command) => match command {
                        CommandFloat::Constant { value } => CommandString::Constant {
                            value: value.to_string().into(),
                        },
                        other => CommandString::FromFloat {
                            float: Box::new(other),
                        },
                    },
                    Command::String(command) => command,
                    _ => {
                        context
                            .compiler
                            .errors
                            .push(Error::custom("cannot convert to String".to_string(), span));
                        return None;
                    }
                };

                Command::String(command)
            }
            FunctionIdentifier::SpiritLightString => Command::String(spirit_light_string(
                arg(&mut context)?,
                &mut context.compiler.rng,
                false,
            )),
            FunctionIdentifier::RemoveSpiritLightString => Command::String(spirit_light_string(
                arg(&mut context)?,
                &mut context.compiler.rng,
                true,
            )),
            FunctionIdentifier::ResourceString => {
                Command::String(resource_string(arg(&mut context)?, false))
            }
            FunctionIdentifier::RemoveResourceString => {
                Command::String(resource_string(arg(&mut context)?, true))
            }
            FunctionIdentifier::SkillString => {
                Command::String(skill_string(arg(&mut context)?, false))
            }
            FunctionIdentifier::RemoveSkillString => {
                Command::String(skill_string(arg(&mut context)?, true))
            }
            FunctionIdentifier::ShardString => {
                Command::String(shard_string(arg(&mut context)?, false))
            }
            FunctionIdentifier::RemoveShardString => {
                Command::String(shard_string(arg(&mut context)?, true))
            }
            FunctionIdentifier::TeleporterString => {
                Command::String(teleporter_string(arg(&mut context)?, false))
            }
            FunctionIdentifier::RemoveTeleporterString => {
                Command::String(teleporter_string(arg(&mut context)?, true))
            }
            FunctionIdentifier::CleanWaterString => Command::String(clean_water_string(false)),
            FunctionIdentifier::RemoveCleanWaterString => Command::String(clean_water_string(true)),
            FunctionIdentifier::WeaponUpgradeString => {
                Command::String(weapon_upgrade_string(arg(&mut context)?, false))
            }
            FunctionIdentifier::RemoveWeaponUpgradeString => {
                Command::String(weapon_upgrade_string(arg(&mut context)?, true))
            }
            FunctionIdentifier::CurrentZone => Command::Zone(CommandZone::CurrentZone {}),
            FunctionIdentifier::SpiritLight => {
                let amount = arg::<CommandInteger>(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: spirit_light_string(
                                amount.clone(),
                                &mut context.compiler.rng,
                                false,
                            ),
                        },
                        add(UberIdentifier::SPIRIT_LIGHT, amount),
                    ],
                })
            }
            FunctionIdentifier::RemoveSpiritLight => {
                let amount = arg::<CommandInteger>(&mut context)?;
                let negative = match amount.clone() {
                    CommandInteger::Constant { value } => {
                        CommandInteger::Constant { value: -value }
                    }
                    other => CommandInteger::Arithmetic {
                        operation: Box::new(Operation {
                            left: other,
                            operator: ArithmeticOperator::Multiply,
                            right: CommandInteger::Constant { value: -1 },
                        }),
                    },
                };
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: spirit_light_string(amount, &mut context.compiler.rng, true),
                        },
                        add(UberIdentifier::SPIRIT_LIGHT, negative),
                    ],
                })
            }
            FunctionIdentifier::Resource => {
                let resource = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: resource_string(resource, false),
                        },
                        add(
                            resource.uber_identifier(),
                            CommandInteger::Constant { value: 1 },
                        ),
                    ],
                })
            }
            FunctionIdentifier::RemoveResource => {
                let resource = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: resource_string(resource, true),
                        },
                        add(
                            resource.uber_identifier(),
                            CommandInteger::Constant { value: -1 },
                        ),
                    ],
                })
            }
            FunctionIdentifier::Skill => {
                let skill = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: skill_string(skill, false),
                        },
                        set(skill.uber_identifier(), true),
                    ],
                })
            }
            FunctionIdentifier::RemoveSkill => {
                let skill = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: skill_string(skill, true),
                        },
                        set(skill.uber_identifier(), false),
                    ],
                })
            }
            FunctionIdentifier::Shard => {
                let shard = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: shard_string(shard, false),
                        },
                        set(shard.uber_identifier(), true),
                    ],
                })
            }
            FunctionIdentifier::RemoveShard => {
                let shard = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: shard_string(shard, true),
                        },
                        set(shard.uber_identifier(), false),
                    ],
                })
            }
            FunctionIdentifier::Teleporter => {
                let teleporter = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: teleporter_string(teleporter, false),
                        },
                        set(teleporter.uber_identifier(), true),
                    ],
                })
            }
            FunctionIdentifier::RemoveTeleporter => {
                let teleporter = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: teleporter_string(teleporter, true),
                        },
                        set(teleporter.uber_identifier(), false),
                    ],
                })
            }
            FunctionIdentifier::CleanWater => Command::Void(CommandVoid::Multi {
                commands: vec![
                    CommandVoid::ItemMessage {
                        message: clean_water_string(false),
                    },
                    set(UberIdentifier::CLEAN_WATER, true),
                ],
            }),
            FunctionIdentifier::RemoveCleanWater => Command::Void(CommandVoid::Multi {
                commands: vec![
                    CommandVoid::ItemMessage {
                        message: clean_water_string(true),
                    },
                    set(UberIdentifier::CLEAN_WATER, false),
                ],
            }),
            FunctionIdentifier::WeaponUpgrade => {
                let weapon_upgrade = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: weapon_upgrade_string(weapon_upgrade, false),
                        },
                        set(weapon_upgrade.uber_identifier(), true),
                    ],
                })
            }
            FunctionIdentifier::RemoveWeaponUpgrade => {
                let weapon_upgrade = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::ItemMessage {
                            message: weapon_upgrade_string(weapon_upgrade, true),
                        },
                        set(weapon_upgrade.uber_identifier(), false),
                    ],
                })
            }
            FunctionIdentifier::ItemMessage => Command::Void(CommandVoid::ItemMessage {
                message: arg(&mut context)?,
            }),
            FunctionIdentifier::ItemMessageWithTimeout => {
                Command::Void(CommandVoid::ItemMessageWithTimeout {
                    message: arg(&mut context)?,
                    timeout: arg(&mut context)?,
                })
            }
            FunctionIdentifier::PriorityMessage => Command::Void(CommandVoid::PriorityMessage {
                message: arg(&mut context)?,
                timeout: arg(&mut context)?,
            }),
            FunctionIdentifier::ControlledMessage => {
                Command::Void(CommandVoid::ControlledMessage {
                    id: message_id(&mut context)?,
                    message: arg(&mut context)?,
                })
            }
            FunctionIdentifier::SetMessageText => Command::Void(CommandVoid::SetMessageText {
                id: message_id(&mut context)?,
                message: arg(&mut context)?,
            }),
            FunctionIdentifier::SetMessageTimeout => {
                Command::Void(CommandVoid::SetMessageTimeout {
                    id: message_id(&mut context)?,
                    timeout: arg(&mut context)?,
                })
            }
            FunctionIdentifier::DestroyMessage => Command::Void(CommandVoid::DestroyMessage {
                id: message_id(&mut context)?,
            }),
            FunctionIdentifier::Store => store(true, &mut context)?,
            FunctionIdentifier::StoreWithoutTriggers => store(false, &mut context)?,
            FunctionIdentifier::SetBoolean => Command::Void(CommandVoid::SetBoolean {
                id: boolean_id(&mut context)?,
                value: arg(&mut context)?,
            }),
            FunctionIdentifier::SetInteger => Command::Void(CommandVoid::SetInteger {
                id: integer_id(&mut context)?,
                value: arg(&mut context)?,
            }),
            FunctionIdentifier::SetFloat => Command::Void(CommandVoid::SetFloat {
                id: float_id(&mut context)?,
                value: arg(&mut context)?,
            }),
            FunctionIdentifier::SetString => Command::Void(CommandVoid::SetString {
                id: string_id(&mut context)?,
                value: arg(&mut context)?,
            }),
            FunctionIdentifier::DefineTimer => Command::Void(CommandVoid::DefineTimer {
                toggle: arg(&mut context)?,
                timer: arg(&mut context)?,
            }),
            FunctionIdentifier::Save => Command::Void(CommandVoid::Save {}),
            FunctionIdentifier::Checkpoint => Command::Void(CommandVoid::Checkpoint {}),
            FunctionIdentifier::Warp => Command::Void(CommandVoid::Warp {
                x: arg(&mut context)?,
                y: arg(&mut context)?,
            }),
            FunctionIdentifier::Equip => Command::Void(CommandVoid::Equip {
                slot: arg(&mut context)?,
                equipment: arg(&mut context)?,
            }),
            FunctionIdentifier::Unequip => Command::Void(CommandVoid::Unequip {
                equipment: arg(&mut context)?,
            }),
            FunctionIdentifier::TriggerKeybind => Command::Void(CommandVoid::TriggerKeybind {
                bind: arg(&mut context)?,
            }),
            FunctionIdentifier::EnableServerSync => Command::Void(CommandVoid::EnableServerSync {
                uber_identifier: arg(&mut context)?,
            }),
            FunctionIdentifier::DisableServerSync => {
                Command::Void(CommandVoid::DisableServerSync {
                    uber_identifier: arg(&mut context)?,
                })
            }
            FunctionIdentifier::CreateWarpIcon => Command::Void(CommandVoid::CreateWarpIcon {
                id: warp_icon_id(&mut context)?,
                x: arg(&mut context)?,
                y: arg(&mut context)?,
            }),
            FunctionIdentifier::SetWarpIconLabel => Command::Void(CommandVoid::SetWarpIconLabel {
                id: warp_icon_id(&mut context)?,
                label: arg(&mut context)?,
            }),
            FunctionIdentifier::DestroyWarpIcon => Command::Void(CommandVoid::DestroyWarpIcon {
                id: warp_icon_id(&mut context)?,
            }),
            FunctionIdentifier::SetShopItemData => {
                let uber_identifier = arg::<UberIdentifier>(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::SetShopItemPrice {
                            uber_identifier,
                            price: arg(&mut context)?,
                        },
                        CommandVoid::SetShopItemName {
                            uber_identifier,
                            name: arg(&mut context)?,
                        },
                        CommandVoid::SetShopItemDescription {
                            uber_identifier,
                            description: arg(&mut context)?,
                        },
                        CommandVoid::SetShopItemIcon {
                            uber_identifier,
                            icon: arg(&mut context)?,
                        },
                    ],
                })
            }
            FunctionIdentifier::SetShopItemPrice => Command::Void(CommandVoid::SetShopItemPrice {
                uber_identifier: arg(&mut context)?,
                price: arg(&mut context)?,
            }),
            FunctionIdentifier::SetShopItemName => Command::Void(CommandVoid::SetShopItemName {
                uber_identifier: arg(&mut context)?,
                name: arg(&mut context)?,
            }),
            FunctionIdentifier::SetShopItemDescription => {
                Command::Void(CommandVoid::SetShopItemDescription {
                    uber_identifier: arg(&mut context)?,
                    description: arg(&mut context)?,
                })
            }
            FunctionIdentifier::SetShopItemIcon => Command::Void(CommandVoid::SetShopItemIcon {
                uber_identifier: arg(&mut context)?,
                icon: arg(&mut context)?,
            }),
            FunctionIdentifier::SetShopItemHidden => {
                Command::Void(CommandVoid::SetShopItemHidden {
                    uber_identifier: arg(&mut context)?,
                    hidden: arg(&mut context)?,
                })
            }
            FunctionIdentifier::SetWheelItemData => {
                let wheel = wheel_id(&mut context)?;
                let position = arg(&mut context)?;
                Command::Void(CommandVoid::Multi {
                    commands: vec![
                        CommandVoid::SetWheelItemName {
                            wheel,
                            position,
                            name: arg(&mut context)?,
                        },
                        CommandVoid::SetWheelItemDescription {
                            wheel,
                            position,
                            description: arg(&mut context)?,
                        },
                        CommandVoid::SetWheelItemIcon {
                            wheel,
                            position,
                            icon: arg(&mut context)?,
                        },
                        CommandVoid::SetWheelItemAction {
                            wheel,
                            position,
                            bind: WheelBind::All,
                            action: arg(&mut context)?,
                        },
                    ],
                })
            }
            FunctionIdentifier::SetWheelItemName => Command::Void(CommandVoid::SetWheelItemName {
                wheel: wheel_id(&mut context)?,
                position: arg(&mut context)?,
                name: arg(&mut context)?,
            }),
            FunctionIdentifier::SetWheelItemDescription => {
                Command::Void(CommandVoid::SetWheelItemDescription {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    description: arg(&mut context)?,
                })
            }
            FunctionIdentifier::SetWheelItemIcon => Command::Void(CommandVoid::SetWheelItemIcon {
                wheel: wheel_id(&mut context)?,
                position: arg(&mut context)?,
                icon: arg(&mut context)?,
            }),
            FunctionIdentifier::SetWheelItemColor => {
                Command::Void(CommandVoid::SetWheelItemColor {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    red: arg(&mut context)?,
                    green: arg(&mut context)?,
                    blue: arg(&mut context)?,
                    alpha: arg(&mut context)?,
                })
            }
            FunctionIdentifier::SetWheelItemAction => {
                Command::Void(CommandVoid::SetWheelItemAction {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    bind: arg(&mut context)?,
                    action: arg(&mut context)?,
                })
            }
            FunctionIdentifier::DestroyWheelItem => Command::Void(CommandVoid::DestroyWheelItem {
                wheel: wheel_id(&mut context)?,
                position: arg(&mut context)?,
            }),
            FunctionIdentifier::SwitchWheel => Command::Void(CommandVoid::SwitchWheel {
                wheel: wheel_id(&mut context)?,
            }),
            FunctionIdentifier::SetWheelPinned => Command::Void(CommandVoid::SetWheelPinned {
                wheel: wheel_id(&mut context)?,
                pinned: arg(&mut context)?,
            }),
            FunctionIdentifier::ClearAllWheels => Command::Void(CommandVoid::ClearAllWheels {}),
        };

        if let Some(excess) = context.parameters.next() {
            let span = excess.span();
            let end = context
                .parameters
                .last()
                .map_or(span.end, |last| last.span().end);
            context.compiler.errors.push(Error::custom(
                "Too many parameters".to_string(),
                span.start..end,
            ));
            return None;
        }

        Some(action)
    }
}

fn spirit_light_string(amount: CommandInteger, rng: &mut Pcg64Mcg, remove: bool) -> CommandString {
    CommandString::Multi {
        commands: vec![
            CommandVoid::If {
                condition: CommandBoolean::RandomSpiritLightNames {},
                command: Box::new(CommandVoid::SetString {
                    id: 2,
                    value: CommandString::Constant {
                        value: (*SPIRIT_LIGHT_NAMES.choose(rng).unwrap()).into(),
                    },
                }),
            },
            CommandVoid::If {
                condition: CommandBoolean::CompareBoolean {
                    operation: Box::new(Operation {
                        left: CommandBoolean::RandomSpiritLightNames {},
                        operator: EqualityComparator::Equal,
                        right: CommandBoolean::Constant { value: false },
                    }),
                },
                command: Box::new(CommandVoid::SetString {
                    id: 2,
                    value: CommandString::Constant {
                        value: "Spirit Light".into(),
                    },
                }),
            },
        ],
        last: Box::new(if remove {
            CommandString::Concatenate {
                left: Box::new(match amount {
                    CommandInteger::Constant { value } => CommandString::Constant {
                        value: format!("@ Remove {value} ").into(),
                    },
                    other => CommandString::Concatenate {
                        left: Box::new(CommandString::Constant {
                            value: "@ Remove ".into(),
                        }),
                        right: Box::new(CommandString::Concatenate {
                            left: Box::new(CommandString::FromInteger {
                                integer: Box::new(other),
                            }),
                            right: Box::new(CommandString::Constant { value: " ".into() }),
                        }),
                    },
                }),
                right: Box::new(CommandString::Concatenate {
                    left: Box::new(CommandString::GetString { id: 2 }),
                    right: Box::new(CommandString::Constant { value: "@".into() }),
                }),
            }
        } else {
            CommandString::Concatenate {
                left: Box::new(match amount {
                    CommandInteger::Constant { value } => CommandString::Constant {
                        value: format!("{value} ").into(),
                    },
                    other => CommandString::Concatenate {
                        left: Box::new(CommandString::FromInteger {
                            integer: Box::new(other),
                        }),
                        right: Box::new(CommandString::Constant { value: " ".into() }),
                    },
                }),
                right: Box::new(CommandString::GetString { id: 2 }),
            }
        }),
    }
}
fn resource_string(resource: Resource, remove: bool) -> CommandString {
    let resource_cased = resource
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Title);
    let value = if remove {
        format!("@Remove {resource_cased}@")
    } else {
        resource_cased
    }
    .into();
    CommandString::Constant { value }
}
fn skill_string(skill: Skill, remove: bool) -> CommandString {
    let skill_cased = skill
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Title);
    let value = if remove {
        format!("@Remove {skill_cased}@")
    } else {
        match skill {
            Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => {
                format!("#{skill_cased}#")
            }
            _ => format!("*{skill_cased}*"),
        }
    }
    .into();
    CommandString::Constant { value }
}
fn shard_string(shard: Shard, remove: bool) -> CommandString {
    let shard_cased = shard
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Title);
    let value = if remove {
        format!("@Remove {shard_cased}@")
    } else {
        format!("${shard_cased}$")
    }
    .into();
    CommandString::Constant { value }
}
fn teleporter_string(teleporter: Teleporter, remove: bool) -> CommandString {
    let name = match teleporter {
        Teleporter::Inkwater => "Inkwater Marsh",
        Teleporter::Den => "Howl's Den",
        Teleporter::Hollow => "Kwolok's Hollow",
        Teleporter::Glades => "Glades",
        Teleporter::Wellspring => "Wellspring",
        Teleporter::Burrows => "Midnight Burrows",
        Teleporter::WoodsEntrance => "Woods Entrance",
        Teleporter::WoodsExit => "Woods Exit",
        Teleporter::Reach => "Baur's Reach",
        Teleporter::Depths => "Mouldwood Depths",
        Teleporter::CentralLuma => "Central Luma",
        Teleporter::LumaBoss => "Luma Boss",
        Teleporter::FeedingGrounds => "Feeding Grounds",
        Teleporter::CentralWastes => "Central Wastes",
        Teleporter::OuterRuins => "Outer Ruins",
        Teleporter::InnerRuins => "Inner Ruins",
        Teleporter::Willow => "Willow's End",
        Teleporter::Shriek => "Shriek",
    };
    let value = if remove {
        format!("@Remove {name} Teleporter@")
    } else {
        format!("#{name} Teleporter#")
    }
    .into();
    CommandString::Constant { value }
}
fn clean_water_string(remove: bool) -> CommandString {
    let value = if remove {
        "@Remove Clean Water@"
    } else {
        "*Clean Water*"
    }
    .into();
    CommandString::Constant { value }
}
fn weapon_upgrade_string(weapon_upgrade: WeaponUpgrade, remove: bool) -> CommandString {
    let weapon_upgrade_cased = weapon_upgrade
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Title);
    let value = if remove {
        format!("@Remove {weapon_upgrade_cased}@")
    } else {
        format!("#{weapon_upgrade_cased}#")
    }
    .into();
    CommandString::Constant { value }
}

fn add(uber_identifier: UberIdentifier, amount: CommandInteger) -> CommandVoid {
    CommandVoid::StoreInteger {
        uber_identifier,
        value: CommandInteger::Arithmetic {
            operation: Box::new(Operation {
                left: CommandInteger::FetchInteger { uber_identifier },
                operator: ArithmeticOperator::Add,
                right: amount,
            }),
        },
        check_triggers: true,
    }
}
fn set(uber_identifier: UberIdentifier, value: bool) -> CommandVoid {
    CommandVoid::StoreBoolean {
        uber_identifier,
        value: CommandBoolean::Constant { value },
        check_triggers: true,
    }
}

fn store(check_triggers: bool, context: &mut ArgContext) -> Option<Command> {
    let (uber_identifier, span) = spanned_arg::<UberIdentifier>(context)?;
    let command = match context.compiler.uber_state_type(uber_identifier, &span)? {
        UberStateType::Boolean => CommandVoid::StoreBoolean {
            uber_identifier,
            value: arg(context)?,
            check_triggers,
        },
        UberStateType::Integer => CommandVoid::StoreInteger {
            uber_identifier,
            value: arg(context)?,
            check_triggers,
        },
        UberStateType::Float => CommandVoid::StoreFloat {
            uber_identifier,
            value: arg(context)?,
            check_triggers,
        },
    };
    if context
        .compiler
        .global
        .uber_state_data
        .id_lookup
        .get(&uber_identifier)
        .map_or(false, |entry| entry.readonly)
    {
        context.compiler.errors.push(Error::custom(
            "this uberState is readonly".to_string(),
            span,
        ));
    }
    Some(Command::Void(command))
}

const SPIRIT_LIGHT_NAMES: &[&str] = &[
    "Spirit Light",
    "Gallons",
    "Spirit Bucks",
    "Gold",
    "Geo",
    "EXP",
    "Experience",
    "XP",
    "Gil",
    "GP",
    "Dollars",
    "Tokens",
    "Tickets",
    "Pounds Sterling",
    "Brownie Points",
    "Euros",
    "Credits",
    "Bells",
    "Fish",
    "Zenny",
    "Pesos",
    "Exalted Orbs",
    "Hryvnia",
    "Poké",
    "Glod",
    "Dollerydoos",
    "Boonbucks",
    "Pieces of Eight",
    "Shillings",
    "Farthings",
    "Kalganids",
    "Quatloos",
    "Crowns",
    "Solari",
    "Widgets",
    "Ori Money",
    "Money",
    "Cash",
    "Munny",
    "Nuyen",
    "Rings",
    "Rupees",
    "Coins",
    "Echoes",
    "Sovereigns",
    "Points",
    "Drams",
    "Doubloons",
    "Spheres",
    "Silver",
    "Slivers",
    "Rubies",
    "Emeralds",
    "Notes",
    "Yen",
    "Złoty",
    "Likes",
    "Comments",
    "Subs",
    "Bananas",
    "Sapphires",
    "Diamonds",
    "Fun",
    "Minerals",
    "Vespene Gas",
    "Sheep",
    "Brick",
    "Wheat",
    "Wood",
    "Quills",
    "Bits",
    "Bytes",
    "Nuts",
    "Bolts",
    "Souls",
    "Runes",
    "Pons",
    "Boxings",
    "Stonks",
    "Leaves",
    "Marbles",
    "Stamps",
    "Hugs",
    "Nobles",
    "Socks",
];
