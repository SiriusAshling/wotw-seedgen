use std::fmt::{self, Display};

use wotw_seedgen_data::{Resource, Shard, Skill, Teleporter, WeaponUpgrade};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommonItem {
    SpiritLight(i32),
    RemoveSpiritLight(i32),
    Resource(Resource),
    RemoveResource(Resource),
    Skill(Skill),
    RemoveSkill(Skill),
    Shard(Shard),
    RemoveShard(Shard),
    Teleporter(Teleporter),
    RemoveTeleporter(Teleporter),
    CleanWater,
    RemoveCleanWater,
    WeaponUpgrade(WeaponUpgrade),
    RemoveWeaponUpgrade(WeaponUpgrade),
}

impl Display for CommonItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonItem::SpiritLight(amount) => write!(f, "{amount} Spirit Light"),
            CommonItem::RemoveSpiritLight(amount) => {
                write_remove(f, CommonItem::SpiritLight(*amount))
            }
            CommonItem::Resource(resource) => resource.fmt(f),
            CommonItem::RemoveResource(resource) => write_remove(f, resource),
            CommonItem::Skill(skill) => match skill {
                Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => {
                    write!(f, "#{skill}#")
                }
                _ => write!(f, "*{skill}*"),
            },
            CommonItem::RemoveSkill(skill) => write_remove(f, skill),
            CommonItem::Shard(shard) => write!(f, "#{shard}#"),
            CommonItem::RemoveShard(shard) => write_remove(f, shard),
            CommonItem::Teleporter(teleporter) => write!(f, "#{teleporter} Teleporter#"),
            CommonItem::RemoveTeleporter(teleporter) => write_remove(f, teleporter),
            CommonItem::CleanWater => write!(f, "*Clean Water*"),
            CommonItem::RemoveCleanWater => write_remove(f, "Clean Water"),
            CommonItem::WeaponUpgrade(weapon_upgrade) => write!(f, "#{weapon_upgrade}#"),
            CommonItem::RemoveWeaponUpgrade(weapon_upgrade) => write_remove(f, weapon_upgrade),
        }
    }
}
fn write_remove<D: Display>(f: &mut fmt::Formatter<'_>, value: D) -> fmt::Result {
    write!(f, "@Removed {value}@")
}

