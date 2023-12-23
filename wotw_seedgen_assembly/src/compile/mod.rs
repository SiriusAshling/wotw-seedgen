mod args;
mod command;

use std::collections::hash_map::Entry;

use self::command::MemoryUsed;
use crate::{Command, Event, SeedWorld, Spawn, Trigger};
use rustc_hash::FxHashMap;
use wotw_seedgen_seed_language::output::{self as input, CompilerOutput, StringOrPlaceholder};

// TODO dedup functions?

pub trait Compile {
    type Output;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output;
}

impl SeedWorld {
    pub fn from_intermediate_output(output: CompilerOutput) -> Self {
        let mut flags = output
            .flags
            .into_iter()
            .map(|string| unwrap_string_placeholder(string))
            .collect::<Vec<_>>();
        flags.sort();
        let spawn = output
            .spawn
            .map(|position| Spawn {
                position,
                identifier: "Custom Spawn".to_string(), // TODO
            })
            .unwrap_or_default();

        let mut command_lookup = vec![];
        command_lookup.resize_with(output.command_lookup.len(), Default::default);
        for (index, command) in output.command_lookup.into_iter().enumerate() {
            command_lookup[index] = command.compile(&mut command_lookup).0;
        }

        let mut events = FxHashMap::<_, usize>::default();
        events.reserve(output.events.len());
        for event in output.events {
            let trigger = event.trigger.compile(&mut command_lookup);
            match events.entry(trigger) {
                Entry::Occupied(occupied) => {
                    let (new, _) = event.command.compile(&mut command_lookup);
                    let existing = &mut command_lookup[*occupied.get()];
                    existing.extend(new);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(compile_into_lookup(event.command, &mut command_lookup));
                }
            }
        }
        let events = events
            .into_iter()
            .map(|(trigger, command)| Event(trigger, command))
            .collect();

        Self {
            flags,
            spawn,
            events,
            command_lookup,
            metadata: (),
        }
    }
}

// TODO if the command is just one single execute, we should follow it and insert the execute index directly in the event
// TODO we should concatenate all commands on identical triggers as an optimization and so there's reliable ordering
impl Compile for input::Event {
    type Output = Event;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        Event(
            self.trigger.compile(command_lookup),
            compile_into_lookup(self.command, command_lookup),
        )
    }
}

impl Compile for input::Trigger {
    type Output = Trigger;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::ClientEvent(trigger) => Trigger::ClientEvent(trigger),
            Self::Binding(uber_identifier) => Trigger::Binding(uber_identifier),
            Self::Condition(command) => {
                Trigger::Condition(compile_into_lookup(command, command_lookup))
            }
        }
    }
}

fn compile_into_lookup<I: Compile<Output = (Vec<Command>, MemoryUsed)>>(
    input: I,
    command_lookup: &mut Vec<Vec<Command>>,
) -> usize {
    // TODO are we allowed to ignore memoryused here?
    let (command, _) = input.compile(command_lookup);
    let index = command_lookup.len();
    command_lookup.push(command);
    index
}

fn unwrap_string_placeholder(value: StringOrPlaceholder) -> String {
    match value {
        StringOrPlaceholder::Value(value) => value,
        _ => panic!("Unresolved string placeholder"),
    }
}
