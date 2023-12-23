use super::StringOrPlaceholder;
use ordered_float::OrderedFloat;
use std::fmt::{self, Display};
use wotw_seedgen_assets::UberStateAlias;

// TODO is this still used for anything other than variables?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    UberIdentifier(UberStateAlias),
    Boolean(bool),
    Integer(i32),
    Float(OrderedFloat<f32>),
    String(StringOrPlaceholder),
    Constant(Constant),
    PathIcon(String),
}
impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::UberIdentifier(value) => value.fmt(f),
            Literal::Boolean(value) => value.fmt(f),
            Literal::Integer(value) => value.fmt(f),
            Literal::Float(value) => value.fmt(f),
            Literal::String(value) => value.fmt(f),
            Literal::Constant(value) => value.fmt(f),
            Literal::PathIcon(path) => write!(f, "icon: \"{path}\""),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    Resource(wotw_seedgen_data::Resource),
    Skill(wotw_seedgen_data::Skill),
    Shard(wotw_seedgen_data::Shard),
    Teleporter(wotw_seedgen_data::Teleporter),
    WeaponUpgrade(wotw_seedgen_data::WeaponUpgrade),
    Equipment(wotw_seedgen_data::Equipment),
    Zone(wotw_seedgen_data::Zone),
    OpherIcon(wotw_seedgen_data::OpherIcon),
    LupoIcon(wotw_seedgen_data::LupoIcon),
    GromIcon(wotw_seedgen_data::GromIcon),
    TuleyIcon(wotw_seedgen_data::TuleyIcon),
    MapIcon(wotw_seedgen_data::MapIcon),
    EquipSlot(wotw_seedgen_data::EquipSlot),
    WheelItemPosition(wotw_seedgen_data::WheelItemPosition),
    WheelBind(wotw_seedgen_data::WheelBind),
    Alignment(wotw_seedgen_data::Alignment),
    ScreenPosition(wotw_seedgen_data::ScreenPosition),
}
impl Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Resource(value) => write!(f, "Resource::{value}"),
            Constant::Skill(value) => write!(f, "Skill::{value}"),
            Constant::Shard(value) => write!(f, "Shard::{value}"),
            Constant::Teleporter(value) => write!(f, "Teleporter::{value}"),
            Constant::WeaponUpgrade(value) => write!(f, "WeaponUpgrade::{value}"),
            Constant::Equipment(value) => write!(f, "Equipment::{value}"),
            Constant::Zone(value) => write!(f, "Zone::{value}"),
            Constant::OpherIcon(value) => write!(f, "OpherIcon::{value}"),
            Constant::LupoIcon(value) => write!(f, "LupoIcon::{value}"),
            Constant::GromIcon(value) => write!(f, "GromIcon::{value}"),
            Constant::TuleyIcon(value) => write!(f, "TuleyIcon::{value}"),
            Constant::MapIcon(value) => write!(f, "MapIcon::{value}"),
            Constant::EquipSlot(value) => write!(f, "EquipSlot::{value}"),
            Constant::WheelItemPosition(value) => write!(f, "WheelItemPosition::{value}"),
            Constant::WheelBind(value) => write!(f, "WheelBind::{value}"),
            Constant::Alignment(value) => write!(f, "Alignment::{value}"),
            Constant::ScreenPosition(value) => write!(f, "ScreenPosition::{value}"),
        }
    }
}
