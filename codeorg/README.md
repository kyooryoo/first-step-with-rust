# Code Organization
Three concepts for organizing Rust code:
* Package: hold cretes and project info.
* Crate: compilation unit to exe or lib.
* Module: nested units within crate.

## Package
What `cargo new my-pj` creates is a package.
By default, there is only one binary crate.
You can put more *rs* file in *src* folder.
Each *rs* file will create a binary crate.

## Crate
By default, `cargo new` creates one crate.
Each third-party library is also one crate.

Use `cargo new -lib my-lib` to create lib.
The file is *lib.rs* instead of *main.rs*.

## Module
Module is a collection of following items:
const, type, fn, struct, enum, trait, impl...

Module item could be *public* or *private*.
By default, all module items are private.
For the current module and its descendants
*pub* makes them accessible from outside.
Good for hiding internal implementation details.

Module is declared with *mod* keyword.
Module is inserted before compile when *use*.
Module code will replace `use <module>` code.
Module code is compiled as a crate when used.

When module contents becomes large, move it.
After moving, use keyword *mod* to import it.

## Function
Two ways for using functions defined in a module:
* For Directly defined *pub* function: 
Use `<modulename>::<functionname>();`
See the *greeting* function in *auth_mod*
* For function from *impl* of a struct:
Use `<structobject>.<functionname>();`
See other User function in *auth_mod*

## Visilibity
Module resides in another module could be used:
`<parentmodule>::<childmodule>::<function>();`
The child module and function should be *pub*

## Third-Party
Modify *Cargo.toml* to add dependent from [Crates.io](https://crates.io/).
It's Rust's community central registry for libs.
Add `<libname>="var.num"` in *dependencies* section.
Then run `cargo build` command to apply the change.
