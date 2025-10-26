use std::{path::PathBuf, process::Command};

/// The filename of the examples
const EXAMPLES: [&str; 3] = ["code_modif_noop", "code_modif_params", "code_modif_replace"];

/// Examples of the workspace are compiled with release profile.
/// If the examples have been optimized successfully thanks to
/// `const_init` then the string "I should be absent" (used
/// in the removed branch) should not appear in the produced binary.
#[test]
fn examples_are_optimized_with_const_init() {
    let target_dir: PathBuf = ["target", "tests", "with_const_init"].iter().collect();
    build_code(true, &target_dir);
    check_branch_optimizations(true, &target_dir);
}

#[test]
fn examples_are_not_optimized_without_const_init() {
    let target_dir: PathBuf = ["target", "tests", "without_const_init"].iter().collect();
    build_code(false, &target_dir);
    check_branch_optimizations(false, &target_dir);
}

fn build_code(with_const_init: bool, target_dir: &PathBuf) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Path to the generated file
    let generated_file_path = [&manifest_dir, "examples", "utils", "generated_settings.rs"];
    let generated_file_path: PathBuf = generated_file_path.iter().collect();

    let target_dir = PathBuf::from(manifest_dir).join(target_dir);

    // Generate the file containing the constants from "settings.json" as "settings.rs"
    // with "build.rs" and build the examples

    let status = if with_const_init {
        Command::new("cargo")
            .arg("build")
            .arg("--examples") // build all examples
            .arg("--release") // build in release mode to activate the optimizations
            .arg("--target-dir")
            .arg(&target_dir)
            .arg("--features")
            .arg("const-init") // build with const-init
            .status()
            .expect("Failed to run cargo build")
    } else {
        Command::new("cargo")
            .arg("build")
            .arg("--examples") // build all examples
            .arg("--release") // build in release mode to activate the optimizations
            .arg("--target-dir")
            .arg(&target_dir)
            .status()
            .expect("Failed to run cargo build")
    };

    assert!(status.success(), "cargo build failed");
    assert!(generated_file_path.exists());
}

fn check_branch_optimizations(should_be_optimized: bool, target_dir: &PathBuf) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    for example in EXAMPLES {
        let mut binary_path = vec!["release", "examples"];
        let binary: String;
        if cfg!(target_os = "windows") {
            binary = format!("{}.exe", example);
        } else {
            binary = example.to_string();
        }
        binary_path.push(&binary);
        let binary_path: PathBuf = binary_path.iter().collect();
        let target_dir = PathBuf::from(&manifest_dir).join(target_dir);
        let binary_path = target_dir.join(binary_path);

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
            &binary_path
        );

        let output =
            String::from_utf8(output.stdout).expect("Failed to turn output to utf-8 strings");
        assert!(
            output.contains("I should be present"),
            "Failed for example: {}",
            example
        );
        if should_be_optimized {
            assert!(
                !output.contains("I should be absent"),
                "Failed for example: {}",
                example
            );
        } else {
            assert!(
                output.contains("should be absent"),
                "Failed for example: {}",
                example
            );
        }
    }
}
