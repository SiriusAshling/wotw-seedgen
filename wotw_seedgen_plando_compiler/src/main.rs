use clap::Parser;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::{
    fmt::{self, Debug},
    fs::{self, File},
    io::{self, ErrorKind, Write},
    mem,
    path::{Path, PathBuf},
};
use wotw_seedgen_assembly::{compile_intermediate_output, Command, Package};
use wotw_seedgen_assets::{SnippetAccess, Source};
use wotw_seedgen_seed_language::{compile::Compiler, output::DebugOutput};
use wotw_seedgen_static_assets::UBER_STATE_DATA;

fn main() -> Result<(), Error> {
    // TODO remove
    // bugsalot::debugger::wait_until_attached(None)?;

    TEMP()?;

    fs::create_dir_all("seeds/out")?;

    let folder = Cli::parse().folder;
    let mut rng = rand::thread_rng();
    let files = Files { folder };
    let mut compiler = Compiler::new(&mut rng, &files, &UBER_STATE_DATA, Default::default());
    compiler.debug();
    compiler.compile_snippet("main")?;
    let mut output = compiler.finish(&mut io::stderr())?;

    // TODO remove
    // fs::write("seeds/out/out.intermediate", format!("{output:#?}"))?;

    let compiler_data = mem::take(&mut output.debug).unwrap();

    let mut package = Package::new("seeds/out/out_release.wotwr")?;
    package.add_from_intermediate_output(output.clone(), false)?;
    package.finish()?;

    let mut package = Package::new("seeds/out/out.wotwr")?;
    let (seed_world, icons) = compile_intermediate_output(output);
    package.add_seed(&seed_world, true)?;
    for (name, icon) in icons {
        let mut path = PathBuf::from("assets");
        path.push(name);
        package.add_data(path, icon)?;
    }

    let metadata = Metadata {
        compiler_data,
        indexed_lookup: seed_world
            .command_lookup
            .iter()
            .cloned()
            .enumerate()
            .collect(),
    };
    package.add_data("debug", serde_json::to_vec_pretty(&metadata)?)?;

    package.finish()?;

    let mut old = File::create("seeds/out/out_old.wotwr")?;
    write!(old, "wotwr,0.0.1,p\n")?;
    let seed = serde_json::to_string(&seed_world)?.replace("File", "Path");
    write!(old, "{seed}")?;

    Ok(())
}

fn TEMP() -> Result<(), Error> {
    // let mut file = File::create("dangerous/reset.wotws")?;

    // writeln!(file, "!callback(reset)")?;
    // writeln!(file, "!on_callback(reset, world_reset())")?;
    // writeln!(file, "")?;
    // writeln!(file, "fun world_reset() {{")?;

    // for (uber_identifier, data) in &UBER_STATE_DATA.id_lookup {
    //     if !(uber_identifier.group == 9 || data.readonly) {
    //         writeln!(file, "    store({uber_identifier}, {})", data.default_value)?;
    //     }
    // }

    // writeln!(file, "}}")?;

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
        filename.set_extension("wotws");

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
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, String> {
        let mut full_path = self.folder.clone();
        full_path.push(path);
        fs::read(full_path).map_err(|err| err.to_string())
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
    /// The plando folder, which should contain "main.wotws" as entry point
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
