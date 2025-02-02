# Packages, Crates, and Modules

A *module system* include:

1. **Packages** : A cargo feature that lets you build, test, and share crates.
2. **Crates**: A tree of modules that produces a library or executable.
3. **Modules** and **use**: Let you control the organization, scope, and privacy of paths.
4. **Paths**: A way of naming an item, such as a struct, function, or module.

## Crates
- A *crate* is the smallest amount of code that the Rust compiler considers at a time.
- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
- A crate can come in one of two forms:
  1. A binary crate.
  2. A library crate.
- *Binary crates* are programs you can compile to an executable that you can run, Each **MUST** have a function called `main` that defines what happens when the executables runs.
- *Library crates* do **NOT** have a `main` function, and they do **NOT** compile to an executable. Instead, they define functionality intended to be shared with multiple projects.
- Most of the time when Rustaceans say *crate*, they mean library crate, and they use *crate* interchangeably with the general programming concept of a *library*.
- The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.

## Packages

- A package is a bundle of one or more crates that provides a set of functionality.
- A package contains a `Cargo.toml` file that describes how to build those crates.
- A package can contain as many binary crates as you like, but at most only one library crate.
- A package **MUST** contain at least one crate, whether that's a library or binary crate.
- Cargo is actually a package that contains the binary crate or library crate that the binary crate depends on.
- Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.
- Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, as `src/lib.rs` is its crate root.
- Cargo passes the crate root files to `rustc` to build the library or binary.
- If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary and a library, both with the same name as the package.
- A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

## How Modules Work?

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file for code to compile. (`src/lib.rs` or `src/main.rs`)
- **Declaring modules**: In the crate root file, you can declare new modules; say you declare a *garden* module with `mod gargen;`. The compiler will look for the module's code in these places:
  1. Inline, within curly brackets that replace the semicolon following `mod gargen`
  2. In the file `src/gargen.rs`.
  3. In the file `src/garden/mod.rs`
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in `src/gargen.rs`. The compiler will look for the submodule's code within the directory named for the parent module in these places:
  1. Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  2. In the file `src/gargen/vegetables.rs`
  3. In the file `src/gargen/vegetables/mod.rs`
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the gargen vegetables modules would be found at `crate::gargen::vegetables::Asparagus`.
- **Private vs Public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

