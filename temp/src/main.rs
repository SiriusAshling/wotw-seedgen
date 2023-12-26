#![feature(test)]
extern crate test;

use wotw_seedgen_settings::{HeaderConfig, WorldPreset};

fn main() {
    let mut preset: WorldPreset = WorldPreset::default();
    preset.hard = Some(true);
    // let bincode: WorldPreset = bincode::deserialize(&bincode::serialize(&preset).unwrap()).unwrap();
    // assert_eq!(bincode, preset);
    // let postcard: WorldPreset =
    //     postcard::from_bytes(&postcard::to_allocvec(&preset).unwrap()).unwrap();
    // assert_eq!(postcard, preset);
    let bendy: WorldPreset =
        bendy::serde::from_bytes(&bendy::serde::to_bytes(&preset).unwrap()).unwrap();
    assert_eq!(bendy, preset);
    let mut bytes = vec![];
    ciborium::into_writer(&preset, &mut bytes).unwrap();
    let ciborium: WorldPreset = ciborium::from_reader(bytes.as_slice()).unwrap();
    assert_eq!(ciborium, preset);
    // let rmp_serde: WorldPreset =
    //     rmp_serde::from_slice(&rmp_serde::to_vec(&preset).unwrap()).unwrap();
    // assert_eq!(rmp_serde, preset);
    let flexbuffers: WorldPreset =
        flexbuffers::from_slice(&flexbuffers::to_vec(&preset).unwrap()).unwrap();
    assert_eq!(flexbuffers, preset);
}

fn preset() -> WorldPreset {
    let mut preset: WorldPreset = WorldPreset::default();
    preset.includes = Some(('a'..='z').map(|c| c.to_string()).collect());
    preset.hard = Some(true);
    preset.header_config = Some(
        ('a'..='z')
            .map(|c| {
                let s = c.to_string();
                HeaderConfig {
                    header_name: format!("nnnnnnnnnnnnnnn{s}"),
                    config_name: format!("ccccccccc{s}"),
                    config_value: format!("vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv{s}"),
                }
            })
            .collect(),
    );
    preset
}

#[bench]
fn bendy(b: &mut test::Bencher) {
    let preset = preset();
    b.iter(|| {
        bendy::serde::from_bytes::<WorldPreset>(&bendy::serde::to_bytes(&preset).unwrap()).unwrap()
    })
}

#[bench]
fn ciborium(b: &mut test::Bencher) {
    let preset = preset();
    b.iter(|| {
        let mut bytes = vec![];
        ciborium::into_writer(&preset, &mut bytes).unwrap();
        ciborium::from_reader::<WorldPreset, _>(bytes.as_slice()).unwrap()
    })
}

#[bench]
fn flexbuffers(b: &mut test::Bencher) {
    let preset = preset();
    b.iter(|| {
        flexbuffers::from_slice::<WorldPreset>(&flexbuffers::to_vec(&preset).unwrap()).unwrap()
    })
}
