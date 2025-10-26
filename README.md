# ConstInit

`ConstInit` allows you to do build-time initializations of your custom types from a JSON configuration file.
When compiling your binary with the `const-init` feature the targeted types
will have their values initialized at build time.
This enables more compiler optimizations and especially branch removal.

Kind of pattern we want to optimize:

```rust
let config = Config::init_at_runtime();
if config.title == "foo" && config.syntax {
    // Compilers can't optimize this branch since the conditions values
    // can only be computed at runtime
    ...
}
```

Result:

```rust
const config: Config = Config::const_init();
if config.title == "foo" && config.syntax {
    // This branch will be optimized away by the compiler
    // since we know at build time if the condition is true or false
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

- [`const_init_build`](https://crates.io/crates/const_init_build) crate helps you generate a Rust file in `build.rs`. This Rust file contains constants variables obtained from a JSON configuration file.
- [`const_init_macros`](https://crates.io/crates/const_init_macros) provides macros to do constant initializations with your custom struct.
  - [`ConstInit`](https://docs.rs/const_init_macros/latest/const_init_macros/derive.ConstInit.html)
  derives constant functions to initialize your struct.
  - [`const_init_code_modif(...)`](https://docs.rs/const_init_macros/latest/const_init_macros/attr.const_init_code_modif.html)
  gives attributes macro to modify your code to make the most of compiler optimizations


## Benchmarks

A detail result of the benchmarks can be found at ![docs/BENCHs.md](docs/BENCHs.md).

tl;dr:
- Gains due to compiler constant propagation optimizations exist but are not overwhelming
- CPU behavior depending on where/how store your data can overwrite impacts of such compiler optimizations
- There is a net gain when initializing your data from json at build-time compared to runtime

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

## Workflow

_Cargo.toml_:

```TOML
[dependencies]
const_init_macros = "0.1"

[build-dependencies]
const_init_build = "0.1"

[features]
# Use this feature when you want to build your binary in "const_init" mode
const-init = []
```

_settings.json_:

```json
{
  "foo": true,
  "bar": 1
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
Build-time generated rust file _generated::settings.rs_:
```rust
pub const FOO: bool = true;
pub const BAR: isize = 1;
```

## Usage in Rust code

Build your code in `const_init` mode with
```
cargo build --features const-init
```

Code:
```rust
mod utils;
use const_init_macros::{ConstInit, const_init_code_modif};

// Content of the generated file "utils::generated_settings.rs"
// produced by `const_init_build`:
// pub const FOO: bool = true;
// pub const BAR: isize = 1;
// pub const B: [isize; 3] = [1,2,-3];
// pub const C: f64 = 3.14;
// pub const D: &str = "ding!";

#[derive(ConstInit)]
// Use this attribute if you want to import the global variables only
//  for the init function and not pollute the module namespace
// Otherwise you need to import the constants at the module level,
// here it would be `use utils::generated_settings::*;`
#[const_init(import_path = utils::generated_settings)]
struct FooBar {
    // Without attribute, looking for matching uppercase field name, here "FOO"
    foo: bool,
    bar: isize,
    // With attribute, it specifies a constant expr that will be assigned
    #[const_init(value = B)]
    b: [isize; 3],
    #[const_init(value = 3.14)]
    c: f64,
    #[const_init(value = D)]
    d: &'static str,
}

/* The code produced by the macro looks like this:
   impl FooBar {
       pub const fn const_init() -> Self {
           use generated::settings::*;
           FooBar {
               foo: FOO,
               bar: BAR,
               b: a::B,
               c: 3.14,
               d: a::D
           }
       }
   }
*/

// We show macros that can be used to modify your code easily
impl FooBar {
    #[const_init_code_modif(replace_with_const_init)]
    fn new(foo: bool, bar: isize, b: [isize; 3], c: f64, d: &'static str) -> Self {
        // With the macro the function code will be replaced with
        // "FooBar::CONST_INIT_VAR"
        FooBar { foo, bar, b, c, d }
    }

    #[const_init_code_modif(noop)]
    fn set_foo(&mut self, foo: bool) {
        // With the macro the function code will be replaced with noop
        self.foo = foo;
    }

    #[const_init_code_modif(target_params(self))]
    fn read_foo_bar(&self) {
        // With the macro this code will be added at the beginning
        // "self = FooBar:CONST_INIT_VAR;"
        println!("{}{}", self.foo, self.bar);
    }
}

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    // Constant initialization
    const FOO_BAR: FooBar = FooBar::const_init();
    if FOO_BAR.foo
        && FOO_BAR.bar == 1
        && FOO_BAR.b == [1, 2, -3]
        && FOO_BAR.c == 3.14
        && FOO_BAR.d == "ding!"
    {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
```

## License

This project is licensed under the [MIT License](LICENSE.txt).
