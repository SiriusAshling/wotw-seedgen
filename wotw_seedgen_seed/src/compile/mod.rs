mod args;
mod command;

use crate::{Command, Event, Trigger};
use args::Args;
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
            command: compile_into_lookup(self.action, command_lookup),
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

impl Compile for input::Action {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        match self {
            Self::Command(command) => command.compile(command_lookup),
            Self::Condition(condition) => condition.compile(command_lookup),
            Self::Multi(multi) => multi
                .into_iter()
                .flat_map(|action| action.compile(command_lookup))
                .collect(),
        }
    }
}

impl Compile for input::ActionCondition {
    type Output = Vec<Command>;

    fn compile(self, command_lookup: &mut Vec<Vec<Command>>) -> Self::Output {
        let index = compile_into_lookup(self.action, command_lookup);
        Args::new(1, command_lookup)
            .bool(self.condition)
            .call(Command::ExecuteIf { index })
    }
}
