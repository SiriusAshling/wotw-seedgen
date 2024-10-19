# Ori and the Will of the Wisps Seed Generator

Seed generator for the [Ori and the Will of the Wisps randomizer](https://wotw.orirando.com/)

## Development

### Workspace structure

- `assets`: not a crate, contains assets which are shipped to users with the randomizer
- `wotw_seedgen`: high-level interface to generate seeds
- `wotw_seedgen_assets`: data structures and parsing for simple assets
- `wotw_seedgen_cli`: command line interface exposing features of the other workspace members
- `wotw_seedgen_data`: constants and data structures describing the game and randomizer
- `wotw_seedgen_derive`: derive macros used throughout the workspace
- `wotw_seedgen_logic_language`: compiler for areas.wotw
- `wotw_seedgen_parse`: homebrew parsing library
- `wotw_seedgen_seed`: generates the seed file format after compilation and/or seed generation
- `wotw_seedgen_seed_language`: compiler for snippets and plandos
- `wotw_seedgen_settings`: data structures representing settings for generating seeds
- `wotw_seedgen_static_assets`: precompiled versions of assets to embed in binaries or tests
- `wotw_seedgen_stats`: analyzes huge amounts of seeds

### Automated testing

After making changes, consider running the automatic tests:

```
cargo test
```

### Manual testing

The `wotw_seedgen_cli` workspace member is the only binary in the workspace, so it will be selected when using `cargo run`. You can use it to generate seeds with any settings you want to test:

```
cd assets
cargo run seed --help
```

### Compiling

You can compile your own version of `seedgen.exe` (which comes from `wotw_seedgen_cli`):

```
cargo build --release
```

It will be located at `target/release/seedgen.exe`.

Note that even if you replace `seedgen.exe` in your randomizer installation, it will not be picked up by the launcher because it uses an online seedgen, nor by the logic filter which uses `seedgen_interop.dll`. It's only useful for the cli it provides.
