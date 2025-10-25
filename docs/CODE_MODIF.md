# Code Modification

The `#[const_init_code_modif(...)]` macro is used to modify the code to make the most
of constant initialization of certain data types.

We want to have two modes when integrating `const_init`:
- normal mode, the binary behaves normally and `const_init` is disabled
- streamline mode, `const_init` is activated at build time. Target data
are computed at build time and usage of these data are modified to make
the most of the compilation optimizations

## Streamline mode

### Assumption and risks

When in this mode we assume that the
__target `const_init` types will be not modified during the program runtime__.
If the instrumented code does not respect this assumption then the program
will potentially not behave like intended.

Potential behaviors (still in conception) if a `const_init` value is modified:
- changes to the `cont_init` types will be ignored
- the program will crash

### Code modification algorithm

- target data is now considered constant and won't be modified at runtime

Cases to handle:
Input:
- owned value
- shared ref
- mut ref
- Smart pointers like Box
Output:
- owned value
- shared ref to a const
- mut ref to a const
- Smart pointers like Box
