# Settings as constants

The goal is to be to read some environment variables and store them in variables that will be seen constants for the compiler.
The use case is for highly configurable projects where code complexity increases a lot due to having a lot of potential settings.
The perfect example is for an IDE. I had this idea while working on Zed codebase which contains so many conditional branches depending on your settings.
Ideally when I finish tweaking my settings I'd be able to compile my own version of Zed that
is custom and optimized.

## Steps

- [ ] Set global variables values from configuration file
- [ ] Check that conditional branches using these global variables are optimized away
- [ ] Extend the use case by setting these variables as fields of a struct (it's us)
- [ ] Write some macros that could be used to use such feature on struct without rewriting a whole codebase
- [ ] Enable two possible modes with macros:
  - normal: targeted variables are mutable and can be edited at runtime
  - constant: targeted variables are fetched at build time and are constants / optimized by the compiler for branches
