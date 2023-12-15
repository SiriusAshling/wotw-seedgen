use super::{expression::CompileInto, Compile, SnippetCompiler};
use crate::{
    ast::{self, UberStateType},
    output::{
        Action, Command, CommandBoolean, CommandFloat, CommandIcon, CommandInteger, CommandString,
        CommandVoid, CommandZone, CommonItem, StringOrPlaceholder,
    },
};
use parse_display::FromStr;
use std::ops::Range;
use wotw_seedgen_data::{UberIdentifier, WheelBind};
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
    GetFloat,
    ToFloat,
    GetString,
    ToString,
    ResourceName,
    SkillName,
    ShardName,
    TeleporterName,
    CleanWaterName,
    WeaponUpgradeName,
    CurrentZone,
    ReadIcon,
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
    SetKwolokStatueEnabled,
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
    type Output = Option<Action>;

    fn compile(self, compiler: &mut SnippetCompiler<'_, 'source, '_>) -> Self::Output {
        if let Some(&index) = compiler.function_indices.get(self.identifier.data.0) {
            return Some(Action::Command(Command::Void(CommandVoid::Lookup {
                index,
            })));
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
                let function = match context.compiler.uber_state_type(uber_identifier, &span)? {
                    UberStateType::Boolean => {
                        Command::Boolean(CommandBoolean::FetchBoolean { uber_identifier })
                    }
                    UberStateType::Integer => {
                        Command::Integer(CommandInteger::FetchInteger { uber_identifier })
                    }
                    UberStateType::Float => {
                        Command::Float(CommandFloat::FetchFloat { uber_identifier })
                    }
                };
                Action::Command(function)
            }
            FunctionIdentifier::IsInHitbox => {
                Action::Command(Command::Boolean(CommandBoolean::IsInHitbox {
                    x1: boxed_arg(&mut context)?, // TODO we short circuit potential error messages here, but this does avoid duplicate "too few arguments" errors, so we'd need a different approach to begin with
                    y1: boxed_arg(&mut context)?,
                    x2: boxed_arg(&mut context)?,
                    y2: boxed_arg(&mut context)?,
                }))
            }
            FunctionIdentifier::GetBoolean => {
                Action::Command(Command::Boolean(CommandBoolean::GetBoolean {
                    id: boolean_id(&mut context)?,
                }))
            }
            FunctionIdentifier::GetInteger => {
                Action::Command(Command::Integer(CommandInteger::GetInteger {
                    id: integer_id(&mut context)?,
                }))
            }
            FunctionIdentifier::GetFloat => {
                Action::Command(Command::Float(CommandFloat::GetFloat {
                    id: float_id(&mut context)?,
                }))
            }
            FunctionIdentifier::ToFloat => Action::Command(Command::Float(CommandFloat::ToFloat {
                integer: boxed_arg(&mut context)?,
            })),
            FunctionIdentifier::GetString => {
                Action::Command(Command::String(CommandString::GetString {
                    id: string_id(&mut context)?,
                }))
            }
            FunctionIdentifier::ToString => {
                // TODO sometimes we can evaluate this already
                Action::Command(Command::String(CommandString::ToString {
                    command: boxed_arg(&mut context)?,
                }))
            }
            FunctionIdentifier::ResourceName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(
                        CommonItem::Resource(arg(&mut context)?).to_string(),
                    ),
                }))
            }
            FunctionIdentifier::SkillName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(
                        CommonItem::Skill(arg(&mut context)?).to_string(),
                    ),
                }))
            }
            FunctionIdentifier::ShardName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(
                        CommonItem::Shard(arg(&mut context)?).to_string(),
                    ),
                }))
            }
            FunctionIdentifier::TeleporterName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(
                        CommonItem::Teleporter(arg(&mut context)?).to_string(),
                    ),
                }))
            }
            FunctionIdentifier::CleanWaterName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(CommonItem::CleanWater.to_string()),
                }))
            }
            FunctionIdentifier::WeaponUpgradeName => {
                Action::Command(Command::String(CommandString::Constant {
                    value: StringOrPlaceholder::Value(
                        CommonItem::WeaponUpgrade(arg(&mut context)?).to_string(),
                    ),
                }))
            }
            FunctionIdentifier::CurrentZone => {
                Action::Command(Command::Zone(CommandZone::CurrentZone {}))
            }
            FunctionIdentifier::ReadIcon => Action::Command(Command::Icon(CommandIcon::ReadIcon {
                path: boxed_arg(&mut context)?,
            })),
            FunctionIdentifier::SpiritLight => {
                Action::Command(Command::Custom(CommonItem::SpiritLight(arg(&mut context)?)))
            }
            FunctionIdentifier::RemoveSpiritLight => Action::Command(Command::Custom(
                CommonItem::RemoveSpiritLight(arg(&mut context)?),
            )),
            FunctionIdentifier::Resource => {
                Action::Command(Command::Custom(CommonItem::Resource(arg(&mut context)?)))
            }
            FunctionIdentifier::RemoveResource => Action::Command(Command::Custom(
                CommonItem::RemoveResource(arg(&mut context)?),
            )),
            FunctionIdentifier::Skill => {
                Action::Command(Command::Custom(CommonItem::Skill(arg(&mut context)?)))
            }
            FunctionIdentifier::RemoveSkill => {
                Action::Command(Command::Custom(CommonItem::RemoveSkill(arg(&mut context)?)))
            }
            FunctionIdentifier::Shard => {
                Action::Command(Command::Custom(CommonItem::Shard(arg(&mut context)?)))
            }
            FunctionIdentifier::RemoveShard => {
                Action::Command(Command::Custom(CommonItem::RemoveShard(arg(&mut context)?)))
            }
            FunctionIdentifier::Teleporter => {
                Action::Command(Command::Custom(CommonItem::Teleporter(arg(&mut context)?)))
            }
            FunctionIdentifier::RemoveTeleporter => Action::Command(Command::Custom(
                CommonItem::RemoveTeleporter(arg(&mut context)?),
            )),
            FunctionIdentifier::CleanWater => {
                Action::Command(Command::Custom(CommonItem::CleanWater))
            }
            FunctionIdentifier::RemoveCleanWater => {
                Action::Command(Command::Custom(CommonItem::RemoveCleanWater))
            }
            FunctionIdentifier::WeaponUpgrade => Action::Command(Command::Custom(
                CommonItem::WeaponUpgrade(arg(&mut context)?),
            )),
            FunctionIdentifier::RemoveWeaponUpgrade => Action::Command(Command::Custom(
                CommonItem::RemoveWeaponUpgrade(arg(&mut context)?),
            )),
            FunctionIdentifier::ItemMessage => {
                Action::Command(Command::Void(CommandVoid::ItemMessage {
                    message: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::ItemMessageWithTimeout => {
                Action::Command(Command::Void(CommandVoid::ItemMessageWithTimeout {
                    message: arg(&mut context)?,
                    timeout: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::PriorityMessage => {
                Action::Command(Command::Void(CommandVoid::PriorityMessage {
                    message: arg(&mut context)?,
                    timeout: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::ControlledMessage => {
                Action::Command(Command::Void(CommandVoid::ControlledMessage {
                    id: message_id(&mut context)?,
                    message: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetMessageText => {
                Action::Command(Command::Void(CommandVoid::SetMessageText {
                    id: message_id(&mut context)?,
                    message: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetMessageTimeout => {
                Action::Command(Command::Void(CommandVoid::SetMessageTimeout {
                    id: message_id(&mut context)?,
                    timeout: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::DestroyMessage => {
                Action::Command(Command::Void(CommandVoid::DestroyMessage {
                    id: message_id(&mut context)?,
                }))
            }
            FunctionIdentifier::Store => store(true, &mut context)?,
            FunctionIdentifier::StoreWithoutTriggers => store(false, &mut context)?,
            FunctionIdentifier::SetBoolean => {
                Action::Command(Command::Void(CommandVoid::SetBoolean {
                    id: boolean_id(&mut context)?,
                    value: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetInteger => {
                Action::Command(Command::Void(CommandVoid::SetInteger {
                    id: integer_id(&mut context)?,
                    value: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetFloat => Action::Command(Command::Void(CommandVoid::SetFloat {
                id: float_id(&mut context)?,
                value: arg(&mut context)?,
            })),
            FunctionIdentifier::SetString => {
                Action::Command(Command::Void(CommandVoid::SetString {
                    id: string_id(&mut context)?,
                    value: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::DefineTimer => {
                Action::Command(Command::Void(CommandVoid::DefineTimer {
                    toggle: arg(&mut context)?,
                    timer: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::Save => Action::Command(Command::Void(CommandVoid::Save {})),
            FunctionIdentifier::Checkpoint => {
                Action::Command(Command::Void(CommandVoid::Checkpoint {}))
            }
            FunctionIdentifier::Warp => Action::Command(Command::Void(CommandVoid::Warp {
                x: arg(&mut context)?,
                y: arg(&mut context)?,
            })),
            FunctionIdentifier::Equip => Action::Command(Command::Void(CommandVoid::Equip {
                slot: arg(&mut context)?,
                equipment: arg(&mut context)?,
            })),
            FunctionIdentifier::Unequip => Action::Command(Command::Void(CommandVoid::Unequip {
                equipment: arg(&mut context)?,
            })),
            FunctionIdentifier::TriggerKeybind => {
                Action::Command(Command::Void(CommandVoid::TriggerKeybind {
                    bind: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::EnableServerSync => {
                Action::Command(Command::Void(CommandVoid::EnableServerSync {
                    uber_identifier: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::DisableServerSync => {
                Action::Command(Command::Void(CommandVoid::DisableServerSync {
                    uber_identifier: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetKwolokStatueEnabled => {
                Action::Command(Command::Void(CommandVoid::SetKwolokStatueEnabled {
                    enabled: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::CreateWarpIcon => {
                Action::Command(Command::Void(CommandVoid::CreateWarpIcon {
                    id: warp_icon_id(&mut context)?,
                    x: arg(&mut context)?,
                    y: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWarpIconLabel => {
                Action::Command(Command::Void(CommandVoid::SetWarpIconLabel {
                    id: warp_icon_id(&mut context)?,
                    label: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::DestroyWarpIcon => {
                Action::Command(Command::Void(CommandVoid::DestroyWarpIcon {
                    id: warp_icon_id(&mut context)?,
                }))
            }
            FunctionIdentifier::SetShopItemData => {
                let uber_identifier = arg::<UberIdentifier>(&mut context)?;
                Action::Multi(vec![
                    Action::Command(Command::Void(CommandVoid::SetShopItemPrice {
                        uber_identifier,
                        price: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetShopItemName {
                        uber_identifier,
                        name: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetShopItemDescription {
                        uber_identifier,
                        description: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetShopItemIcon {
                        uber_identifier,
                        icon: arg(&mut context)?,
                    })),
                ])
            }
            FunctionIdentifier::SetShopItemPrice => {
                Action::Command(Command::Void(CommandVoid::SetShopItemPrice {
                    uber_identifier: arg(&mut context)?,
                    price: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetShopItemName => {
                Action::Command(Command::Void(CommandVoid::SetShopItemName {
                    uber_identifier: arg(&mut context)?,
                    name: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetShopItemDescription => {
                Action::Command(Command::Void(CommandVoid::SetShopItemDescription {
                    uber_identifier: arg(&mut context)?,
                    description: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetShopItemIcon => {
                Action::Command(Command::Void(CommandVoid::SetShopItemIcon {
                    uber_identifier: arg(&mut context)?,
                    icon: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetShopItemHidden => {
                Action::Command(Command::Void(CommandVoid::SetShopItemHidden {
                    uber_identifier: arg(&mut context)?,
                    hidden: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelItemData => {
                let wheel = wheel_id(&mut context)?;
                let position = arg(&mut context)?;
                Action::Multi(vec![
                    Action::Command(Command::Void(CommandVoid::SetWheelItemName {
                        wheel,
                        position,
                        name: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetWheelItemDescription {
                        wheel,
                        position,
                        description: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetWheelItemIcon {
                        wheel,
                        position,
                        icon: arg(&mut context)?,
                    })),
                    Action::Command(Command::Void(CommandVoid::SetWheelItemAction {
                        wheel,
                        position,
                        bind: WheelBind::All,
                        action: arg(&mut context)?,
                    })),
                ])
            }
            FunctionIdentifier::SetWheelItemName => {
                Action::Command(Command::Void(CommandVoid::SetWheelItemName {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    name: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelItemDescription => {
                Action::Command(Command::Void(CommandVoid::SetWheelItemDescription {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    description: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelItemIcon => {
                Action::Command(Command::Void(CommandVoid::SetWheelItemIcon {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    icon: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelItemColor => {
                Action::Command(Command::Void(CommandVoid::SetWheelItemColor {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    red: arg(&mut context)?,
                    green: arg(&mut context)?,
                    blue: arg(&mut context)?,
                    alpha: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelItemAction => {
                Action::Command(Command::Void(CommandVoid::SetWheelItemAction {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                    bind: arg(&mut context)?,
                    action: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::DestroyWheelItem => {
                Action::Command(Command::Void(CommandVoid::DestroyWheelItem {
                    wheel: wheel_id(&mut context)?,
                    position: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::SwitchWheel => {
                Action::Command(Command::Void(CommandVoid::SwitchWheel {
                    wheel: wheel_id(&mut context)?,
                }))
            }
            FunctionIdentifier::SetWheelPinned => {
                Action::Command(Command::Void(CommandVoid::SetWheelPinned {
                    wheel: wheel_id(&mut context)?,
                    pinned: arg(&mut context)?,
                }))
            }
            FunctionIdentifier::ClearAllWheels => {
                Action::Command(Command::Void(CommandVoid::ClearAllWheels {}))
            }
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

fn store(check_triggers: bool, context: &mut ArgContext) -> Option<Action> {
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
    Some(Action::Command(Command::Void(command)))
}
