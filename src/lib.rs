// Constants
pub mod con;
pub use crate::con::{_E, _PI};

// Functions
pub mod fun;
pub use crate::fun::{_abs, _add, _pow};

#[allow(dead_code)]
pub mod math {
    // Constants
    use crate::{_E, _PI};

    /// E
    ///
    /// Mathematical constant
    ///
    /// The Number e (Euler's number)
    ///
    /// 2.718281828459045
    pub const E: f64 = _E;

    /// PI
    ///
    /// Mathematical constant
    ///
    /// The Number Pi
    ///
    /// (21.991148575128552 / 7) = 3.141592653589793
    pub const PI: f64 = _PI;

    // Functions

    use crate::{_abs, _add, _pow};

    /// abs(x)
    ///
    /// Native function
    ///
    /// The abs function returns the absolute value of a number.
    ///
    /// Example:
    /// ```rust
    /// use mathlab::math::abs;
    /// assert_eq!(abs(0.0), 0.0);
    /// assert_eq!(abs(-1.0), 1.0);
    /// assert_eq!(abs(3.33), 3.33);
    /// assert_eq!(abs(-3.33), 3.33);
    /// ```
    pub fn abs(x: f64) -> f64 {
        return _abs(x);
    }

    /// add(x, y)
    ///
    /// Native function
    ///
    /// The add function accepts two floating-point numbers, x and y,
    /// as parameters and returns their sum.
    ///
    /// Example:
    /// ```rust
    /// use mathlab::math::add;
    /// assert_eq!(add(1.0, 2.0), 3.0);
    /// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
    /// assert_eq!(add(0.1, 0.2) as f64, 0.30000000000000004);
    /// assert_eq!(add(0.1, 0.2) as f32, 0.3);
    /// ```
    pub fn add(x: f64, y: f64) -> f64 {
        return _add(x, y);
    }

    /// pow(x, y)
    ///
    /// Native function
    ///
    /// The pow function is a mathematical function that computes the power of a number.
    ///
    /// Example:
    /// ```rust
    /// use mathlab::math::pow;
    /// assert_eq!(pow(0.0, 1.0), 0.0);
    /// assert_eq!(pow(0.0, 0.0), 1.0);
    /// assert_eq!(pow(0.0 / 0.0, 0.0), 1.0);
    /// assert_eq!(pow(1.0 , 0.0), 1.0);
    /// assert_eq!(pow(3.0 , 3.0), 27.0);
    /// assert_eq!(pow(2.0 , -3.0), 0.125);
    /// assert_eq!(pow(-3.0 , 2.0), 9.0);
    /// assert_eq!(pow(-3.0 , -3.0), -0.037037037037037035);
    /// assert_eq!(pow(3.33 , 3.33), 54.92110892259572);
    /// assert_eq!(pow(3.33 , -3.33), 0.01820793533883979);
    /// ```
    pub fn pow(x: f64, y: f64) -> f64 {
        return _pow(x, y);
    }
}
