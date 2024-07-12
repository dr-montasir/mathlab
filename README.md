[<img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20mathlab-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/dr-montasir/mathlab)[<img alt="crates.io" src="https://img.shields.io/crates/v/mathlab.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/mathlab)[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mathlab-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/mathlab)[<img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">](https://choosealicense.com/licenses/apache-2.0)

# MathLab

> A Powerful Math Library for Rust

## Install

Run the following Cargo command in your project directory:

```shell
cargo add mathlab
```

Or

 Add the following line to your `Cargo.toml` file with the specified version, then run `cargo build`:

```toml
mathlab = "MAJOR.MINOR.PATCH"
```

## Changelog

[![github](https://img.shields.io/badge/github-%20changelog-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab/blob/master/CHANGELOG.md)

https://github.com/dr-montasir/mathlab/blob/master/CHANGELOG.md

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

Or

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

> #### **[https://docs.rs/mathlab](https://docs.rs/mathlab)**
>
> **MathLab :**
>
> A Powerful Math Library for Rust

[![crates.io](https://img.shields.io/crates/v/mathlab.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/mathlab)

[![docs.rs](https://img.shields.io/badge/docs.rs-mathlab-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/mathlab)

[![license](https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache)](https://choosealicense.com/licenses/apache-2.0)

[![github](https://img.shields.io/badge/github-dr%20montasir%20/%20mathlab-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/mathlab)
