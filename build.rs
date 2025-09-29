use std::path::PathBuf;

use const_init_build::generate_constants_from_json;

fn main() {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // We read the settings from "settings.json" file
    let json_input: PathBuf = [&manifest_path, "settings.json"].iter().collect();

    // We output "settings.rs" containing the variables of "settings.json" as constants
    let rust_output: PathBuf = [&manifest_path, "examples", "generated", "settings.rs"]
        .iter()
        .collect();
    generate_constants_from_json(json_input, rust_output);
}
