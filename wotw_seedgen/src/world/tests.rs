use super::*;
use crate::{generator::ItemPool, tests::AREAS};
use rand_pcg::Pcg64Mcg;
use wotw_seedgen_settings::{Difficulty, UniverseSettings, DEFAULT_SPAWN};
use wotw_seedgen_static_assets::{LOC_DATA, STATE_DATA, UBER_STATE_DATA};

#[test]
fn reach_check() {
    let mut universe_settings = UniverseSettings::new(String::default());
    universe_settings.world_settings[0].difficulty = Difficulty::Gorlek;

    let graph = Graph::compile(
        AREAS.clone(),
        LOC_DATA.clone(),
        STATE_DATA.clone(),
        &universe_settings.world_settings,
    )
    .into_result()
    .unwrap();

    let spawn = graph.find_node(DEFAULT_SPAWN).unwrap();
    let uber_states = UberStates::new(&UBER_STATE_DATA);
    let mut world = World::new(
        &graph,
        spawn,
        &universe_settings.world_settings[0],
        uber_states,
    );
    let output = CompilerOutput::default();

    let mut pool = ItemPool::default();
    for item in pool.drain(&mut Pcg64Mcg::new(0xcafef00dd15ea5e5)) {
        world.simulate(&item, &output);
    }
    world.set_spirit_light(10000, &output);

    let reached = world
        .reached()
        .iter()
        .filter_map(|node| match node {
            Node::State(_) | Node::LogicalState(_) => None,
            _ => Some(node.identifier()),
        })
        .collect();

    let all_locations = LOC_DATA
        .entries
        .iter()
        .map(|location| location.identifier.as_str())
        .collect::<FxHashSet<_>>();

    if !(reached == all_locations) {
        let diff: Vec<_> = all_locations.difference(&reached).collect();
        eprintln!(
            "difference ({} / {} items): {:?}",
            reached.len(),
            all_locations.len(),
            diff
        );
    }

    assert_eq!(reached, all_locations);

    let spawn = graph.find_node("GladesTown.Teleporter").unwrap();
    let uber_states = UberStates::new(&UBER_STATE_DATA);
    let mut world = World::new_spawn(
        &graph,
        spawn,
        &universe_settings.world_settings[0],
        uber_states,
    );

    world.modify_resource(Resource::HealthFragment, 7, &output);
    world.modify_resource(Resource::EnergyFragment, 6, &output);
    world.set_skill(Skill::DoubleJump, true, &output);
    world.set_shard(Shard::TripleJump, true, &output);

    let reached = world
        .reached()
        .iter()
        .map(|node| node.identifier())
        .collect::<FxHashSet<_>>();
    assert_eq!(
        reached,
        [
            "GladesTown.UpdraftCeilingEX",
            "GladesTown.AboveTpEX",
            "GladesTown.BountyShard",
            "GladesTown.BelowHoleHutEX"
        ]
        .into_iter()
        .collect()
    );
}
