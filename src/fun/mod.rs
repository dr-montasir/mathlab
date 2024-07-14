/// ### abs(x)
///
/// Native function
///
/// The `abs` function returns the absolute value of a number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::abs;
/// assert_eq!(abs(0.0), 0.0);
/// assert_eq!(abs(-1.0), 1.0);
/// assert_eq!(abs(3.33), 3.33);
/// assert_eq!(abs(-3.33), 3.33);
/// ```
pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// ### sign(x)
///
/// Native function
///
/// The `sign` function returns only one of three possible values: `−1`, `0` or `1`.
/// ### Examples
/// ```rust
/// use mathlab::math::sign;
/// assert_eq!(sign(-9.0), -1.0);
/// assert_eq!(sign(9.0), 1.0);
/// assert_eq!(sign(--9.5), 1.0);
/// assert_eq!(sign(6.0 - 15.0), -1.0);
/// assert_eq!(sign(0.0), 0.0);
/// assert_eq!(sign(0.0 / 0.0), 0.0);
/// ```
pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// ### fact(x)
///
/// Native function
///
/// The factorial function, denoted as `n!`, is a mathematical function
/// that multiplies a given positive integer n by all the positive integers less than it.
/// The factorial of `n` is defined as:
///
/// `n!=n×(n−1)×(n−2)×…×1`
///
/// For example, the factorial of 5 (denoted as 5!) is:
///
/// `5!=5×4×3×2×1=120`
///
/// By definition, the factorial of 0 is 1, i.e., `0! = 1`.
/// ### Examples
/// ```rust
/// use mathlab::math::fact;
/// assert_eq!(fact(0), 1);
/// assert_eq!(fact(1), 1);
/// assert_eq!(fact(2), 2);
/// assert_eq!(fact(3), 6);
/// assert_eq!(fact(3) as u8, 6);
/// assert_eq!(fact(3) as i32, 6);
/// assert_eq!(fact(3) as f64, 6.0);
/// assert_eq!(fact(16), 20922789888000);
/// assert_eq!(fact(18), 6402373705728000);
/// ```
pub fn fact(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        x * fact(x - 1)
    }
}

/// ### gamma(x)
///
/// Extended factorial function
///
/// `Γ(n)` is a way to extend the factorial function to all complex numbers
/// except the negative integers and zero.
/// For any positive integer, the Gamma function is defined as:
///
/// `Γ(n)=(n−1)!`
///
/// For example, the gamma of `3` (denoted as `Γ(3)`) is:
///
/// `Γ(3)=(3−1)! = 2!=2×1=2`
///
/// By definition, the Gamma function of `0` returns an `error` because `0 − 1 = − 1`,
/// which is not accepted in the factorial function.
/// ### Examples
/// ```rust
/// use mathlab::math::gamma;
/// assert_eq!(gamma(1), 1);
/// assert_eq!(gamma(2), 1);
/// assert_eq!(gamma(3), 2);
/// assert_eq!(gamma(4), 6);
/// assert_eq!(gamma(4) as u8, 6);
/// assert_eq!(gamma(4) as i32, 6);
/// assert_eq!(gamma(4) as f64, 6.0);
/// assert_eq!(gamma(17), 20922789888000);
/// assert_eq!(gamma(19), 6402373705728000);
/// ```
pub fn gamma(x: u64) -> u64 {
    fact(x - 1)
}

/// ### add(x, y)
///
/// Native function
///
/// The `add(x, y)` function returns the sum of `x` and `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::add;
/// assert_eq!(add(1.0, 2.0), 3.0);
/// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f64, 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f32, 0.3);
/// ```
pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

/// ### subt(x, y)
///
/// Native function
///
/// The `subt(x, y)` function is a mathematical operation that subtracts the value of `y` from `x`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::subt;
/// assert_eq!(subt(1.0, 2.0), -1.0);
/// assert_eq!(subt(0.3, 0.2), 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f64, 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f32, 0.1);
/// ```
pub fn subt(x: f64, y: f64) -> f64 {
    x - y
}

/// ### mult(x, y)
///
/// Native function
///
/// The `mult(x, y)` function is a mathematical operation that multiplies the value of `x` by `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::mult;
/// assert_eq!(mult(2.0, 3.0), 6.0);
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f64, 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
/// ```
pub fn mult(x: f64, y: f64) -> f64 {
    x * y
}

/// ### divi(x, y)
///
/// Native function
///
/// The `divi(x, y)` function is a mathematical operation that divides the value of `x` by `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::divi;
/// assert_eq!(divi(2.0, 3.0), 0.6666666666666666);
/// assert_eq!(divi(2.0, 3.0) as f32, 0.6666667);
/// assert_eq!(divi(0.3, 0.6), 0.5);
/// assert_eq!(divi(0.3, 0.6) as f64, 0.5);
/// assert_eq!(divi(0.3, 0.6) as f32, 0.5);
/// ```
pub fn divi(x: f64, y: f64) -> f64 {
    x / y
}

/// ### pow(x, y)
///
/// Native function
///
/// The `pow` function is a mathematical function that computes the power of a number.
///
/// ### Examples
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
    x.powf(y)
}
