#[cfg(feature = "lazy_static")]
use lazy_static::lazy_static;
#[cfg(feature = "snippets")]
use rustc_hash::FxHashMap;
#[cfg(feature = "wotw_seedgen_assets")]
use wotw_seedgen_assets::*;
#[cfg(feature = "presets")]
use wotw_seedgen_settings::{PresetAccess, UniversePreset, WorldPreset};

#[cfg(feature = "loc_data")]
lazy_static! {
    pub static ref LOC_DATA: LocData =
        bincode::deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/loc_data"))).unwrap();
}
#[cfg(feature = "state_data")]
lazy_static! {
    pub static ref STATE_DATA: StateData =
        bincode::deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/state_data"))).unwrap();
}
#[cfg(feature = "uber_state_data")]
lazy_static! {
    pub static ref UBER_STATE_DATA: UberStateData =
        bincode::deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/uber_state_data"))).unwrap();
}
#[cfg(feature = "snippets")]
pub struct StaticSnippetAccess {
    snippets: FxHashMap<&'static str, (String, String)>,
}
#[cfg(feature = "snippets")]
lazy_static! {
    pub static ref SNIPPET_ACCESS: StaticSnippetAccess = StaticSnippetAccess {
        snippets: bincode::deserialize(include_bytes!(concat!(env!("OUT_DIR"), "/snippets")))
            .unwrap()
    };
}
#[cfg(feature = "snippets")]
impl SnippetAccess for StaticSnippetAccess {
    fn read_snippet(&self, identifier: &str) -> Result<Source, String> {
        self.snippets
            .get(identifier)
            .cloned()
            .map(|(id, content)| Source::new(id, content))
            .ok_or_else(|| format!("unknown snippet \"{identifier}\""))
    }
}
#[cfg(feature = "presets")]
pub struct StaticPresetAccess {
    universe_presets: FxHashMap<&'static str, UniversePreset>,
    world_presets: FxHashMap<&'static str, WorldPreset>,
}
#[cfg(feature = "presets")]
lazy_static! {
    pub static ref PRESET_ACCESS: StaticPresetAccess = StaticPresetAccess {
        universe_presets: bincode::deserialize(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/universe_presets"
        )))
        .unwrap(),
        world_presets: bincode::deserialize(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/world_presets"
        )))
        .unwrap()
    };
}
#[cfg(feature = "presets")]
impl PresetAccess for StaticPresetAccess {
    fn universe_preset(&self, identifier: &str) -> Result<UniversePreset, String> {
        self.universe_presets
            .get(identifier)
            .cloned()
            .ok_or_else(|| format!("unknown universe preset \"{identifier}\""))
    }
    fn world_preset(&self, identifier: &str) -> Result<WorldPreset, String> {
        self.world_presets
            .get(identifier)
            .cloned()
            .ok_or_else(|| format!("unknown world preset \"{identifier}\""))
    }
}
