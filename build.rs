use json::JsonValue;

fn main() {
    let settings_file = "settings.json";
    let contents = std::fs::read_to_string(settings_file).expect("Failed to read settings.json");
    let settings = json::parse(&contents).expect("Failed to deserialize settings");
    println!("{:?}", settings);
    let mut generated_settings = String::new();
    generated_settings.push_str("// Generated file, don't modify it\n");
    generated_settings.push_str(
        r#"// This file is built at compile-time and contains the variable from "settings.json""#,
    );
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
    std::fs::write("src/generated_settings.rs", generated_settings)
        .expect("Failed to generated Rust file for settings");
}
