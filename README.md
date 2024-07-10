[![Documentation](https://img.shields.io/badge/Documentation-version%200.1.4-blue.svg)](https://crates.io/crates/mathlab/0.1.4) [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://choosealicense.com/licenses/apache-2.0/)

# MathLab

> A Powerful Math Library for Rust

## Install

Run the following Cargo command in your project directory:

```shell
cargo add mathlab
```

Or add the following line to your Cargo.toml:

```toml
mathlab = "0.1.4"
```

## Usage

```rust
// example

use mathlab::math;

fn main() {
    let abs = math::abs(-2);
    let add = math::add(0.1, 0.2);
    let add_f64 = math::add(0.1, 0.2) as f64;
    let add_f32 = math::add(0.1, 0.2) as f32;

    println!("{}", abs); // 2
    println!("{}", add); // 0.30000000000000004
    println!("{}", add_f64); // 0.30000000000000004
    println!("{}", add_f32); // 0.3
}
```

or

```rust
// example

use mathlab::math::{abs, add};

fn main() {
    let my_abs = abs(-2);
    let my_add = add(0.1, 0.2);
    let my_add_f64 = add(0.1, 0.2) as f64;
    let my_add_f32 = add(0.1, 0.2) as f32;

    println!("{}", my_abs); // 2
    println!("{}", my_add); // 0.30000000000000004
    println!("{}", my_add_f64); // 0.30000000000000004
    println!("{}", my_add_f32); // 0.3
}
```

## Documentation

[![Documentation](https://img.shields.io/badge/Documentation-version%200.1.4-blue.svg)](https://crates.io/crates/mathlab/0.1.4)

https://crates.io/crates/mathlab/0.1.4
