//! Generate Rust constant variables from the data in your configuration file
//! (currently only JSON) at build time.
//!
//! The goal is to improve performance by initializing your types with build-time
//! values and benefit from compiler optimizations.
//! This is meant to be used with
//! [`const_init_macros`](https://docs.rs/const_init_macros/latest/const_init_macros/index.html)
//!  that helps do constant build-time initialization with your custom types.
//!
//! # Workflow
//!
//! Starts from  JSON configuration file, for example _settings.json:_
//! ```json,file:settings.json
//! {
//!   "foo": true,
//!   "bar": 1,
//! }
//! ```
//!
//! Generate Rust constants at build-time in  _build.rs:_
//! ```rust,no_run,file:build.rs
//! use std::path::PathBuf;
//! use const_init_build::generate_constants_from_json;
//! fn main() {
//!     let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
//!     // We read the settings from "settings.json" file
//!     let json_input: PathBuf = [&manifest_path, "settings.json"].iter().collect();
//!     // We output "settings.rs" containing the variables of "settings.json" as constants
//!     let rust_output: PathBuf = [&manifest_path, "examples", "generated", "settings.rs"]
//!         .iter()
//!         .collect();
//!
//!     generate_constants_from_json(&json_input, &rust_output);
//! }
//! ```
//!
//! Obtain Rust constant variables at _examples/generated/settings.rs:_
//! ```rust,file:examples/generated/settings.rs
//! pub const FOO: bool = true;
//! pub const BAR: isize = 1;
//! ```
//!
//!# Limitations
//!
//!## File format
//!
//!Currently only JSON is supported but there are no difficulties to
//!support other formats such as TOML.
//!
//!## Json to Rust
//!
//!Certain JSON types do not translate perfectly into Rust types.
//!- JSON `integers` which are not float are all turned into Rust `isize`
//!- JSON `arrays` containing different types are not handled
//!- JSON `null` is unsupported
//!- JSON `Nan` is unsupported

mod json;
pub use json::*;
