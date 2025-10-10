use std::path::PathBuf;

use const_init_build::generate_multiple_constants_from_json;

fn main() {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // We read the settings from "settings.json" file
    let json_input: PathBuf = [&manifest_path, "settings.json"].iter().collect();
    let outputs: Vec<PathBuf> = vec![
        [&manifest_path, "examples", "generated", "settings.rs"].iter(),
        [&manifest_path, "examples", "code_gen", "settings.rs"].iter(),
        [&manifest_path, "benches", "generated_settings.rs"].iter(),
    ]
    .into_iter()
    .map(|path| path.collect())
    .collect();

    generate_multiple_constants_from_json(&json_input, outputs.iter().collect());
}
