use crate::{Error, SeedSettings};
use std::{
    fs::{self, File},
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};
use wotw_seedgen::{
    assets::{LocData, SnippetAccess, Source, StateData, UberStateData},
    generate_seed,
    logic_language::{ast::parse, output::Graph},
    settings::{PresetAccess, UniversePreset, UniverseSettings, WorldPreset},
};

pub(crate) fn seed(settings: SeedSettings) -> Result<(), Error> {
    let universe_preset = settings.into_universe_preset();
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

impl SeedSettings {
    fn into_universe_preset(self) -> UniversePreset {
        let Self { presets } = self;
        UniversePreset {
            info: None,
            includes: None,
            seed: None,
            world_settings: Some(vec![WorldPreset {
                includes: Some(presets.into_iter().collect()),
                ..Default::default()
            }]),
        }
    }
}

struct SnippetFileAccess;
impl SnippetAccess for SnippetFileAccess {
    fn read_snippet(&self, identifier: &str) -> Result<Source, String> {
        let mut path = PathBuf::from("snippets");
        path.push(identifier);
        path.set_extension("wotws");
        let id = path.to_string_lossy().to_string();

        let content = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;

        Ok(Source { id, content })
    }
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, String> {
        let mut full_path = PathBuf::from("snippets");
        full_path.push(path);
        fs::read(&full_path).map_err(|err| format!("Failed to read {}: {err}", full_path.display()))
    }
}

struct PresetFileAccess;
impl PresetAccess for PresetFileAccess {
    fn universe_preset(&self, identifier: &str) -> Result<UniversePreset, String> {
        let mut path = PathBuf::from("universe_presets");
        path.push(identifier);
        path.set_extension("json");
        serde_json::from_reader(BufReader::new(
            File::open(&path).map_err(|err| format!("Failed to read {}: {err}", path.display()))?,
        ))
        .map_err(|err| format!("Failed to parse {}: {err}", path.display()))
    }
    fn world_preset(&self, identifier: &str) -> Result<WorldPreset, String> {
        let mut path = PathBuf::from("world_presets");
        path.push(identifier);
        path.set_extension("json");
        serde_json::from_reader(BufReader::new(
            File::open(&path).map_err(|err| format!("Failed to read {}: {err}", path.display()))?,
        ))
        .map_err(|err| format!("Failed to parse {}: {err}", path.display()))
    }
}
