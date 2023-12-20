use clap::Parser;
use itertools::Itertools;
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
};
use wotw_seedgen_assets::{SnippetAccess, Source};
use wotw_seedgen_seed::{Compile, SeedWorld, Spawn};
use wotw_seedgen_seed_language::compile::Compiler;
use wotw_seedgen_static_assets::UBER_STATE_DATA;

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
    /// The plando folder, which should contains "main.wotwrs" as entry point
    folder: PathBuf,
}

fn main() -> Result<(), String> {
    let folder = Cli::parse().folder;
    let mut rng = rand::thread_rng();
    let files = Files { folder };
    let mut compiler: Compiler<'_, '_, Files> =
        Compiler::new(&mut rng, &files, &UBER_STATE_DATA, Default::default());
    compiler.compile_snippet("main")?;
    let output = compiler
        .finish(&mut io::stderr())
        .map_err(|err| err.to_string())?;

    let flags = output.flags.into_iter().join(", ");
    let spawn = output
        .spawn
        .map(|position| Spawn {
            position,
            identifier: "Custom Spawn".to_string(),
        })
        .unwrap_or_default();

    let mut command_lookup = vec![];
    command_lookup.resize_with(output.command_lookup.len(), Default::default);
    for (index, command) in output.command_lookup.into_iter().enumerate() {
        command_lookup[index] = command.compile(&mut command_lookup);
    }
    let events = output
        .events
        .into_iter()
        .map(|event| event.compile(&mut command_lookup))
        .collect::<Vec<_>>();

    let seed = SeedWorld {
        flags,
        spawn,
        events,
        command_lookup,
        metadata: (),
    };

    fs::create_dir_all("seeds").map_err(|err| err.to_string())?;
    let mut file = File::create("seeds/out.wotwr").map_err(|err| err.to_string())?;
    file.write_all(b"wotwr,0.0.1,\n")
        .map_err(|err| err.to_string())?;
    serde_json::to_writer(file, &seed).map_err(|err| err.to_string())?;

    Ok(())
}