// TODO
// pub fn common_item<T: LiteralTypes<UberIdentifier = UberIdentifier>>(
//     action: &Action<T>,
//     action_lookup: &ActionLookup,
// ) -> Option<CommonItem> {
//     fn is(index: usize, default_action: DefaultAction, action_lookup: &ActionLookup) -> bool {
//         action_lookup
//             .default_actions
//             .get(&default_action)
//             .map_or(false, |action_index| index == *action_index)
//     }
//     fn try_from_u16<T: TryFrom<u8>>(value: u16) -> Option<T> {
//         u8::try_from(value).ok().and_then(|id| T::try_from(id).ok())
//     }
//
//     match action {
//         Action::Multi(multi) => match &multi[..] {
//             [Action::Command(Command::Void(CommandVoid::SetInteger {
//                 id: i32::MAX,
//                 value: CommandInteger::Constant { value: amount },
//             })), _, Action::Command(Command::Void(CommandVoid::Lookup { index }))] => {
//                 if is(*index, DefaultAction::SpiritLight, action_lookup) {
//                     Some(CommonItem::SpiritLight(*amount))
//                 } else if is(*index, DefaultAction::RemoveSpiritLight, action_lookup) {
//                     Some(CommonItem::RemoveSpiritLight(*amount))
//                 } else {
//                     None
//                 }
//             }
//             [_, Action::Command(Command::Void(CommandVoid::StoreBoolean {
//                 uber_identifier,
//                 value: CommandBoolean::Constant { value },
//             }))] => match uber_identifier {
//                 UberIdentifier { group: 6, member } => match member {
//                     1000..=1121 => try_from_u16(*member - 1000).map(|skill| {
//                         if *value {
//                             CommonItem::Skill(skill)
//                         } else {
//                             CommonItem::RemoveSkill(skill)
//                         }
//                     }),
//                     2000 => Some(if *value {
//                         CommonItem::CleanWater
//                     } else {
//                         CommonItem::RemoveCleanWater
//                     }),
//                     4000..=4047 => try_from_u16(*member - 4000).map(|shard| {
//                         if *value {
//                             CommonItem::Shard(shard)
//                         } else {
//                             CommonItem::RemoveShard(shard)
//                         }
//                     }),
//                     5000..=5004 => try_from_u16(*member - 5000).map(|weapon_upgrade| {
//                         if *value {
//                             CommonItem::WeaponUpgrade(weapon_upgrade)
//                         } else {
//                             CommonItem::RemoveWeaponUpgrade(weapon_upgrade)
//                         }
//                     }),
//                     _ => None,
//                 },
//                 UberIdentifier {
//                     group: 21786,
//                     member: 10185,
//                 } => Some(CommonItem::Teleporter(Teleporter::Marsh)),
//                 UberIdentifier {
//                     group: 11666,
//                     member: 61594,
//                 } => Some(CommonItem::Teleporter(Teleporter::Den)),
//                 UberIdentifier {
//                     group: 937,
//                     member: 26601,
//                 } => Some(CommonItem::Teleporter(Teleporter::Hollow)),
//                 UberIdentifier {
//                     group: 42178,
//                     member: 42096,
//                 } => Some(CommonItem::Teleporter(Teleporter::Glades)),
//                 UberIdentifier {
//                     group: 53632,
//                     member: 18181,
//                 } => Some(CommonItem::Teleporter(Teleporter::Wellspring)),
//                 UberIdentifier {
//                     group: 24922,
//                     member: 42531,
//                 } => Some(CommonItem::Teleporter(Teleporter::Burrows)),
//                 UberIdentifier {
//                     group: 58674,
//                     member: 7071,
//                 } => Some(CommonItem::Teleporter(Teleporter::WestWoods)),
//                 UberIdentifier {
//                     group: 58674,
//                     member: 1965,
//                 } => Some(CommonItem::Teleporter(Teleporter::EastWoods)),
//                 UberIdentifier {
//                     group: 28895,
//                     member: 54235,
//                 } => Some(CommonItem::Teleporter(Teleporter::Reach)),
//                 UberIdentifier {
//                     group: 18793,
//                     member: 38871,
//                 } => Some(CommonItem::Teleporter(Teleporter::Depths)),
//                 UberIdentifier {
//                     group: 945,
//                     member: 58183,
//                 } => Some(CommonItem::Teleporter(Teleporter::EastLuma)),
//                 UberIdentifier {
//                     group: 945,
//                     member: 1370,
//                 } => Some(CommonItem::Teleporter(Teleporter::WestLuma)),
//                 UberIdentifier {
//                     group: 58674,
//                     member: 10029,
//                 } => Some(CommonItem::Teleporter(Teleporter::FeedingGrounds)),
//                 UberIdentifier {
//                     group: 20120,
//                     member: 49994,
//                 } => Some(CommonItem::Teleporter(Teleporter::EastWastes)),
//                 UberIdentifier {
//                     group: 20120,
//                     member: 41398,
//                 } => Some(CommonItem::Teleporter(Teleporter::OuterRuins)),
//                 UberIdentifier {
//                     group: 10289,
//                     member: 4928,
//                 } => Some(CommonItem::Teleporter(Teleporter::InnerRuins)),
//                 UberIdentifier {
//                     group: 16155,
//                     member: 41465,
//                 } => Some(CommonItem::Teleporter(Teleporter::Willow)),
//                 UberIdentifier {
//                     group: 16155,
//                     member: 50867,
//                 } => Some(CommonItem::Teleporter(Teleporter::Shriek)),
//                 _ => None,
//             },
//             _ => None,
//         },
//         Action::Command(Command::Void(CommandVoid::Lookup { index })) => {
//             if is(*index, DefaultAction::HealthFragment, action_lookup) {
//                 Some(CommonItem::Resource(Resource::HealthFragment))
//             } else if is(*index, DefaultAction::EnergyFragment, action_lookup) {
//                 Some(CommonItem::Resource(Resource::EnergyFragment))
//             } else if is(*index, DefaultAction::GorlekOre, action_lookup) {
//                 Some(CommonItem::Resource(Resource::GorlekOre))
//             } else if is(*index, DefaultAction::Keystone, action_lookup) {
//                 Some(CommonItem::Resource(Resource::Keystone))
//             } else if is(*index, DefaultAction::ShardSlot, action_lookup) {
//                 Some(CommonItem::Resource(Resource::ShardSlot))
//             } else {
//                 None
//             }
//         }
//         _ => None,
//     }
// }

