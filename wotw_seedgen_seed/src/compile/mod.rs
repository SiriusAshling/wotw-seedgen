mod args;
mod command;

use crate::{Command, Event, Trigger};
use wotw_seedgen_seed_language::output as input;

pub trait Compile {
    type Output;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output;
}

fn compile_into_lookup<I: Compile<Output = Vec<Command>>>(
    input: I,
    command_lookup: &mut Vec<Vec<Command>>,
) -> usize {
    let command = input.compile(command_lookup);
    let index = command_lookup.len();
    command_lookup.push(command);
    index
}

impl Compile for input::Event {
    type Output = Event;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        Event {
            trigger: self.trigger.compile(command_lookup),
            command: compile_into_lookup(self.command, command_lookup),
        }
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
