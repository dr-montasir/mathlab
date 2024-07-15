// Constants
pub mod con;

// Functions
pub mod fun;

#[allow(dead_code)]
pub mod math {
    // Constants
    pub use crate::con::{E, LN10, LN2, LOG10E, LOG2E, PHI, PI, TAU};

    // Functions
    pub use crate::fun::{
        abs, add, ceil, deg_to_rad, divi, f64_to_f32, fact, floor, fround, gamma, mult, pow,
        rad_to_deg, round, sign, subt,
    };
}
