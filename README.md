# parseln! - a println! counterpart

![example workflow](https://github.com/albheim/parseline/actions/workflows/check-and-lint.yaml/badge.svg) ![example workflow](https://github.com/albheim/parseline/actions/workflows/test.yaml/badge.svg) [![codecov](https://codecov.io/gh/albheim/parseline/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/albheim/parseline)

A small helper crate to parse strings using a similar syntax as used in the `println!` macro.

## Installation
To use this library in a project simply add
```toml
[dependencies]
parseline = { git = "https://github.com/albheim/parseline" }
```
to your project `Cargo.toml` and then import parseline as
```
use parseline::parseln
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

## Disclaimer
This is the first crate I have made and it was mostly as a learning experience. I needed some parsing for *Advent of Code* and thought regular expressions were overkill, and learning how to set up a crate with tests as well as figuring out how to create the needed macro was interesting to try.