// Constants
pub mod con;
pub use crate::con::{_E, _LN10, _LN2, _LOG10E, _LOG2E, _PHI, _PI, _TAU};

// Functions
pub mod fun;
pub use crate::fun::{_abs, _add, _divi, _mult, _pow, _subt};

#[allow(dead_code)]
pub mod math {
    // Constants
    use crate::{_E, _LN10, _LN2, _LOG10E, _LOG2E, _PHI, _PI, _TAU};

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

    /// PHI
    ///
    /// Mathematical constant
    ///
    /// The Golden Ratio (Phi)
    ///
    /// (1 + sqrt(5)) / 2 = 1.618033988749895
    pub const PHI: f64 = _PHI;

    /// TAU
    ///
    /// Mathematical constant
    ///
    /// Tau is a circle constant and the value is equivalent to 2π
    ///
    /// (2 * PI) = 6.283185307179586
    pub const TAU: f64 = _TAU;

    /// LN2
    ///
    /// Mathematical constant
    ///
    /// The natural logarithm of 2
    ///
    /// 0.693147180559945
    pub const LN2: f64 = _LN2;

    /// LN10
    ///
    /// Mathematical constant
    ///
    /// The natural logarithm of 10
    ///
    /// 2.302585092994046
    pub const LN10: f64 = _LN10;

    /// LOG2E
    ///
    /// Mathematical constant
    ///
    /// The base 2 logarithm of E
    ///
    /// 1.442695040888963
    pub const LOG2E: f64 = _LOG2E;

    /// LOG10E
    ///
    /// Mathematical constant
    ///
    /// The base 10 logarithm of E
    ///
    /// 0.434294481903252
    pub const LOG10E: f64 = _LOG10E;

    // Functions
    use crate::{_abs, _add, _divi, _mult, _pow, _subt};

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
    /// The add(x, y) function returns the sum of x and y.
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

    /// subt(x, y)
    ///
    /// Native function
    ///
    /// The subt(x, y) function is a mathematical operation that subtracts the value of y from x.
    ///
    /// Example
    /// ```rust
    /// use mathlab::math::subt;
    /// assert_eq!(subt(1.0, 2.0), -1.0);
    /// assert_eq!(subt(0.3, 0.2), 0.09999999999999998);
    /// assert_eq!(subt(0.3, 0.2) as f64, 0.09999999999999998);
    /// assert_eq!(subt(0.3, 0.2) as f32, 0.1);
    /// ```
    pub fn subt(x: f64, y: f64) -> f64 {
        return _subt(x, y);
    }

    /// _mult(x, y)
    ///
    /// Native function
    ///
    /// The mult(x, y) function is a mathematical operation that multiplies the value of x by y.
    ///
    /// Example:
    /// ```rust
    /// use mathlab::math::mult;
    /// assert_eq!(mult(2.0, 3.0), 6.0);
    /// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
    /// assert_eq!(mult(0.1, 0.2) as f64, 0.020000000000000004);
    /// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
    /// ```
    pub fn mult(x: f64, y: f64) -> f64 {
        return _mult(x, y);
    }

    /// divi(x, y)
    ///
    /// Native function
    ///
    /// The divi(x, y) function is a mathematical operation that divides the value of x by y.
    ///
    /// Example:
    /// ```rust
    /// use mathlab::math::divi;
    /// assert_eq!(divi(2.0, 3.0), 0.6666666666666666);
    /// assert_eq!(divi(2.0, 3.0) as f32, 0.6666667);
    /// assert_eq!(divi(0.3, 0.6), 0.5);
    /// assert_eq!(divi(0.3, 0.6) as f64, 0.5);
    /// assert_eq!(divi(0.3, 0.6) as f32, 0.5);
    /// ```
    pub fn divi(x: f64, y: f64) -> f64 {
        return _divi(x, y);
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
