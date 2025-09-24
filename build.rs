use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    foo: bool,
    bar: u8,
}

fn main() {
    let settings_file = "settings.json";
    if let Ok(contents) = fs::read_to_string(settings_file) {
        let settings: Settings =
            serde_json::from_str(&contents).expect("Failed to deserialize settings");
        let foo = settings.foo;
        let bar = settings.bar;
        println!("cargo::rustc-env=TEMP_FOO={foo}");
        println!("cargo::rustc-env=TEMP_BAR={bar}");
    }
}
