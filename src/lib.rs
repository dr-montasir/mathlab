#![doc(html_logo_url = "https://github.com/dr-montasir/mathlab/raw/HEAD/logo.svg")]
#![doc = r##"
<div align="center">
  <a href="https://github.com/dr-montasir/mathlab">
      <img src="https://github.com/dr-montasir/mathlab/raw/HEAD/logo.svg" width="100">
  </a>
  <h2><a href="https://github.com/dr-montasir/mathlab">MATHLAB</a></h2>
  <h3><b>A Powerful Math Library for Rust</b></h3>
  <a href="https://github.com/dr-montasir/mathlab" target="_blank"><img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20mathlab-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22"></a>
  <a href="https://crates.io/crates/mathlab" target="_blank"><img alt="crates.io" src="https://img.shields.io/crates/v/mathlab.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22"></a>
  <a href="https://docs.rs/mathlab" target="_blank"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mathlab-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22"></a>
  <a href="https://choosealicense.com/licenses/apache-2.0" target="_blank"><img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22"></a>
  <a href="https://choosealicense.com/licenses/mit" target="_blank"><img alt="license" src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22"></a>
  <a href="https://crates.io/crates/mathlab" target="_blank"><img alt="downloads" src="https://img.shields.io/crates/d/mathlab.svg?style=for-the-badge&labelColor=555555&logo=&color=428600" height="22"></a>
  <p>
    The <b>mathlab</b> crate is a Rust library designed to facilitate matrix and mathematical operations similar to those found in MATLAB. This crate provides high-level abstractions that make it user-friendly for mathematical computations and linear algebra tasks.
  </p>
</div>
"##]

// Constants
pub mod constants;

// Functions
pub mod functions;

#[allow(dead_code)]
pub mod math {
    // Constants
    pub use crate::constants::*;

    // Functions
    pub use crate::functions::*;
}
