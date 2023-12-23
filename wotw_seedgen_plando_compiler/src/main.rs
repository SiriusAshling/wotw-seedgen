use clap::Parser;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::{
    fmt::{self, Debug},
    fs::{self, File},
    io::{self, ErrorKind},
    mem,
    path::{Path, PathBuf},
};
use wotw_seedgen_assembly::{Command, SeedWorld};
use wotw_seedgen_assets::{SnippetAccess, Source};
use wotw_seedgen_seed_language::{compile::Compiler, output::DebugOutput};
use wotw_seedgen_static_assets::UBER_STATE_DATA;

fn main() -> Result<(), Error> {
    // TODO remove
    // bugsalot::debugger::wait_until_attached(None)?;

    fs::create_dir_all("seeds/out")?;

    let folder = Cli::parse().folder;
    let mut rng = rand::thread_rng();
    let files = Files { folder };
    let mut compiler: Compiler<'_, '_, Files> =
        Compiler::new(&mut rng, &files, &UBER_STATE_DATA, Default::default());
    compiler.debug();
    compiler.compile_snippet("main")?;
    let mut output = compiler.finish(&mut io::stderr())?;
    let compiler_data = mem::take(&mut output.debug).unwrap();
    let seed = SeedWorld::from_intermediate_output(output);

    // TODO remove
    // fs::write("seeds/out/out.intermediate", format!("{output:#?}"))?;

    let file = File::create("seeds/out/out_release.wotwr")?;
    seed.package(file)?;

    let indexed_lookup = seed.command_lookup.iter().cloned().enumerate().collect();

    let seed = SeedWorld {
        flags: seed.flags,
        spawn: seed.spawn,
        events: seed.events,
        command_lookup: seed.command_lookup,
        metadata: Metadata {
            compiler_data,
            indexed_lookup,
        },
    };
    let file = File::create("seeds/out/out.wotwr")?;
    seed.package_pretty(file)?;

    Ok(())
}

#[derive(Serialize)]
struct Metadata {
    compiler_data: DebugOutput,
    indexed_lookup: FxHashMap<usize, Vec<Command>>,
}

struct Files {
    folder: PathBuf,
}
impl SnippetAccess for Files {
    fn read_snippet(&self, identifier: &str) -> Result<Source, String> {
        let mut filename = PathBuf::from(identifier);
        filename.set_extension("wotwrs");

        let mut path_plando = self.folder.clone();
        path_plando.push(&filename);
        if let Some(result) = try_read(&path_plando) {
            return result;
        }

        let mut path_snippet = PathBuf::from("snippets");
        path_snippet.push(&filename);
        if let Some(result) = try_read(&path_snippet) {
            return result;
        }

        if let Some(result) = try_read(&filename) {
            return result;
        }

        Err(format!(
            "failed to find \"{}\" at \"{}\", \"{}\" or \"{}\"",
            identifier,
            path_plando.display(),
            path_snippet.display(),
            filename.display()
        ))
    }
}

fn try_read(path: &Path) -> Option<Result<Source, String>> {
    match fs::read_to_string(&path) {
        Ok(content) => Some(Ok(Source::new(path.to_string_lossy().to_string(), content))),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => None,
            _ => Some(Err(format!(
                "failed to read \"{}\": {}",
                path.display(),
                err
            ))),
        },
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The plando folder, which should contain "main.wotwrs" as entry point
    folder: PathBuf,
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
