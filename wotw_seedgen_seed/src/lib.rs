pub use wotw_seedgen_data as data;
pub use wotw_seedgen_seed_language as seed_language;
pub use wotw_seedgen_settings as settings;

pub mod assembly;

mod compile;
mod package;

use assembly::Assembly;
use compile::intermediate::{compile_command_lookup, compile_events};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use wotw_seedgen_data::Position;
use wotw_seedgen_seed_language::output::IntermediateOutput;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub const FORMAT_VERSION: &str = "0.0.0";

// TODO the settings crate doesn't implement Eq on its types, why?
/// Everything necessary to package a seed for one world
#[derive(Debug, Clone, PartialEq)]
pub struct Seed {
    pub format_version: &'static str,
    pub preload: Preload,
    pub assembly: Assembly,
    pub assets: FxHashMap<String, Vec<u8>>,
}

impl Seed {
    pub fn new(mut output: IntermediateOutput) -> Self {
        let mut command_lookup = compile_command_lookup(output.command_lookup);
        let events = compile_events(output.events, &mut command_lookup);
        output.tags.sort();

        Self {
            format_version: FORMAT_VERSION,
            preload: Preload {
                tags: output.tags,
                spawn: output.spawn.unwrap_or(Position::new(-799., -4310.)),
                slug: String::new(),
            },
            assembly: Assembly {
                events,
                command_lookup,
            },
            assets: output.icons.into_iter().collect(), // TODO decide on a consistent data structure
        }
    }
}

/// Contains necessary information while preloading in the main menu.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preload {
    /// May be displayed as brief summary of the settings
    pub tags: Vec<String>,
    /// For preloading before starting the savefile
    pub spawn: Position,
    /// Identical for seeds with the same universe settings (including the rng seed)
    pub slug: String,
}