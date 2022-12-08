# parseln! - a println! counterpart

This is just a small helper crate to parse strings using a similar syntax as used in the `println!` macro.

## Installation
Run `cargo add https://github.com/albheim/parseline` in the project you want this in.

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
This is the first crate I have made and it was mostly as a learning experience. I needed some parsing for *Advent of Code* and thought regexp was overkill, and learning how to set up a crate with tests as well as figuring out how to create the needed macro was interesting to try.