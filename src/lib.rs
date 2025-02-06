#![doc(html_logo_url = "https://github.com/dr-montasir/mathlab/raw/HEAD/logo.svg")]
#![doc = r"<div align='center'><a href='https://github.com/dr-montasir/mathlab' target='_blank'><img src='https://github.com/dr-montasir/mathlab/raw/HEAD/logo.svg' alt='Mathlab' width='80' height='auto' /></a><br><br><a href='https://github.com/dr-montasir/mathlab' target='_blank'>MATHLAB</a><br><br>A Powerful Math Library for Rust</div>"]

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
