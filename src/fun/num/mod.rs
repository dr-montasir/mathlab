use crate::con::{INF_F32, INF_F64, NINF_F32, NINF_F64, PI};

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

/// ### floor(x)
///
/// Rounding function
///
/// The `floor` function returns the largest integer less than or equal to a given floating-point number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::floor;
/// assert_eq!(floor(0.0), 0.0);
/// assert_eq!(floor(0.99), 0.0);
/// assert_eq!(floor(-0.99), -1.0);
/// assert_eq!(floor(1.99), 1.0);
/// assert_eq!(floor(1.01), 1.0);
/// assert_eq!(floor(-1.99), -2.0);
/// ```
pub fn floor(x: f64) -> f64 {
    x.floor()
}

/// ### ceil(x)
///
/// Rounding function
///
/// The `ceil` function rounds a number up to the nearest integer greater than or equal to it.
///
/// ### Examples
/// ```rust
/// use mathlab::math::ceil;
/// assert_eq!(ceil(0.0), 0.0);
/// assert_eq!(ceil(0.99), 1.0);
/// assert_eq!(ceil(-0.99), 0.0);
/// assert_eq!(ceil(1.99), 2.0);
/// assert_eq!(ceil(1.01), 2.0);
/// assert_eq!(ceil(-1.99), -1.0);
/// ```
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// ### round(x)
///
/// Rounding function
///
/// The `round` function aligns a number to the closest integer,
/// adjusting fractions of `0.5` or greater up, and less than `0.5` down.
///
/// The native way to define a `round` function in `Rust` is:
/// ```rust
/// pub fn round(x: f64) -> f64 {
///     x.round()
/// }
/// ```
///
/// The alternative way to define a native round function is by using the ceil function for negative numbers and the floor function for non-negative numbers.
/// ```rust
/// use mathlab::math::{ceil, floor};
/// pub fn round(x: f64) -> f64 {
///     if x < 0.0 {
///         ceil(x - 0.5)
///     } else {
///         floor(x + 0.5)
///     }
/// }
/// ```
///
/// ### Examples
/// ```rust
/// use mathlab::math::round;
/// assert_eq!(round(0.0), 0.0);
/// assert_eq!(round(0.5), 1.0);
/// assert_eq!(round(-0.5), -1.0);
/// assert_eq!(round(1.99), 2.0);
/// assert_eq!(round(1.01), 1.0);
/// assert_eq!(round(-1.99), -2.0);
/// ```
pub fn round(x: f64) -> f64 {
    x.round()
}

/// ### fround(x)
///
/// Rounding function
///
/// The `fround` function performs the same operation as the `f64_to_f32` function.
///
/// The `fround` function rounds a floating-point number to the nearest
/// single-precision `(32-bit)` floating-point number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::fround;
/// assert_eq!(fround(0.6666666666666666), 0.6666667);
/// assert_eq!(fround(0.30000000000000004), 0.3);
/// assert_eq!(fround(0.020000000000000004), 0.02);
/// assert_eq!(fround(0.09999999999999998), 0.1);
/// ```
pub fn fround(x: f64) -> f32 {
    x as f32
}

/// ### f64_to_f32(x)
///
/// Rounding function
///
/// The `f64_to_f32` function performs the same operation as the `fround` function.
///
/// The `f64_to_f32` function rounds a floating-point number to the nearest
/// single-precision `(32-bit)` floating-point number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::f64_to_f32;
/// assert_eq!(f64_to_f32(0.6666666666666666), 0.6666667);
/// assert_eq!(f64_to_f32(0.30000000000000004), 0.3);
/// assert_eq!(f64_to_f32(0.020000000000000004), 0.02);
/// assert_eq!(f64_to_f32(0.09999999999999998), 0.1);
/// ```
pub fn f64_to_f32(x: f64) -> f32 {
    x as f32
}

/// ### u64_to_f64(x)
///
/// Conversion function
///
/// The `u64_to_f64` function takes a u64 value as input and returns its `f64` representation.
///
/// ### Examples
/// ```rust
/// use mathlab::math::u64_to_f64;
/// assert_eq!(u64_to_f64(0), 0.0);
/// assert_eq!(u64_to_f64(1), 1.0);
/// assert_eq!(u64_to_f64(2), 2.0);
/// assert_eq!(u64_to_f64(3), 3.0);
/// ```
pub fn u64_to_f64(x: u64) -> f64 {
    x as f64
}

/// ### i64_to_f64(x)
///
/// Conversion function
///
/// The `i64_to_f64` function takes a i64 value as input and returns its `f64` representation.
///
/// ### Examples
/// ```rust
/// use mathlab::math::i64_to_f64;
/// assert_eq!(i64_to_f64(-2), -2.0);
/// assert_eq!(i64_to_f64(-1), -1.0);
/// assert_eq!(i64_to_f64(0), 0.0);
/// assert_eq!(i64_to_f64(1), 1.0);
/// assert_eq!(i64_to_f64(2), 2.0);
/// ```
pub fn i64_to_f64(x: i64) -> f64 {
    x as f64
}

