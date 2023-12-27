#![allow(clippy::too_many_arguments)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_bool)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::struct_excessive_bools)]

mod common_item;
mod constants;
mod generator;
mod inventory;
mod logical_difficulty;
mod orbs;
#[cfg(test)]
mod tests;
mod world;

// TODO update imports
// maybe since this is the top crate it should reexport everything?
pub use generator::{generate_seed, item_pool, spoiler, Seed};
pub use world::{Player, Simulate, UberStates, World};
// pub use reach_check::reach_check;

pub(crate) use world::{filter_redundancies, node_condition, node_trigger};

// TODO use this and also set the other metadata: current world, format version, settings
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"));

mod log {
    macro_rules! trace {
        ($($arg:tt)+) => {{
            #[cfg(any(feature = "log", test))]
            ::log::trace!($($arg)+)
        }}
    }
    pub(crate) use trace;
    macro_rules! info {
        ($($arg:tt)+) => {{
            #[cfg(any(feature = "log", test))]
            ::log::info!($($arg)+)
        }}
    }
    pub(crate) use info;
    macro_rules! warning {
        ($($arg:tt)+) => {{
            #[cfg(any(feature = "log", test))]
            ::log::warn!($($arg)+)
        }}
    }
    pub(crate) use warning; // warn is a built in attribute
}
