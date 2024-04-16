use crate::generate_seed;

use env_logger::Env;
use lazy_static::lazy_static;
use log::info;
use wotw_seedgen_assets::{PresetAccess, UniversePreset, WorldPreset};
use wotw_seedgen_logic_language::{
    ast::{parse, Areas},
    output::Graph,
};
use wotw_seedgen_settings::{Difficulty, UniverseSettings};
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
        .is_test(true)
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
    info!("Testing Default settings");
    generate_seed(
        &graph,
        &*UBER_STATE_DATA,
        &*SNIPPET_ACCESS,
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
    info!("Testing Unsafe");
    generate_seed(
        &graph,
        &*UBER_STATE_DATA,
        &*SNIPPET_ACCESS,
        &universe_settings,
    )
    .unwrap();

    universe_settings.world_settings[0].snippets.extend([
        "bingo".to_string(),
        "bonus+".to_string(),
        "glades_done".to_string(),
        "launch_fragments".to_string(),
        "launch_from_bingo".to_string(),
        "no_combat".to_string(),
        "no_ks_doors".to_string(),
        "no_quests".to_string(),
        "no_willow_hearts".to_string(),
        // "open_mode".to_string(), // TODO migrate?
        "spawn_with_sword".to_string(),
        "util_twillen".to_string(),
        "vanilla_opher_upgrades".to_string(),
        "bonus_opher_upgrades".to_string(),
    ]);

    for preset in ["gorlek", "rspawn"] {
        let preset = PRESET_ACCESS.world_preset(preset).unwrap();
        preset
            .apply(&mut universe_settings.world_settings[0], &*PRESET_ACCESS)
            .unwrap();
    }

    let preset = UniversePreset {
        world_settings: Some(vec![WorldPreset::default(); 2]),
        ..UniversePreset::default()
    };
    preset
        .apply(&mut universe_settings, &*PRESET_ACCESS)
        .unwrap();

    info!("Testing multiworld Gorlek with headers");
    generate_seed(
        &graph,
        &*UBER_STATE_DATA,
        &*SNIPPET_ACCESS,
        &universe_settings,
    )
    .unwrap();
}