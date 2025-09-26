use settings_as_constants::*;
use std::{path::PathBuf, process::Command};

#[test]
fn generated_constant_values_are_correct() {
    assert!(FOO);
    assert_eq!(BAR, 1);
}

#[test]
fn branches_are_optimized_away() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let generated_file_path = [&manifest_dir, "src", "generated_settings.rs"];
    let generated_file_path: PathBuf = generated_file_path.iter().collect();

    // Regenerate the file containing the constants from "settings.json" and
    // Build the binary that uses these constants
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release") // build in release mode to activate the optimizations
        .status()
        .expect("Failed to run cargo build");

    assert!(status.success(), "cargo build failed");
    assert!(generated_file_path.exists());

    let mut binary_path = vec![&manifest_dir, "target", "release"];
    if cfg!(target_os = "windows") {
        binary_path.push("settings_as_constants.exe")
    } else {
        binary_path.push("settings_as_constants")
    }
    let binary_path: PathBuf = binary_path.iter().collect();

    // Check that the conditional branches using the constant generated are not present
    // The conditional branches containing strings containing the word "present" for
    // the branch that is kept and "absent" for the one which should have been
    // removed by the compiler
    let output = Command::new("strings")
        .arg(binary_path)
        .output()
        .expect(r#"Failed to run "strings" on binary"#);

    let output = String::from_utf8(output.stdout).expect("Failed to turn output to utf-8 strings");
    assert!(output.contains("present"));
    assert!(!output.contains("absent"));
}
