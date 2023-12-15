use ansi_term::Colour;
use wotw_seedgen_data::{Teleporter, UberIdentifier, Zone};
use wotw_seedgen_seed_language::output::{Action, Command, CommonItem};

// TODO remove this module

pub const SPAWN_GRANTS: &[(&str, Action)] = &[(
    "EastPools.Teleporter",
    Action::Command(Command::Custom(CommonItem::Teleporter(
        Teleporter::CentralLuma,
    ))),
)];
pub const RELIC_ZONES: &[Zone] = &[
    Zone::Marsh,
    Zone::Hollow,
    Zone::Glades,
    Zone::Wellspring,
    Zone::Woods,
    Zone::Reach,
    Zone::Depths,
    Zone::Pools,
    Zone::Wastes,
    Zone::Willow,
    Zone::Burrows,
];
pub const KEYSTONE_DOORS: &[(&str, i32)] = &[
    ("MarshSpawn.KeystoneDoor", 2),
    ("HowlsDen.KeystoneDoor", 2),
    ("MarshPastOpher.EyestoneDoor", 2),
    ("MidnightBurrows.KeystoneDoor", 4),
    ("WoodsEntry.KeystoneDoor", 2),
    ("WoodsMain.KeystoneDoor", 4),
    ("LowerReach.KeystoneDoor", 4),
    ("UpperReach.KeystoneDoor", 4),
    ("UpperDepths.EntryKeystoneDoor", 2),
    ("UpperDepths.CentralKeystoneDoor", 2),
    ("UpperPools.KeystoneDoor", 4),
    ("UpperWastes.KeystoneDoor", 2),
];

pub const WISP_STATES: &[UberIdentifier] = &[
    UberIdentifier::new(46462, 59806),
    UberIdentifier::new(945, 49747),
    UberIdentifier::new(28895, 25522),
    UberIdentifier::new(18793, 63291),
    UberIdentifier::new(10289, 22102),
];

pub const SPAWN_SLOTS: usize = 7;
pub const PREFERRED_SPAWN_SLOTS: usize = 3;
const _: usize = SPAWN_SLOTS - PREFERRED_SPAWN_SLOTS; // check that SPAWN_SLOTS >= PREFERRED_SPAWN_SLOTS
pub const RESERVE_SLOTS: usize = 1; // how many slots to reserve after random placements for the next iteration
pub const PLACEHOLDER_SLOTS: usize = 25; // how many slots to keep as placeholders for bigger progressions
pub const RETRIES: u16 = 10; // How many retries to allow when generating a seed
pub const RANDOM_PROGRESSION: f64 = 0.4; // How likely to choose a progression item as random placement
pub const UNSHARED_ITEMS: usize = 5; // How many items to place per world that are guaranteed not being sent to another world

pub const HEADER_INDENT: usize = 24; // Which column to align header descriptions on
pub const NAME_COLOUR: Colour = Colour::Yellow;
pub const UBERSTATE_COLOUR: Colour = Colour::Cyan;
