# ConstInit

Utilities to help you do constant initializations or build-time initializations of your custom types from a JSON configuration file.
When compiled in release mode, usage of the instances that were constant initialized
will be optimized. Especially branches where condition can be resolved at build time.

Kind of pattern we want to optimize:
```rust
let config = Config::init_at_runtime();
if config.title == "foo" && config.syntax {
    // Compilers can't optimize this branch due to runtime initialization
    ...
}
```

Result:
```rust
const config: Config = Config::const_init();
if config.title == "foo" && config.syntax {
    // This branch will be optimized away by the compiler
    ...
}
```
## Use cases

This is meant for projects with a lot of configuration.
Where code complexity increases a lot due to having a lot of potential settings.

I had this idea while working on [Zed](https://github.com/zed-industries/zed) codebase which contains so many conditional branches depending on your settings.
Ideally when I finish tweaking my settings I'd be able to compile my custom version of Zed that will be optimized.
I'd also like to apply this to [Tauri](https://github.com/tauri-apps/tauri) applications which are also highly configurable.

## Features

- `const_init_build` crate helps you generate a Rust file in `build.rs`. This Rust file contains constants variables obtained from a JSON configuration file.
- `const_init_macro` provides a macro to do constant initializations with your custom struct.

## Workflow

_Cargo.toml_:
```TOML
[dependencies]
const_init_macros = "0.1"

[build-dependencies]
const_init_build = "0.1"
```

_settings.json_:
```json
{
  "foo": true,
  "bar": 1,
}
```
_build.rs_:
```rust
fn main() {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // We read the settings from "settings.json" file
    let json_input: std::path::PathBuf = [&manifest_path, "settings.json"].iter().collect();
    // We output "settings.rs" containing the variables of "settings.json" as constants
    let rust_output: std::path::PathBuf = [&manifest_path, "examples", "generated", "settings.rs"]
        .iter()
        .collect();
    // Generate Rust file from "settings.json"
    const_init_build::generate_constants_from_json(&json_input, &rust_output);
}
```
generated rust file _generated::settings.rs_:
```rust
pub const FOO: bool = true;
pub const BAR: isize = 1;
```
usage in your code:
```rust
mod generated;
use generated::settings::*;
use const_init_macros::ConstInit;

// Macro adds `const_init`, constant function for initialization
#[derive(ConstInit)]
struct FooBar {
    // With attribute, it specifies a constant expr that will be assigned
    #[const_init(value = FOO)]
    foo: bool,
    // Without attribute, looking for matching uppercase field name, here "BAR"
    bar: isize,
}

fn main() {
    // Using the function provided by the derive macro
    const FOO_BAR: FooBar = FooBar::const_init();
    if FOO_BAR.foo
        && FOO_BAR.bar == 1
    {
        // Should be kept during compiler optimizations
        println!("{}", "I should be present in the binary");
    } else {
        // Should be removed by compiler optimizations
        println!("{}", "I should be absent in the binary");
    }
}
```


## Limitations

### File format

Currently only JSON is supported but there are no difficulties to
support other formats such as TOML.

### Json to Rust

Certain JSON types do not translate perfectly into Rust types.
- JSON `integers` are all turned into Rust `isize`
- JSON `arrays` containing different types are not handled
- JSON `null` is unsupported
- JSON `Nan` is unsupported

## License

This project is licensed under the [MIT License](LICENSE.txt).
