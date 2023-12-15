use super::StringOrPlaceholder;
use decorum::R32;
use wotw_seedgen_assets::UberStateAlias;

// TODO is this still used for anything other than variables?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    UberIdentifier(UberStateAlias),
    Boolean(bool),
    Integer(i32),
    Float(R32),
    String(StringOrPlaceholder),
    Constant(Constant),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
}