/// ### is_nan_f32(x)
///
/// Boolean Check Function
///
/// The `is_nan_f32` function is a utility method used to check whether a given `f32 number`
/// represents Not a Number (`NaN`) according to `IEEE 754` arithmetic standards.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{NAN_F64, is_nan_f32, f64_to_f32, add};
/// assert!(is_nan_f32(add(NAN_F64, 1.0) as f32));
/// assert!(is_nan_f32(f64_to_f32(add(NAN_F64, 1.0))));
/// assert_eq!(assert!(is_nan_f32(add(NAN_F64, 1.0) as f32)), assert!(is_nan_f32(f64_to_f32(add(NAN_F64, 1.0)))));
/// ```
pub fn is_nan_f32(x: f32) -> bool {
    x != x
}

/// ### is_inf_f32(x)
///
/// Boolean Check Function
///
/// The `is_inf_f32` function is a utility method that helps determine whether a specified `f32 number`
/// follows the convention of `infinity` under the `IEEE 754` standard for floating-point arithmetic.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{is_inf_f32, f64_to_f32, divi};
/// assert!(is_inf_f32(divi(2.0, 0.0) as f32));
/// assert!(is_inf_f32(f64_to_f32(divi(2.0, 0.0))));
/// assert_eq!(assert!(is_inf_f32(divi(2.0, 0.0) as f32)), assert!(is_inf_f32(f64_to_f32(divi(2.0, 0.0)))));
/// ```
pub fn is_inf_f32(x: f32) -> bool {
    if x == INF_F32 {
        true
    } else {
        false
    }
}

/// ### is_ninf_f32(x)
///
/// Boolean Check Function
///
/// The `is_ninf_f32` function helps determine whether a provided `f32 negative number` follows
/// the convention of `negative infinity` as per the `IEEE 754` standard for floating-point arithmetic.
///
///
/// ### Examples
/// ```rust
/// use mathlab::math::{is_ninf_f32, f64_to_f32, divi};
/// assert!(is_ninf_f32(divi(-2.0, 0.0) as f32));
/// assert!(is_ninf_f32(f64_to_f32(divi(-2.0, 0.0))));
/// assert_eq!(assert!(is_ninf_f32(divi(-2.0, 0.0) as f32)), assert!(is_ninf_f32(f64_to_f32(divi(-2.0, 0.0)))));
/// ```
pub fn is_ninf_f32(x: f32) -> bool {
    if x == NINF_F32 {
        true
    } else {
        false
    }
}

/// ### is_nan_f64(x)
///
/// Boolean Check Function
///
/// The `is_nan_f64` function is a utility method used to check whether a given `f64 number`
/// represents Not a Number (`NaN`) according to `IEEE 754` arithmetic standards.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{NAN_F64, is_nan_f64, add, divi};
/// assert!(is_nan_f64(add(NAN_F64, 1.0)));
/// assert!(is_nan_f64(divi(0.0, 0.0)));
/// assert_eq!(assert!(is_nan_f64(add(NAN_F64, 1.0))), assert!(is_nan_f64(divi(0.0, 0.0))));
/// ```
pub fn is_nan_f64(x: f64) -> bool {
    x != x
}

/// ### is_inf_f64(x)
///
/// Boolean Check Function
///
/// The `is_inf_f64` function is a utility method that helps determine whether a specified `f64 number`
/// follows the convention of `infinity` under the `IEEE 754` standard for floating-point arithmetic.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{INF_F64, is_inf_f64, divi, mult};
/// assert!(is_inf_f64(divi(2.0, 0.0)));
/// assert!(is_inf_f64(mult(10.0, INF_F64)));
/// assert_eq!(assert!(is_inf_f64(divi(2.0, 0.0))), assert!(is_inf_f64(mult(10.0, INF_F64))));
/// ```
pub fn is_inf_f64(x: f64) -> bool {
    if x == INF_F64 {
        true
    } else {
        false
    }
}