// pub fn spirit_light<T: LiteralTypes, R: Rng>(
//     amount: i32,
//     rng: &mut R,
//     action_lookup: &mut ActionLookup,
// ) -> Action<T> {
//     either_spirit_light(amount, rng, action_lookup, DefaultAction::SpiritLight)
// }
// pub fn remove_spirit_light<T: LiteralTypes, R: Rng>(
//     amount: i32,
//     rng: &mut R,
//     action_lookup: &mut ActionLookup,
// ) -> Action<T> {
//     either_spirit_light(amount, rng, action_lookup, DefaultAction::RemoveSpiritLight)
// }
// fn either_spirit_light<T: LiteralTypes, R: Rng>(
//     amount: i32,
//     rng: &mut R,
//     action_lookup: &mut ActionLookup,
//     default_action: DefaultAction,
// ) -> Action<T> {
//     let random_name = SPIRIT_LIGHT_NAMES.choose(rng).unwrap();
//     Action::Multi(vec![
//         Action::Command(Command::Void(CommandVoid::SetInteger {
//             id: i32::MAX,
//             value: CommandInteger::Constant { value: amount },
//         })),
//         Action::Command(Command::Void(CommandVoid::SetString {
//             id: i32::MAX,
//             value: CommandString::Constant {
//                 value: T::string_literal(random_name.to_string()),
//             },
//         })),
//         Action::Command(Command::Void(CommandVoid::Lookup {
//             index: action_lookup.default_action(default_action),
//         })),
//     ])
// }
// pub fn resource<T: LiteralTypes>(
//     resource: Resource,
//     action_lookup: &mut ActionLookup,
// ) -> Action<T> {
//     Action::Command(Command::Void(CommandVoid::Lookup {
//         index: action_lookup.default_action(match resource {
//             Resource::HealthFragment => DefaultAction::HealthFragment,
//             Resource::EnergyFragment => DefaultAction::EnergyFragment,
//             Resource::GorlekOre => DefaultAction::GorlekOre,
//             Resource::Keystone => DefaultAction::Keystone,
//             Resource::ShardSlot => DefaultAction::ShardSlot,
//         }),
//     }))
// }
// pub fn remove_resource<T: LiteralTypes>(
//     resource: Resource,
//     action_lookup: &mut ActionLookup,
// ) -> Action<T> {
//     Action::Command(Command::Void(CommandVoid::Lookup {
//         index: action_lookup.default_action(match resource {
//             Resource::HealthFragment => DefaultAction::RemoveHealthFragment,
//             Resource::EnergyFragment => DefaultAction::RemoveEnergyFragment,
//             Resource::GorlekOre => DefaultAction::RemoveGorlekOre,
//             Resource::Keystone => DefaultAction::RemoveKeystone,
//             Resource::ShardSlot => DefaultAction::RemoveShardSlot,
//         }),
//     }))
// }
// pub fn skill<T: LiteralTypes>(skill: Skill) -> Action<T> {
//     Action::Multi(vec![
//         skill_message(skill),
//         store_boolean(skill.uber_state(), true),
//     ])
// }
// pub fn remove_skill<T: LiteralTypes>(skill: Skill) -> Action<T> {
//     Action::Multi(vec![
//         remove_skill_message(skill),
//         store_boolean(skill.uber_state(), false),
//     ])
// }
// pub fn shard<T: LiteralTypes>(shard: Shard) -> Action<T> {
//     Action::Multi(vec![
//         shard_message(shard),
//         store_boolean(shard.uber_state(), true),
//     ])
// }
// pub fn remove_shard<T: LiteralTypes>(shard: Shard) -> Action<T> {
//     Action::Multi(vec![
//         remove_shard_message(shard),
//         store_boolean(shard.uber_state(), false),
//     ])
// }
// pub fn teleporter<T: LiteralTypes>(teleporter: Teleporter) -> Action<T> {
//     Action::Multi(vec![
//         teleporter_message(teleporter),
//         store_boolean(teleporter.uber_state(), true),
//     ])
// }
// pub fn remove_teleporter<T: LiteralTypes>(teleporter: Teleporter) -> Action<T> {
//     Action::Multi(vec![
//         remove_teleporter_message(teleporter),
//         store_boolean(teleporter.uber_state(), false),
//     ])
// }
// pub fn clean_water<T: LiteralTypes>() -> Action<T> {
//     Action::Multi(vec![
//         clean_water_message(),
//         store_boolean(CLEAN_WATER_UBER_STATE, true),
//     ])
// }
// pub fn remove_clean_water<T: LiteralTypes>() -> Action<T> {
//     Action::Multi(vec![
//         remove_clean_water_message(),
//         store_boolean(CLEAN_WATER_UBER_STATE, false),
//     ])
// }
// pub fn weapon_upgrade<T: LiteralTypes>(weapon_upgrade: WeaponUpgrade) -> Action<T> {
//     Action::Multi(vec![
//         weapon_upgrade_message(weapon_upgrade),
//         store_boolean(weapon_upgrade.uber_state(), true),
//     ])
// }
// pub fn remove_weapon_upgrade<T: LiteralTypes>(weapon_upgrade: WeaponUpgrade) -> Action<T> {
//     Action::Multi(vec![
//         remove_weapon_upgrade_message(weapon_upgrade),
//         store_boolean(weapon_upgrade.uber_state(), false),
//     ])
// }
//
// pub(crate) fn message_with<T: LiteralTypes>(command: CommandString<T>) -> Action<T> {
//     Action::Command(Command::Void(CommandVoid::ItemMessage { message: command }))
// }
// pub(crate) fn message<T: LiteralTypes>(value: String) -> Action<T> {
//     message_with(CommandString::Constant {
//         value: T::string_literal(value),
//     })
// }
// pub(crate) fn remove_message_with<T: LiteralTypes>(command: CommandString<T>) -> Action<T> {
//     Action::Command(Command::Void(CommandVoid::ItemMessage {
//         message: CommandString::Concatenate {
//             left: Box::new(CommandString::Constant {
//                 value: T::string_literal("@Removed ".to_string()),
//             }),
//             right: Box::new(CommandString::Concatenate {
//                 left: Box::new(command),
//                 right: Box::new(CommandString::Constant {
//                     value: T::string_literal("@".to_string()),
//                 }),
//             }),
//         },
//     }))
// }
// // TODO reuse display from above
// pub(crate) fn remove_message<T: LiteralTypes, D: Display>(value: D) -> Action<T> {
//     message(format!("@Removed {value}@"))
// }
//
// pub(crate) fn resource_message<T: LiteralTypes>(resource: Resource) -> Action<T> {
//     message(resource.to_string())
// }
// pub(crate) fn skill_message<T: LiteralTypes>(skill: Skill) -> Action<T> {
//     message(match skill {
//         Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => format!("#{skill}#"),
//         _ => format!("*{skill}*"),
//     })
// }
// pub(crate) fn remove_skill_message<T: LiteralTypes>(skill: Skill) -> Action<T> {
//     remove_message(skill)
// }
// pub(crate) fn shard_message<T: LiteralTypes>(shard: Shard) -> Action<T> {
//     message(format!("${shard}$"))
// }
// pub(crate) fn remove_shard_message<T: LiteralTypes>(shard: Shard) -> Action<T> {
//     remove_message(shard)
// }
// pub(crate) fn teleporter_message<T: LiteralTypes>(teleporter: Teleporter) -> Action<T> {
//     message(format!("#{teleporter} Teleporter#"))
// }
// pub(crate) fn remove_teleporter_message<T: LiteralTypes>(teleporter: Teleporter) -> Action<T> {
//     remove_message(format!("{teleporter} Teleporter"))
// }
// pub(crate) fn clean_water_message<T: LiteralTypes>() -> Action<T> {
//     message("*Clean Water*".to_string())
// }
// pub(crate) fn remove_clean_water_message<T: LiteralTypes>() -> Action<T> {
//     remove_message("Clean Water")
// }
// pub(crate) fn weapon_upgrade_message<T: LiteralTypes>(weapon_upgrade: WeaponUpgrade) -> Action<T> {
//     message(format!("#{weapon_upgrade}#"))
// }
// pub(crate) fn remove_weapon_upgrade_message<T: LiteralTypes>(
//     weapon_upgrade: WeaponUpgrade,
// ) -> Action<T> {
//     remove_message(weapon_upgrade)
// }
// pub(crate) fn store_boolean<T: LiteralTypes>(
//     uber_identifier: UberIdentifier,
//     value: bool,
// ) -> Action<T> {
//     Action::Command(Command::Void(CommandVoid::StoreBoolean {
//         uber_identifier: T::uber_identifier_literal(uber_identifier),
//         value: CommandBoolean::Constant { value },
//     }))
// }

