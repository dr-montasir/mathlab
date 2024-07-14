// Constants
pub mod con;

// Functions
pub mod fun;

#[allow(dead_code)]
pub mod math {
    // Constants
    pub use crate::con::{E, LN10, LN2, LOG10E, LOG2E, PHI, PI, TAU};

    // Functions
    pub use crate::fun::{abs, add, divi, mult, pow, sign, subt};
}
