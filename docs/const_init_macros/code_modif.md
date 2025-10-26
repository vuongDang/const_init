# Code Modification with `#[const_init_code_modif(...)]`

The `#[const_init_code_modif(...)]` macro is used to modify the code to make the most of constant initialization of certain data types.

This macro is enabled when building your binary with the feature `const-init`.

Target `const_init` types are computed at build time and usage of these data in the code is modified to make the most of the compilers optimizations.

__NOTE__: this changes the behaviors of your code and should be used
in a scenario where your target const_init types won't be modified at runtime

### Assumption
When in this mode we assume that:

__target `const_init` types will be not modified at runtime__.


If the instrumented code does not respect this assumption then the program
will potentially not behave like intended.

### Risks (TODO)

Potential behaviors if the assumption is not respected:
- changes to the `cont_init` types will be ignored
- the program will crash

### Activation of the `const_init` mode

Use the feature `const-init` when building your binary.
```ignore
cargo build --features const-init
```

## Code modification macros

There are 3 available macros:
- `#[const_init_code_modif(noop)]`
- `#[const_init_code_modif(replace_with_const_init)]`
- `#[const_init_code_modif(target_params(...))]`

### `#[const_init_code_modif(noop)]`

The macro  `#[const_init_code_modif(noop)]`
is meant to be used with functions where target const_init
parameters are mutable owned value or mutable ref.
When using this crate, the assumption is that the target
`const_init` types will be immutable during the whole
execution. Hence any functions mutating these types
should be unnecessary.
To enhance performance this macro transforms
a function performing mutations into a noop functions.

#### Notes

- this can only be applied to functions returning the unit type
- this macro will remove any side-effect that your function was
using (`println!`, IO, mutation of other types...)

#### Example

```rust,ignore
#[const_init_code_modif(noop)]
fn mutate_foo(foo: &mut Foo) {
    ...
}
```

### `#[const_init_code_modif(replace_with_const_init)]`

The macro  `#[const_init_code_modif(replace)]`
is meant to be used with functions which return
an owned value of a target `const_init` type.

When using this crate, the assumption is that the target
const_init type will be immutable during the whole
execution.

To enhance performance this macro  replace
a function constructing a `const_init` type
to simply returning a constant value of that type


#### Notes

- this can only be applied to functions returning an owned type
- this macro will remove any side-effect that your function was
using (`println!`, IO, mutation of other types...)

#### Example

```rust,ignore
impl Foo {
    #[const_init_code_modif(replace_with_const_init)]
    fn new() -> Self {
        ...
    }
}
```

### `#[const_init_code_modif(target_params())]`

The macro  `#[const_init_code_modif(target_params(...))]`
is meant to be used with functions where target `const_init`
types are used as immutable parameters.
Mostly immutable owned value or shared ref.

The macro will add at the beginning of the function the
statement `[param_ident] = [param_type]::CONST_INIT_VAR;`
to set them with a constant value and enable compiler
optimizations in the rest of the functions.


#### Example

```rust,ignore
impl Foo {
    #[const_init_code_modif(target_params(self, bar))]
    fn heavy_computation(&self, bar: Bar)  {
        ...
    }
}
```