// const SPIRIT_LIGHT_NAMES: &[&str] = &[
//     "Spirit Light",
//     "Gallons",
//     "Spirit Bucks",
//     "Gold",
//     "Geo",
//     "EXP",
//     "Experience",
//     "XP",
//     "Gil",
//     "GP",
//     "Dollars",
//     "Tokens",
//     "Tickets",
//     "Pounds Sterling",
//     "Brownie Points",
//     "Euros",
//     "Credits",
//     "Bells",
//     "Fish",
//     "Zenny",
//     "Pesos",
//     "Exalted Orbs",
//     "Hryvnia",
//     "Poké",
//     "Glod",
//     "Dollerydoos",
//     "Boonbucks",
//     "Pieces of Eight",
//     "Shillings",
//     "Farthings",
//     "Kalganids",
//     "Quatloos",
//     "Crowns",
//     "Solari",
//     "Widgets",
//     "Ori Money",
//     "Money",
//     "Cash",
//     "Munny",
//     "Nuyen",
//     "Rings",
//     "Rupees",
//     "Coins",
//     "Echoes",
//     "Sovereigns",
//     "Points",
//     "Drams",
//     "Doubloons",
//     "Spheres",
//     "Silver",
//     "Slivers",
//     "Rubies",
//     "Emeralds",
//     "Notes",
//     "Yen",
//     "Zloty",
//     "Likes",
//     "Comments",
//     "Subs",
//     "Bananas",
//     "Sapphires",
//     "Diamonds",
//     "Fun",
//     "Minerals",
//     "Vespine Gas",
//     "Sheep",
//     "Brick",
//     "Wheat",
//     "Wood",
//     "Quills",
//     "Bits",
//     "Bytes",
//     "Nuts",
//     "Bolts",
//     "Souls",
//     "Runes",
//     "Pons",
//     "Boxings",
//     "Stonks",
//     "Leaves",
//     "Marbles",
//     "Stamps",
//     "Hugs",
//     "Nobles",
//     "Socks",
// ];
