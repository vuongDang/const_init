
 This crate contains two macros:
 - derive macro [`ConstInit`]
 - attribute macro [`const_init_code_modif`]

 [`ConstInit`] is used to create a constant function and constant variable
 for the targeted type.

 [`const_init_code_modif`] is used to maximize the compiler optimizations
 by modifying the user code by replacing usage of the targeted types
 with their constant values. This macro change the code behavior and
 should only be used within the assumption that targeted types
 won't be modified at runtime.

 This crate is meant to be used in conjunction with
 [`const_init_build`](https://docs.rs/const_init_build/latest/const_init_build/index.html)

More details on the macros are available in  [`ConstInit`]
 and [`const_init_code_modif`] sections.
