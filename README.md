# ConstInit

The goal is to be to read some environment variables and store them in variables that will be seen constants for the compiler.
The use case is for highly configurable projects where code complexity increases a lot due to having a lot of potential settings.
The perfect example is for an IDE. I had this idea while working on Zed codebase which contains so many conditional branches depending on your settings.
Ideally when I finish tweaking my settings I'd be able to compile my own version of Zed that
is custom and optimized.

## Steps

- [x] Set global variables values from configuration file
- [x] Check that conditional branches using these global variables are optimized away
  - [x] write a test for it
- [x] Extend the use case by setting these variables as fields of a struct (like it would be used in Zed)
- [x] Write some macros that could be used to use such feature on struct without rewriting a whole codebase
- [ ] Not necessary anymore, struct can be initialized as a const and compiler will optimize anytime it can.

  ~Enable two possible modes with macros:~.
  - ~normal: targeted variables are mutable and can be edited at runtime~
  - ~constant: targeted variables are fetched at build time and are constants / optimized by the compiler for branches~
- [x] Build a function that can used in the build.rs to easily generate constant Rust values from a json file
- [ ] Test performance gain

## Limitations

Certain JSON types do not translate perfectly into Rust types.
- JSON `integers` are turned into Rust `isize`
- JSON `null` is unsupported
- JSON `Nan` is unsupported

Currently all JSON `integers` are turned into Rust `isize` (hence we exclude `floats` and we don't differentiate between
the different Rust integer types).

## TODO

- Better macro and examples to handle arrays and "hashmap"
