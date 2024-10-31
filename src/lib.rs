#![doc(
    html_logo_url = "https://github.com/dr-montasir/mathlab/raw/HEAD/logo.svg?sanitize=true",
    html_root_url = "https://docs.rs/mathlab/latest/mathlab"
)]

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