/// ### is_ninf_f64(x)
///
/// Boolean Check Function
///
/// The `is_ninf_f64` function helps determine whether a provided `f64 negative number` follows
/// the convention of `negative infinity` as per the `IEEE 754` standard for floating-point arithmetic.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{INF_F64, NINF_F64, is_ninf_f64, divi, mult};
/// assert!(is_ninf_f64(divi(-2.0, 0.0)));
/// assert!(is_ninf_f64(mult(10.0, -INF_F64)));
/// assert_eq!(assert!(is_ninf_f64(divi(-2.0, 0.0))), assert!(is_ninf_f64(mult(10.0, -INF_F64))));
/// assert_eq!(assert!(is_ninf_f64(divi(-2.0, 0.0))), assert!(is_ninf_f64(mult(10.0, NINF_F64))));
/// ```
pub fn is_ninf_f64(x: f64) -> bool {
    if x == NINF_F64 {
        true
    } else {
        false
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
/// use mathlab::math::{subt, is_nan_f64, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(subt(1.0, 2.0), -1.0);
/// assert_eq!(subt(1.0, 2.0), -1.0);
/// assert_eq!(subt(0.3, 0.2), 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f64, 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f32, 0.1);
/// assert!(is_nan_f64(subt(NAN_F64, 2.0)));
/// assert_eq!(subt(INF_F64, 2.0), INF_F64);
/// assert_eq!(subt(1.0, INF_F64), -INF_F64);
/// assert_eq!(subt(1.0, INF_F64), NINF_F64);
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
/// use mathlab::math::{mult, is_nan_f64, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(mult(2.0, 3.0), 6.0);
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f64, 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
/// assert!(is_nan_f64(mult(NAN_F64, 2.0)));
/// assert_eq!(mult(INF_F64, 2.0), INF_F64);
/// assert_eq!(mult(-1.0, INF_F64), -INF_F64);
/// assert_eq!(mult(1.0, -INF_F64), NINF_F64);
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
/// use mathlab::math::{divi, is_nan_f64, is_inf_f64, is_ninf_f64, is_ninf_f32, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(divi(2.0, 3.0), 0.6666666666666666);
/// assert_eq!(divi(2.0, 3.0) as f32, 0.6666667);
/// assert_eq!(divi(2.0, 3.0) as f32, 0.6666667 as f32);
/// assert_eq!(divi(0.3, 0.6), 0.5);
/// assert_eq!(divi(0.3, 0.6) as f64, 0.5);
/// assert_eq!(divi(0.3, 0.6) as f32, 0.5);
///
/// assert_eq!(divi(0.3, 0.0), INF_F64);
/// assert_eq!(divi(0.3, 0.0) as f32, INF_F64 as f32);
/// assert_eq!(divi(-0.3, 0.0), -INF_F64);
/// assert_eq!(divi(-0.3, 0.0), NINF_F64);
///
/// assert!(is_nan_f64(divi(0.0, 0.0)));
/// assert!(is_inf_f64(divi(1.0, 0.0)));
/// assert!(is_ninf_f64(divi(-1.0, 0.0)));
/// assert!(is_ninf_f32(divi(-1.0, 0.0) as f32));
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
/// use mathlab::math::{pow, is_nan_f64, NAN_F64, INF_F64};
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
/// assert!(is_nan_f64(pow(NAN_F64, 2.0)));
/// assert_eq!(pow(INF_F64, 2.0), INF_F64);
/// ```
pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

/// ### deg_to_rad(x)
///
/// Conversion function
///
/// The `deg_to_rad` function converts an `angle` from `degrees` to `radians`.
/// This is useful in `trigonometric` calculations, where `angles` are often required in `radians`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::deg_to_rad;
/// assert_eq!(deg_to_rad(0.0), 0.0);
/// assert_eq!(deg_to_rad(1.0), 0.017453292519943295);
/// assert_eq!(deg_to_rad(30.0), 0.5235987755982988);
/// assert_eq!(deg_to_rad(45.0), 0.7853981633974483);
/// assert_eq!(deg_to_rad(60.0), 1.0471975511965976);
/// assert_eq!(deg_to_rad(90.0), 1.5707963267948966);
/// assert_eq!(deg_to_rad(180.0), 3.141592653589793);
/// assert_eq!(deg_to_rad(360.0), 6.283185307179586);
/// assert_eq!(deg_to_rad(-360.0), -6.283185307179586);
/// ```
pub fn deg_to_rad(x: f64) -> f64 {
    x * PI / 180.0
}

/// ### rad_to_deg(x)
///
/// Conversion function
///
/// The `rad_to_deg` function converts an `angle` from `radians` to `degrees`.
/// This is useful in `trigonometric` calculations, where `angles` are often required in `degrees`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::rad_to_deg;
/// assert_eq!(rad_to_deg(0.0), 0.0);
/// assert_eq!(rad_to_deg(0.017453292519943295), 1.0);
/// assert_eq!(rad_to_deg(0.017453292519943295) as f32, 1.0);
/// assert_eq!(rad_to_deg(0.5235987755982988), 29.999999999999996);
/// assert_eq!(rad_to_deg(0.5235987755982988) as f32, 30.0);
/// assert_eq!(rad_to_deg(0.7853981633974483), 45.0);
/// assert_eq!(rad_to_deg(0.7853981633974483) as f32, 45.0);
/// assert_eq!(rad_to_deg(1.0471975511965976), 59.99999999999999);
/// assert_eq!(rad_to_deg(1.0471975511965976) as f32, 60.0);
/// assert_eq!(rad_to_deg(1.5707963267948966), 90.0);
/// assert_eq!(rad_to_deg(1.5707963267948966) as f32, 90.0);
/// assert_eq!(rad_to_deg(3.141592653589793), 180.0);
/// assert_eq!(rad_to_deg(3.141592653589793) as f32, 180.0);
/// assert_eq!(rad_to_deg(6.283185307179586), 360.0);
/// assert_eq!(rad_to_deg(6.283185307179586) as f32, 360.0);
/// assert_eq!(rad_to_deg(-6.283185307179586), -360.0);
/// assert_eq!(rad_to_deg(-6.283185307179586) as f32, -360.0);
/// ```
pub fn rad_to_deg(x: f64) -> f64 {
    x * 180.0 / PI
}
