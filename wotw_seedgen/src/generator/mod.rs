pub mod item_pool;
pub mod spoiler;

mod cost;
mod placement;
mod spirit_light;
mod string_placeholders;

use self::spoiler::SeedSpoiler;
use crate::{
    generator::placement::generate_placements,
    log::{info, trace, warning},
    logical_difficulty,
    world::World,
    UberStates,
};
use rand::{seq::IteratorRandom, Rng};
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;
use std::{io, iter};
use wotw_seedgen_assembly::SeedWorld;
use wotw_seedgen_assets::{SnippetAccess, UberStateData};
use wotw_seedgen_logic_language::output::Graph;
use wotw_seedgen_seed_language::{compile::Compiler, output::CompilerOutput};
use wotw_seedgen_settings::{Spawn, UniverseSettings, WorldSettings};

/// End Result of seed generation
pub struct Seed {
    /// Seed data per world
    pub worlds: Vec<SeedWorld>,
    /// Spoiler data for the generation process
    pub spoiler: SeedSpoiler,
}

const RETRIES: u16 = 10; // How many retries to allow when generating a seed

pub fn generate_seed<F: SnippetAccess, W: io::Write>(
    graph: &Graph,
    snippet_access: &F,
    uber_state_data: &UberStateData,
    // TODO we don't fully support writing to an arbitrary output, so maybe this should be made consistent with that
    // Maybe it could be put into the return type?
    write_errors: &mut W,
    settings: &UniverseSettings,
) -> Result<Seed, String> {
    let mut rng: Pcg64Mcg = Seeder::from(&settings.seed).make_rng();
    trace!("Seeded RNG with \"{}\"", settings.seed);

    let snippet_outputs = settings
        .world_settings
        .iter()
        .map(|world_settings| {
            let compiler = Compiler::new(
                &mut rng,
                snippet_access,
                uber_state_data,
                world_settings.snippet_config.clone(),
            );
            // TODO this is inefficient because we probably do a lot of redundant work between the worlds
            let output = parse_snippets(&world_settings.snippets, compiler, write_errors)?;
            Ok((world_settings, output))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let uber_states = UberStates::new(uber_state_data);

    for attempt in 1..=RETRIES {
        trace!("Attempt #{attempt} to generate");

        let worlds = snippet_outputs
            .iter()
            .map(|(world_settings, output)| {
                let spawn = choose_spawn(graph, world_settings, &mut rng)?;
                if output.spawn.is_some() {
                    warning!("A Snippet attempted to set spawn");
                }
                let world = World::new_spawn(graph, spawn, world_settings, uber_states.clone());
                Ok((world, output.clone()))
            })
            .collect::<Result<Vec<_>, String>>()?;

        match generate_placements(&mut rng, worlds) {
            Ok(seed) => {
                if attempt > 1 {
                    info!(
                        "Generated seed after {attempt} attempts{}",
                        if attempt <= RETRIES / 2 {
                            ""
                        } else {
                            " (phew)"
                        }
                    );
                }

                return Ok(seed);
            }
            #[cfg_attr(not(feature = "log"), allow(unused_variables))]
            Err(err) => warning!("{err}"),
        }
    }

    Err(format!(
        "All {RETRIES} attempts to generate a seed failed :("
    ))
}

const SEED_FAILED_MESSAGE: &str = "Failed to seed child RNG";

fn parse_snippets<W: io::Write>(
    snippets: &[String],
    mut compiler: Compiler,
    write_errors: &mut W,
) -> Result<CompilerOutput, String> {
    for identifier in iter::once("seed_core").chain(snippets.iter().map(String::as_str)) {
        compiler
            .compile_snippet(identifier)
            .map_err(|err| format!("Failed to read snippet \"{identifier}\": {err}"))?;
    }

    let output = compiler
        .finish(write_errors)
        .map_err(|err| format!("Failed to write errors: {err}"))?;

    if output.success {
        Ok(output)
    } else {
        Err("Snippet compilation failed, see errors for more details".to_string())
    }
}

fn choose_spawn(
    graph: &Graph,
    world_settings: &WorldSettings,
    rng: &mut impl Rng,
) -> Result<usize, String> {
    let spawn = match &world_settings.spawn {
        Spawn::Random => {
            let spawns = logical_difficulty::spawn_locations(world_settings.difficulty);
            graph
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, node)| spawns.contains(&node.identifier()))
                .choose(rng)
                .ok_or_else(|| String::from("No valid spawn locations available"))?
                .0
        }
        Spawn::FullyRandom => {
            graph
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, node)| node.can_spawn())
                .choose(rng)
                .ok_or_else(|| String::from("No valid spawn locations available"))?
                .0
        }
        Spawn::Set(spawn_loc) => {
            let (index, node) = graph
                .nodes
                .iter()
                .enumerate()
                .find(|(_, node)| node.identifier() == spawn_loc)
                .ok_or_else(|| format!("Spawn {} not found", spawn_loc))?;
            if !node.can_spawn() {
                return Err(format!("{} is not a valid spawn", spawn_loc));
            }
            index
        }
    };
    Ok(spawn)
}

// TODO migrate
// fn block_spawn_sets(preplacement: &header::Pickup, world: &mut World) {
//     if let Item::UberState(uber_state_item) = &preplacement.item {
//         if preplacement.trigger != UberStateTrigger::spawn() {
//             return;
//         }
//         if let UberStateOperator::Value(value) = &uber_state_item.operator {
//             for trigger in world
//                 .graph
//                 .nodes
//                 .iter()
//                 .filter(|node| node.can_place())
//                 .filter_map(|node| node.trigger())
//                 .filter(|trigger| trigger.check(uber_state_item.identifier, value.to_f32()))
//             {
//                 trace!(
//                     "adding an empty pickup at {} to prevent placements",
//                     trigger.code()
//                 );
//                 let mut message = Message::new(String::new());
//                 message.frames = Some(0);
//                 message.quiet = true;
//                 message.noclear = true;
//                 let null_item = Item::Message(message);
//                 world.preplace(trigger.clone(), null_item);
//             }
//         }
//     }
// }
