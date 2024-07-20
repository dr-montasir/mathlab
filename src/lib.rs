// Constants
pub mod con;

// Functions
pub mod fun;

#[allow(dead_code)]
pub mod math {
    // Constants
    pub use crate::con::*;

    // Functions
    pub use crate::fun::*;
}
