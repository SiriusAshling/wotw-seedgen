mod args;
mod command;

use self::command::MemoryUsed;
use crate::{Command, Event, Trigger};
use wotw_seedgen_seed_language::output as input;

// TODO inline single command functions
// TODO dedup functions

pub trait Compile {
    type Output;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output;
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
            Self::Pseudo(trigger) => Trigger::Pseudo(trigger),
            Self::Binding(uber_identifier) => Trigger::Binding(uber_identifier),
            Self::Condition(command) => {
                Trigger::Condition(compile_into_lookup(command, command_lookup))
            }
        }
    }
}
