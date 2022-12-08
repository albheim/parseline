# parseln! - a println! counterpart

[![Test](https://github.com/albheim/parseline/actions/workflows/test.yaml/badge.svg)](https://github.com/albheim/parseline/actions/workflows/test.yaml) [![Check and Lint](https://github.com/albheim/parseline/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/albheim/parseline/actions/workflows/check-and-lint.yaml) [![Codecov](https://codecov.io/gh/albheim/parseline/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/albheim/parseline)

A small helper crate to parse strings using a similar syntax as used in the `println!` macro.

## Installation
To use this library in a project simply add
```toml
[dependencies]
parseline = { git = "https://github.com/albheim/parseline" }
```
to your project `Cargo.toml` and then import parseline as
```
use parseline::parseln;
```

## Usage
It can be used either with already defined variables as 
```rust
let month: String;
let day: isize;
parseln!("Date: apr 13", "Date: {} {}", month, day);
```
or by generating new binding, though then we need to supply the type to be parsed
```rust
parseln!("Date: apr 13", "Date: {} {}", month: String, day: i32);
```

## TODO
* Currently it is not possible to mix these methods.
* Allow for writing to references, automatically dereference?
* cargo doc?
* Don't do split->collect->iter, check [macro book](https://veykril.github.io/tlborm/decl-macros/macros-methodical.html) for tips on how to do indexing in repeating macros.

## Disclaimer
This is the first crate I have made and it was mostly as a learning experience. I needed some parsing for *Advent of Code* and thought regular expressions were overkill, and learning how to set up a crate with tests as well as figuring out how to create the needed macro was interesting to try.
