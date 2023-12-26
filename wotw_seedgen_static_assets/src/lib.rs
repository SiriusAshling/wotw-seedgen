use std::path::Path;

use lazy_static::lazy_static;
#[cfg(feature = "snippets")]
use rustc_hash::FxHashMap;
#[cfg(feature = "wotw_seedgen_assets")]
use wotw_seedgen_assets::*;
#[cfg(feature = "presets")]
use wotw_seedgen_settings::{PresetAccess, UniversePreset, WorldPreset};

// TODO compare bincode with other binary formats

#[cfg(feature = "loc_data")]
lazy_static! {
    pub static ref LOC_DATA: LocData =
        ciborium::from_reader(include_bytes!(concat!(env!("OUT_DIR"), "/loc_data")).as_slice())
            .unwrap();
}
#[cfg(feature = "state_data")]
lazy_static! {
    pub static ref STATE_DATA: StateData =
        ciborium::from_reader(include_bytes!(concat!(env!("OUT_DIR"), "/state_data")).as_slice())
            .unwrap();
}
#[cfg(feature = "uber_state_data")]
lazy_static! {
    pub static ref UBER_STATE_DATA: UberStateData = ciborium::from_reader(
        include_bytes!(concat!(env!("OUT_DIR"), "/uber_state_data")).as_slice()
    )
    .unwrap();
}
#[cfg(feature = "snippets")]
pub struct StaticSnippetAccess {
    snippets: FxHashMap<String, (String, String)>, // TODO can we really not have &'static str here :( Maybe with a different library. Many don't work with the preset format, but flexbuffers and bendy could be worth trying
}
#[cfg(feature = "snippets")]
lazy_static! {
    pub static ref SNIPPET_ACCESS: StaticSnippetAccess = StaticSnippetAccess {
        snippets: ciborium::from_reader(
            include_bytes!(concat!(env!("OUT_DIR"), "/snippets")).as_slice()
        )
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
    fn read_file(&self, _path: &Path) -> Result<Vec<u8>, String> {
        Err("cannot read arbitrary files with static file access".to_string())
    }
}
// TODO these all fail on the presets because of the conditional skips... maybe go to the playground with this
#[cfg(feature = "presets")]
pub struct StaticPresetAccess {
    universe_presets: FxHashMap<String, UniversePreset>,
    world_presets: FxHashMap<String, WorldPreset>,
}
#[cfg(feature = "presets")]
lazy_static! {
    pub static ref PRESET_ACCESS: StaticPresetAccess = StaticPresetAccess {
        universe_presets: ciborium::from_reader(
            include_bytes!(concat!(env!("OUT_DIR"), "/universe_presets")).as_slice()
        )
        .unwrap(),
        world_presets: ciborium::from_reader(
            include_bytes!(concat!(env!("OUT_DIR"), "/world_presets")).as_slice()
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "loc_data")]
    #[test]
    fn loc_data() {
        let _ = &*LOC_DATA;
    }

    #[cfg(feature = "state_data")]
    #[test]
    fn state_data() {
        let _ = &*STATE_DATA;
    }

    #[cfg(feature = "uber_state_data")]
    #[test]
    fn uber_state_data() {
        let _ = &*UBER_STATE_DATA;
    }

    #[cfg(feature = "snippets")]
    #[test]
    fn snippets() {
        let _ = &*SNIPPET_ACCESS;
    }

    #[cfg(feature = "presets")]
    #[test]
    fn presets() {
        let _ = &*PRESET_ACCESS;
    }
}
