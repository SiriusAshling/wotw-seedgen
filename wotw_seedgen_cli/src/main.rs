mod seed;

use clap::{Args, Parser, Subcommand};
use seed::seed;
use std::fmt::{self, Debug};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Command::Seed { settings } => seed(settings),
    }
}

struct Error(String);
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: ToString> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Seed {
        #[command(flatten)]
        settings: SeedSettings,
    },
}
#[derive(Args)]
struct SeedSettings {
    #[arg(short, long, num_args = 0..)]
    presets: Vec<String>,
}
