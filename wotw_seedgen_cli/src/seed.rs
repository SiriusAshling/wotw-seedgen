use crate::{
    files::{PresetFileAccess, SnippetFileAccess},
    Error, SeedSettings,
};
use std::{
    fs::{self, File},
    io::{self, Write},
};
use wotw_seedgen::{
    assets::{LocData, StateData, UberStateData},
    generate_seed,
    logic_language::{ast::parse, output::Graph},
    settings::UniverseSettings,
};

pub(crate) fn seed(settings: SeedSettings) -> Result<(), Error> {
    let universe_preset = settings.0;
    let mut settings = UniverseSettings::new("".to_string());
    settings.apply_preset(universe_preset, &PresetFileAccess)?;

    fs::create_dir_all("seeds")?;

    let source = fs::read_to_string("../wotw_seedgen/areas.wotw")
        .map_err(|err| format!("{err}: ../wotw_seedgen/areas.wotw"))?;
    let areas = parse(&source).into_result()?;
    let loc_data = LocData::from_reader(
        File::open("loc_data.csv").map_err(|err| format!("{err}: loc_data.csv"))?,
    )?;
    let state_data = StateData::from_reader(
        File::open("state_data.csv").map_err(|err| format!("{err}: state_data.csv"))?,
    )?;
    let mut uber_state_data = UberStateData::from_reader(
        File::open("uber_state_dump.json").map_err(|err| format!("{err}: uber_state_dump.json"))?,
    )?;
    uber_state_data.add_loc_data(loc_data.entries.clone());
    uber_state_data.add_state_data(state_data.entries.clone());
    let graph =
        Graph::compile(areas, loc_data, state_data, &settings.world_settings).into_result()?;
    let snippet_access = SnippetFileAccess;
    let mut seed = generate_seed(
        &graph,
        &snippet_access,
        &uber_state_data,
        &mut io::stderr(),
        &settings,
    )?;

    fs::write("seeds/spoiler.txt", seed.spoiler.to_string())?;

    let seed_world = seed.worlds.pop().unwrap();

    let mut seed = File::create("seeds/seed.wotwr")?;
    write!(seed, "wotwr,0.0.1,p\n")?;
    let json = serde_json::to_string_pretty(&seed_world)?.replace("File", "Path");
    write!(seed, "{json}")?;

    Ok(())
}
