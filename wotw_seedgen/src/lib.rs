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
mod world;

// TODO update imports
// maybe since this is the top crate it should reexport everything?
pub use generator::*;
pub use world::*;
// pub use reach_check::reach_check;

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

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Env;
    use lazy_static::lazy_static;
    use std::io;
    use wotw_seedgen_logic_language::{
        ast::{parse, Areas},
        output::Graph,
    };
    use wotw_seedgen_settings::{
        Difficulty, PresetAccess, UniversePreset, UniverseSettings, WorldPreset,
    };
    use wotw_seedgen_static_assets::{
        LOC_DATA, PRESET_ACCESS, SNIPPET_ACCESS, STATE_DATA, UBER_STATE_DATA,
    };

    lazy_static! {
        pub static ref AREAS: Areas<'static> =
            parse(include_str!("../areas.wotw")).into_result().unwrap();
    }

    #[test]
    fn some_seeds() {
        env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
            .format_timestamp(None)
            .init();

        let mut universe_settings = UniverseSettings::new(String::default());
        let mut graph = Graph::compile(
            AREAS.clone(),
            LOC_DATA.clone(),
            STATE_DATA.clone(),
            &universe_settings.world_settings,
        )
        .into_result()
        .unwrap();
        eprintln!("Default settings");
        generate_seed(
            &graph,
            &*SNIPPET_ACCESS,
            &*UBER_STATE_DATA,
            &mut io::stderr(),
            &universe_settings,
        )
        .unwrap();

        universe_settings.world_settings[0].difficulty = Difficulty::Unsafe;
        graph = Graph::compile(
            AREAS.clone(),
            LOC_DATA.clone(),
            STATE_DATA.clone(),
            &universe_settings.world_settings,
        )
        .into_result()
        .unwrap();
        eprintln!("Unsafe");
        generate_seed(
            &graph,
            &*SNIPPET_ACCESS,
            &*UBER_STATE_DATA,
            &mut io::stderr(),
            &universe_settings,
        )
        .unwrap();

        universe_settings.world_settings[0].headers.extend([
            "bingo".to_string(),
            "bonus+".to_string(),
            "glades_done".to_string(),
            "launch_fragments".to_string(),
            "launch_from_bingo".to_string(),
            "no_combat".to_string(),
            "no_ks_doors".to_string(),
            "no_quests".to_string(),
            "no_willow_hearts".to_string(),
            "open_mode".to_string(),
            "spawn_with_sword".to_string(),
            "util_twillen".to_string(),
            "vanilla_opher_upgrades".to_string(),
            "bonus_opher_upgrades".to_string(),
        ]);

        for preset in ["gorlek", "rspawn"] {
            let preset = PRESET_ACCESS.world_preset(preset).unwrap();
            universe_settings.world_settings[0]
                .apply_world_preset(preset, &*PRESET_ACCESS)
                .unwrap();
        }

        let preset = UniversePreset {
            world_settings: Some(vec![WorldPreset::default(); 2]),
            ..UniversePreset::default()
        };
        universe_settings
            .apply_preset(preset, &*PRESET_ACCESS)
            .unwrap();

        eprintln!("Gorlek with headers");
        generate_seed(
            &graph,
            &*SNIPPET_ACCESS,
            &*UBER_STATE_DATA,
            &mut io::stderr(),
            &universe_settings,
        )
        .unwrap();
    }
}
