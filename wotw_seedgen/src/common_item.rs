use wotw_seedgen_data::{uber_identifier, MapIcon, Shard, Skill, Teleporter, WeaponUpgrade};
use wotw_seedgen_seed_language::output::{
    ArithmeticOperator, CommandBoolean, CommandFloat, CommandInteger, CommandVoid, Operation,
};

pub enum CommonItem {
    SpiritLight(usize),
    GorlekOre,
    Keystone,
    ShardSlot,
    HealthFragment,
    EnergyFragment,
    Skill(Skill),
    Shard(Shard),
    Teleporter(Teleporter),
    CleanWater,
    WeaponUpgrade(WeaponUpgrade),
}

impl CommonItem {
    // TODO delete?
    // pub fn into_command(self) -> CommandVoid {
    //     match self {
    //         CommonItem::SpiritLight(amount) => {
    //             compile::add_integer_value(uber_identifier::SPIRIT_LIGHT, amount)
    //         }
    //         GorlekOre,
    //         Keystone,
    //         ShardSlot,
    //         HealthFragment,
    //         EnergyFragment,
    //         CommonItem::Skill(skill) => compile::set_boolean_value(skill.uber_identifier(), true),
    //         CommonItem::Shard(shard) => compile::set_boolean_value(shard.uber_identifier(), true),
    //         CommonItem::Teleporter(teleporter) => {
    //             compile::set_boolean_value(teleporter.uber_identifier(), true)
    //         }
    //         CommonItem::CleanWater => {
    //             compile::set_boolean_value(uber_identifier::CLEAN_WATER, true)
    //         }
    //         CommonItem::WeaponUpgrade(weapon_upgrade) => {
    //             compile::set_boolean_value(weapon_upgrade.uber_identifier(), true)
    //         }
    //     }
    // }

    // TODO could do an iterator here and it would probably be a performance advantage
    pub fn from_command(command: &CommandVoid) -> Vec<Self> {
        match command {
            CommandVoid::Multi { commands } => {
                commands.iter().flat_map(Self::from_command).collect()
            }
            CommandVoid::StoreBoolean {
                uber_identifier,
                value: CommandBoolean::Constant { value: true },
                ..
            } => {
                if let Some(skill) = Skill::from_uber_identifier(*uber_identifier) {
                    vec![CommonItem::Skill(skill)]
                } else if let Some(shard) = Shard::from_uber_identifier(*uber_identifier) {
                    vec![CommonItem::Shard(shard)]
                } else if let Some(teleporter) = Teleporter::from_uber_identifier(*uber_identifier)
                {
                    vec![CommonItem::Teleporter(teleporter)]
                } else if *uber_identifier == uber_identifier::CLEAN_WATER {
                    vec![CommonItem::CleanWater]
                } else if let Some(weapon_upgrade) =
                    WeaponUpgrade::from_uber_identifier(*uber_identifier)
                {
                    vec![CommonItem::WeaponUpgrade(weapon_upgrade)]
                } else {
                    vec![]
                }
            }
            CommandVoid::StoreInteger {
                uber_identifier,
                value: CommandInteger::Arithmetic { operation },
                ..
            } => match &**operation {
                Operation {
                    left:
                        CommandInteger::FetchInteger {
                            uber_identifier: fetch_identifier,
                        },
                    operator: ArithmeticOperator::Add,
                    right: CommandInteger::Constant { value: amount },
                } if fetch_identifier == uber_identifier && *amount > 0 => match *uber_identifier {
                    uber_identifier::SPIRIT_LIGHT => {
                        vec![CommonItem::SpiritLight(*amount as usize)]
                    }
                    uber_identifier::GORLEK_ORE if *amount == 1 => vec![CommonItem::GorlekOre],
                    uber_identifier::KEYSTONES if *amount == 1 => vec![CommonItem::Keystone],
                    uber_identifier::SHARD_SLOTS if *amount == 1 => vec![CommonItem::ShardSlot],
                    uber_identifier::MAX_HEALTH if *amount == 5 => vec![CommonItem::HealthFragment],
                    _ => vec![],
                },
                _ => vec![],
            },
            CommandVoid::StoreFloat {
                uber_identifier: uber_identifier::MAX_ENERGY,
                value: CommandFloat::Arithmetic { operation },
                ..
            } => match &**operation {
                Operation {
                    left:
                        CommandFloat::FetchFloat {
                            uber_identifier: uber_identifier::MAX_ENERGY,
                        },
                    operator: ArithmeticOperator::Add,
                    right: CommandFloat::Constant { value },
                } if *value == 0.5 => vec![CommonItem::EnergyFragment],
                _ => vec![],
            },
            _ => vec![],
        }
    }

    pub const fn map_icon(&self) -> MapIcon {
        match self {
            CommonItem::SpiritLight(_) => MapIcon::Experience,
            CommonItem::GorlekOre => MapIcon::Ore,
            CommonItem::Keystone => MapIcon::Keystone,
            CommonItem::ShardSlot => MapIcon::ShardSlotUpgrade,
            CommonItem::HealthFragment => MapIcon::HealthFragment,
            CommonItem::EnergyFragment => MapIcon::EnergyFragment,
            CommonItem::Skill(_) => MapIcon::AbilityPedestal,
            CommonItem::Shard(_) => MapIcon::SpiritShard,
            CommonItem::Teleporter(_) => MapIcon::Teleporter,
            CommonItem::CleanWater => MapIcon::CleanWater,
            CommonItem::WeaponUpgrade(_) => MapIcon::BonusItem, // TODO is this good?
        }
    }

    pub const fn shop_price(&self) -> f32 {
        match self {
            CommonItem::GorlekOre | CommonItem::Keystone => 100.,
            CommonItem::ShardSlot => 250.,
            CommonItem::HealthFragment => 200.,
            CommonItem::EnergyFragment => 150.,
            CommonItem::Skill(skill) => match skill {
                Skill::WaterBreath | Skill::Regenerate | Skill::Seir => 200.,
                Skill::GladesAncestralLight | Skill::InkwaterAncestralLight => 300.,
                Skill::Blaze => 420.,
                Skill::Launch => 800.,
                _ => 500.,
            },
            CommonItem::CleanWater => 500.,
            CommonItem::Teleporter(_) | CommonItem::Shard(_) => 250.,
            _ => 200.,
        }
    }
}