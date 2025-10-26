use std::{path::PathBuf, process::Command};

/// The filename of the examples
const EXAMPLES: [&str; 6] = [
    "with_const_variables",
    "with_derive_macro",
    "with_struct",
    "code_modif_noop",
    "code_modif_params",
    "code_modif_replace",
];

/// Examples of the workspace are compiled with release profile.
/// If the examples have been optimized successfully thanks to
/// `const_init` then the string "I should be absent" (used
/// in the removed branch) should not appear in the produced binary.
#[test]
fn branches_are_optimized_away_in_examples() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Path to the generated file
    let generated_file_path = [&manifest_dir, "examples", "utils", "generated_settings.rs"];
    let generated_file_path: PathBuf = generated_file_path.iter().collect();

    // Generate the file containing the constants from "settings.json" as "settings.rs"
    // with "build.rs" and build the examples
    let status = Command::new("cargo")
        .arg("build")
        .arg("--examples") // build all examples
        .arg("--release") // build in release mode to activate the optimizations
        .status()
        .expect("Failed to run cargo build");

    assert!(status.success(), "cargo build failed");
    assert!(generated_file_path.exists());

    for example in EXAMPLES {
        let mut binary_path = vec![&manifest_dir, "target", "release", "examples"];
        let binary: String;
        if cfg!(target_os = "windows") {
            binary = format!("{}.exe", example);
        } else {
            binary = example.to_string();
        }
        binary_path.push(&binary);
        let binary_path: PathBuf = binary_path.iter().collect();

        // Check that the conditional branches using the generated constant values have been been optimized
        // If the branches have been optimized the string "I should be absent" should not appear
        // in the compiled binary because it has been removed.
        // On the other hand the string "I should be present" should have been kept through the
        // compiler optimizations
        let output = Command::new("strings")
            .arg(&binary_path)
            .output()
            .expect(r#"Failed to run "strings" on binary"#);

        assert!(
            output.status.success(),
            "Failed to run 'strings' on binary {:?}",
            binary_path
        );

        let output =
            String::from_utf8(output.stdout).expect("Failed to turn output to utf-8 strings");
        assert!(
            output.contains("I should be present"),
            "Failed for example: {}",
            example
        );
        assert!(
            !output.contains("I should be absent"),
            "Failed for example: {}",
            example
        );
    }
}
