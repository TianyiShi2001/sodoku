# sodoku

[![crates.io](https://img.shields.io/crates/d/sodoku.svg)](https://crates.io/crates/sodoku)
[![crates.io](https://img.shields.io/crates/v/sodoku.svg)](https://crates.io/crates/sodoku)
[![crates.io](https://img.shields.io/crates/l/sodoku.svg)](https://crates.io/crates/sodoku)
[![docs.rs](https://docs.rs/sodoku/badge.svg)](https://docs.rs/sodoku)

A crate for generating and solving [sodokus](https://en.wikipedia.org/wiki/Sudoku).

# Usage 

For now, you can try:

```
// main.rs
use sodoku::{SodokuMatrix, Sodoku};
use sodoku::examples::SODOKU;


pub fn main() {
    for sodoku in SODOKU.iter() {
        let mut sodoku = Sodoku::from_matrix(sodoku.clone());
        sodoku.solve();
        println!("{}", sodoku);
    }
}

```

# Roadmap

- [X] sodoku solver
- [ ] Sodoku generator
- [ ] `16*16` hexadecimal sodoku
