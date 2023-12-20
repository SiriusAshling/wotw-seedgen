use crate::{Command, Compile};
use wotw_seedgen_seed_language::output::{
    CommandBoolean, CommandFloat, CommandInteger, CommandString, CommandZone,
};

// TODO brainstorming the memory overwrite problem
// 1 + (2 + (3 + 4))
// should compile to
// SetInteger 1
// CopyInteger 0 3
// SetInteger 2
// CopyInteger 0 2
// SetInteger 3
// CopyInteger 0 1
// SetInteger 4
// ArithmeticInteger 0
// CopyInteger 2 1
// ArithmeticInteger 0
// CopyInteger 3 1
// ArithmeticInteger 0

pub struct Args<'a> {
    commands: Vec<Command>,
    command_lookup: &'a mut Vec<Vec<Command>>,
    bool_index: usize,
    int_index: usize,
    float_index: usize,
    string_index: usize,
}
impl<'a> Args<'a> {
    #[inline]
    pub fn new(arg_count: usize, command_lookup: &'a mut Vec<Vec<Command>>) -> Self {
        Self {
            commands: Vec::with_capacity(arg_count * 2),
            command_lookup,
            bool_index: 1,
            int_index: 1,
            float_index: 1,
            string_index: 1,
        }
    }

    pub fn bool(mut self, arg: CommandBoolean) -> Self {
        // TODO fundamental problem here: To calculate, for example, (a + b) + (c + d), you need at least three memory locations
        // to get the result of one side out of the way while calculating the other side
        // TODO once deduplication exists, check if moving these out into function calls is ever worth it
        self.commands.extend(arg.compile(self.command_lookup));
        self.commands.push(Command::CopyBoolean(0, self.bool_index));
        self.bool_index += 1;
        self
    }

    pub fn int(mut self, arg: CommandInteger) -> Self {
        self.commands.extend(arg.compile(self.command_lookup));
        self.commands.push(Command::CopyInteger(0, self.int_index));
        self.int_index += 1;
        self
    }

    pub fn float(mut self, arg: CommandFloat) -> Self {
        self.commands.extend(arg.compile(self.command_lookup));
        self.commands.push(Command::CopyFloat(0, self.float_index));
        self.float_index += 1;
        self
    }

    pub fn string(mut self, arg: CommandString) -> Self {
        self.commands.extend(arg.compile(self.command_lookup));
        self.commands
            .push(Command::CopyString(0, self.string_index));
        self.string_index += 1;
        self
    }

    pub fn zone(mut self, arg: CommandZone) -> Self {
        self.commands.extend(arg.compile(self.command_lookup));
        self.commands.push(Command::CopyInteger(0, self.int_index));
        self.int_index += 1;
        self
    }

    pub fn call(mut self, command: Command) -> Vec<Command> {
        *self.commands.last_mut().unwrap() = command;
        self.commands
    }
}
