use std::path::Path;

use json::JsonValue;

/// This function is used in build scripts to generate a rust file from a json file.
/// The output file contains all the variables from input file as rust constants.
///
/// # Panics
///
/// Panics if does not find the input json file, can't parse it, or can't write the output rust file.
pub fn generate_constants_from_json<P: AsRef<Path>>(input_json_file: P, output_rust_file: P) {
    let contents = std::fs::read_to_string(&input_json_file).expect("Failed to read input file");
    let settings = json::parse(&contents).expect("Failed to deserialize input json file");

    // Produce the content of the output rust file containing constants
    let mut generated_settings = String::new();
    generated_settings.push_str("// Generated file, don't modify it\n");
    generated_settings.push_str(&format!(
        r#"// This file is built at compile-time and contains the variable from "{}""#,
        input_json_file.as_ref().to_string_lossy()
    ));
    generated_settings.push_str("\n\n");
    for (name, value) in settings.entries() {
        let name = name.to_uppercase();
        let line = match value {
            JsonValue::Boolean(v) => format!("pub const {}: bool = {};\n", name, v),
            JsonValue::Number(v) => format!("pub const {}: usize = {};\n", name, v),
            // TODO: handle other types and recursion
            _ => todo!(),
        };
        generated_settings.push_str(&line);
    }

    // Generate the output file
    std::fs::write(output_rust_file, generated_settings)
        .expect("Failed to generated Rust file for settings");
}
