use bugsalot::debugger;
use clap::Parser;
use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};
use wotw_seedgen_assets::{SnippetAccess, Source};
use wotw_seedgen_seed::{Compile, Position, SeedWorld, Spawn};
use wotw_seedgen_seed_language::{
    ast::{parse, Snippet},
    compile::Compiler,
};
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

    let mut command_lookup = vec![];
    let events = output
        .events
        .into_iter()
        .map(|event| event.compile(&mut command_lookup))
        .collect::<Vec<_>>();

    let seed = SeedWorld {
        spawn: Spawn {
            position: Position::new(-3537., -5881.),
            identifier: "".to_string(),
        },
        events,
        command_lookup,
    };

    fs::write("out.json", serde_json::to_vec(&seed).unwrap()).unwrap();

    Ok(())
}
