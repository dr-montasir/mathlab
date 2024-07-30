use crate::constants::{E, INF_F32, INF_F64, NINF_F32, NINF_F64, PI};

/// ### abs(x)
///
/// Native Function
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
/// <small>End Fun Doc</small>
pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// ### sign(x)
///
/// Native Function
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
/// <small>End Fun Doc</small>
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
/// Rounding Function
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
/// <small>End Fun Doc</small>
pub fn floor(x: f64) -> f64 {
    x.floor()
}

/// ### ceil(x)
///
/// Rounding Function
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
/// <small>End Fun Doc</small>
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// ### round(x)
///
/// Rounding Function
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
/// <small>End Fun Doc</small>
pub fn round(x: f64) -> f64 {
    x.round()
}

/// ### fround(x)
///
/// Rounding Function
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
/// <small>End Fun Doc</small>
pub fn fround(x: f64) -> f32 {
    x as f32
}

/// ### f64_to_f32(x)
///
/// Rounding Function
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
/// <small>End Fun Doc</small>
pub fn f64_to_f32(x: f64) -> f32 {
    x as f32
}

/// ### u64_to_f64(x)
///
/// Conversion Function
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
/// <small>End Fun Doc</small>
pub fn u64_to_f64(x: u64) -> f64 {
    x as f64
}

/// ### i64_to_f64(x)
///
/// Conversion Function
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
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
/// <small>End Fun Doc</small>
pub fn is_ninf_f64(x: f64) -> bool {
    if x == NINF_F64 {
        true
    } else {
        false
    }
}

/// ### fact(x)
///
/// Native Function
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
/// <small>End Fun Doc</small>
pub fn fact(x: u64) -> u64 {
    if x == 0 {
        1
    } else {
        x * fact(x - 1)
    }
}

/// ### gamma(x)
///
/// Extended Factorial Function
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
/// <small>End Fun Doc</small>
pub fn gamma(x: u64) -> u64 {
    fact(x - 1)
}

/// ### inv(x)
///
/// Native Function
///
/// The `inv` function returns the inverse of `x`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{inv, INF_F64};
/// assert_eq!(inv(0.0), INF_F64);
/// assert_eq!(inv(1.0), 1.0);
/// assert_eq!(inv(2.0), 0.5);
/// assert_eq!(inv(10.0), 0.1);
/// assert_eq!(inv(0.1), 10.0);
/// ```
/// <small>End Fun Doc</small>
pub fn inv(x: f64) -> f64 {
    1.0 / x
}

/// ### add(x, y)
///
/// Native Function
///
/// The `add` function returns the sum of `x` and `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{add, fix64};
/// assert_eq!(add(1.0, 2.0), 3.0);
/// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f64, 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f32, 0.3); // 0.3 -> f32
/// assert_eq!(fix64(add(0.1, 0.2)), 0.3); // 0.3 -> f64
/// assert_eq!(0.1 + 0.2, 0.30000000000000004);
/// assert_eq!(fix64(0.1 + 0.2), 0.3); // 0.3 -> f64
/// ```
/// <small>End Fun Doc</small>
pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

/// ### subt(x, y)
///
/// Native Function
///
/// The `subt` function is a mathematical operation that subtracts the value of `y` from `x`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{subt, fix64, is_nan_f64, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(subt(1.0, 2.0), -1.0);
/// assert_eq!(subt(1.0, 2.0), -1.0);
/// assert_eq!(subt(0.3, 0.2), 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f64, 0.09999999999999998);
/// assert_eq!(subt(0.3, 0.2) as f32, 0.1); // 0.1 -> f32
/// assert_eq!(fix64(subt(0.3, 0.2)), 0.1); // 0.1 -> f64
/// assert!(is_nan_f64(subt(NAN_F64, 2.0)));
/// assert_eq!(subt(INF_F64, 2.0), INF_F64);
/// assert_eq!(subt(1.0, INF_F64), -INF_F64);
/// assert_eq!(subt(1.0, INF_F64), NINF_F64);
/// ```
/// <small>End Fun Doc</small>
pub fn subt(x: f64, y: f64) -> f64 {
    x - y
}

/// ### mult(x, y)
///
/// Native Function
///
/// The `mult` function is a mathematical operation that multiplies the value of `x` by `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{mult, fix64, is_nan_f64, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(mult(2.0, 3.0), 6.0);
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02); // 0.02 -> f32
/// assert_eq!(fix64(mult(0.1, 0.2)), 0.02); // 0.02 -> f64
/// assert!(is_nan_f64(mult(NAN_F64, 2.0)));
/// assert_eq!(mult(INF_F64, 2.0), INF_F64);
/// assert_eq!(mult(-1.0, INF_F64), -INF_F64);
/// assert_eq!(mult(1.0, -INF_F64), NINF_F64);
/// ```
/// <small>End Fun Doc</small>
pub fn mult(x: f64, y: f64) -> f64 {
    x * y
}

/// ### divi(x, y)
///
/// Native Function
///
/// The `divi` function is a mathematical operation that divides the value of `x` by `y`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{divi, fix64, is_nan_f64, is_inf_f64, is_ninf_f64, is_ninf_f32, NAN_F64, INF_F64, NINF_F64};
/// assert_eq!(divi(2.0, 3.0), 0.6666666666666666);
/// assert_eq!(divi(2.0, 3.0) as f32, 0.6666667); // 0.6666667 -> f32
/// assert_eq!(fix64(divi(2.0, 3.0)), 0.6666667); // 0.6666667 -> f64
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
/// <small>End Fun Doc</small>
pub fn divi(x: f64, y: f64) -> f64 {
    x / y
}

/// ### pow(x, y)
///
/// Native Function
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
/// <small>End Fun Doc</small>
pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

/// ### deg_to_rad(x)
///
/// Conversion Function
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
/// <small>End Fun Doc</small>
pub fn deg_to_rad(x: f64) -> f64 {
    x * PI / 180.0
}

/// ### rad_to_deg(x)
///
/// Conversion Function
///
/// The `rad_to_deg` function converts an `angle` from `radians` to `degrees`.
/// This is useful in `trigonometric` calculations, where `angles` are often required in `degrees`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rad_to_deg, fix64};
/// assert_eq!(rad_to_deg(0.0), 0.0);
/// assert_eq!(rad_to_deg(0.017453292519943295), 1.0);
/// assert_eq!(rad_to_deg(0.5235987755982988), 30.0);
/// assert_eq!(rad_to_deg(0.7853981633974483), 45.0);
/// assert_eq!(rad_to_deg(1.0471975511965976), 60.0);
/// assert_eq!(rad_to_deg(1.5707963267948966), 90.0);
/// assert_eq!(rad_to_deg(3.141592653589793), 180.0);
/// assert_eq!(rad_to_deg(6.283185307179586), 360.0);
/// assert_eq!(rad_to_deg(-6.283185307179586), -360.0);
/// ```
/// <small>End Fun Doc</small>
pub fn rad_to_deg(x: f64) -> f64 {
    fix64(x * 180.0 / PI)
}

/// ### sqr(x)
///
/// Native Function
///
/// The `sqr` function calculates its square by multiplying it with itself.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sqr, INF_F64};
/// assert_eq!(sqr(0.0), 0.0);
/// assert_eq!(sqr(0.1), 0.010000000000000002);
/// assert_eq!(sqr(0.1) as f32, 0.01);
/// assert_eq!(sqr(1.0), 1.0);
/// assert_eq!(sqr(2.0), 4.0);
/// assert_eq!(sqr(10.0), 100.0);
/// assert_eq!(sqr(INF_F64), INF_F64);
/// ```
/// <small>End Fun Doc</small>
pub fn sqr(x: f64) -> f64 {
    x * x
}

/// ### rem(x, y)
///
/// Operation Function
///
/// The `rem` function provides the remainder of dividing `x` by `y`, returning a `f64` floating-point number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rem, fix64, is_nan_f64, INF_F64};
/// assert!(is_nan_f64(rem(0.0, 0.0)));
/// assert!(is_nan_f64(rem(1.0, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 2.0)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert_eq!(rem(0.0, INF_F64), 0.0);
/// assert_eq!(rem(2.0, INF_F64), 2.0);
/// assert_eq!(rem(1.0, 0.1), 0.09999999999999995);
/// assert_eq!(rem(1.0, 0.1) as f32, 0.1); // f32
/// assert_eq!(fix64(rem(1.0, 0.1)), 0.1); // f64
/// assert_eq!(rem(0.0, 3.0), 0.0);
/// assert_eq!(rem(1.0, 3.0), 1.0);
/// assert_eq!(rem(2.0, 3.0), 2.0);
/// assert_eq!(rem(3.0, 3.0), 0.0);
/// assert_eq!(rem(4.0, 3.0), 1.0);
/// ```
/// <small>End Fun Doc</small>
pub fn rem(x: f64, y: f64) -> f64 {
    x % y
}

/// ### exp(x)
///
/// Operation Function
///
/// The `exp` function defined as `pow(E, x)` raises the mathematical constant `e` to the power of `x`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{exp, E};
/// assert_eq!(exp(0.0), 1.0);
/// assert_eq!(exp(-1.0), 0.36787944117144233);
/// assert_eq!(exp(-1.0) as f32, 0.36787945);
/// assert_eq!(exp(1.0), E);
/// ```
/// <small>End Fun Doc</small>
pub fn exp(x: f64) -> f64 {
    pow(E, x)
}

/// ### ln(x)
///
/// Logarithm Function
///
/// The `ln` function returns the natural logarithm (base e) of the given float value 'x'.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{ln, E, INF_F64 as inf, LN10, is_nan_f64};
/// assert!(is_nan_f64(ln(-inf)));
/// assert_eq!(ln(0.0), -inf);
/// assert_eq!(ln(1.0), 0.0);
/// assert_eq!(ln(E), 1.0);
/// assert_eq!(ln(10.0), 2.302585092994046);
/// assert_eq!(ln(10.0), LN10);
/// assert_eq!(-ln(1.5), -0.4054651081081644);
/// ```
/// <small>End Fun Doc</small>
pub fn ln(x: f64) -> f64 {
    x.ln()
}

/// ### ln1p(x)
///
/// Logarithm Function
///
/// The `ln1p` returns `ln(1+x)` (natural logarithm) more accurately than if the operations were performed separately.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{ln1p, E, INF_F64 as inf, LN10, is_nan_f64};
/// assert!(is_nan_f64(ln1p(-inf)));
/// assert_eq!(ln1p(0.0), 0.0);
/// assert_eq!(ln1p(1.0), 0.6931471805599453);
/// assert_eq!(ln1p(E), 1.3132616875182228);
/// assert_eq!(ln1p(10.0), 2.3978952727983707);
/// assert_eq!(ln1p(LN10), 1.1947055233182953);
/// assert_eq!(-ln1p(1.5), -0.9162907318741551);
/// ```
/// <small>End Fun Doc</small>
pub fn ln1p(x: f64) -> f64 {
    x.ln_1p()
}

/// ### log2(x)
///
/// Logarithm Function
///
/// The `log2` computes the base-2 logarithm of the supplied float number 'x'.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{log2, E, INF_F64 as inf, LN10, is_nan_f64};
/// assert!(is_nan_f64(log2(-inf)));
/// assert_eq!(log2(0.0), -inf);
/// assert_eq!(log2(1.0), 0.0);
/// assert_eq!(log2(E), 1.4426950408889634);
/// assert_eq!(log2(10.0), 3.321928094887362);
/// assert_eq!(log2(LN10), 1.2032544726997219);
/// assert_eq!(-log2(1.5), -0.5849625007211562);
/// ```
/// <small>End Fun Doc</small>
pub fn log2(x: f64) -> f64 {
    x.log2()
}

/// ### log10(x)
///
/// Logarithm Function
///
/// The `log10` computes the base-10 logarithm of the supplied float number 'x'.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{log10, E, INF_F64 as inf, LN10, is_nan_f64};
/// assert!(is_nan_f64(log10(-inf)));
/// assert_eq!(log10(0.0), -inf);
/// assert_eq!(log10(1.0), 0.0);
/// assert_eq!(log10(E), 0.4342944819032518);
/// assert_eq!(log10(10.0), 1.0);
/// assert_eq!(log10(LN10), 0.36221568869946325);
/// assert_eq!(-log10(1.5), -0.17609125905568124);
/// ```
/// <small>End Fun Doc</small>
pub fn log10(x: f64) -> f64 {
    x.log10()
}

/// ### fix64(x)
///
/// Fixation Function
///
/// The `fix64` function, takes a `64-bit floating-point number` (a double precision value) as input
/// and returns an equivalent `fixed-point value` with `the same bit width`. To achieve this conversion,
/// the input float is first converted to a 32-bit floating-point type (an f32) using (as f32) method. Then,
/// this intermediate value is converted back to a string representation using to_string(),
/// parsed as an f32 again, and finally returned as an `f64`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::fix64;
/// // assert_eq!(fix64("abc"), 0.3); // error: expected `f64`, found `&str`
/// // assert_eq!(fix64("0.1"), 0.3); // error: expected `f64`, found `&str`
/// // assert_eq!(fix64(1), 0.3); // error: expected `f64`, found integer
/// assert_eq!(0.1 + 0.2, 0.30000000000000004);
/// assert_eq!(0.3 - 0.2, 0.09999999999999998);
/// assert_eq!(fix64(0.1 + 0.2), 0.3);
/// assert_eq!(fix64(0.3 - 0.2), 0.1);
/// assert_eq!(fix64(0.30000000000000004), 0.3);
/// ```
/// <small>End Fun Doc</small>
pub fn fix64(x: f64) -> f64 {
    ((x) as f32).to_string().parse().expect("")
}

/// ### cube(x)
///
/// Native Function
///
/// The `cube` function computes the value of `x` raised to the power of three,
/// which is equivalent to multiplying `x` by itself three times.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cube, fix64};
/// assert_eq!(cube(0.1), 0.0010000000000000002);
/// assert_eq!(fix64(cube(0.1)), 0.001);
/// assert_eq!(cube(2.0), 8.0);
/// ```
/// <small>End Fun Doc</small>
pub fn cube(x: f64) -> f64 {
    x * x * x
}

/// ### cbrt(x)
///
/// Native Function
///
/// The `cbrt` function computes the real cube root of the given floating-point number `x`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::cbrt;
/// assert_eq!(cbrt(0.001), 0.1);
/// assert_eq!(cbrt(8.0), 2.0);
/// ```
/// <small>End Fun Doc</small>
pub fn cbrt(x: f64) -> f64 {
    x.cbrt()
}

/// ### trunc(x)
///
/// Rounding Function
///
/// The `trunc` function returns the integer part of self.
/// This means that non-integer numbers are always truncated towards zero.
///
/// ### Examples
/// ```rust
/// use mathlab::math::trunc;
/// assert_eq!(trunc(-0.37), 0.0);
/// assert_eq!(trunc(0.37), 0.0);
/// assert_eq!(trunc(-3.7), -3.0);
/// assert_eq!(trunc(3.7), 3.0);
/// ```
/// <small>End Fun Doc</small>
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

/// ### sin(x)
///
/// Trigonometric Function
///
/// The `sin` function computes the sine of a number (in radians).
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sin, deg_to_rad};
/// // x in radians.
/// assert_eq!(sin(0.0), 0.0);
/// assert_eq!(sin(9.999e-15), 0.0);
/// assert_eq!(sin(1e-14), 1e-14);
/// assert_eq!(sin(0.5235987755982988), 0.5);
/// assert_eq!(sin(0.7853981633974483), 0.70710677);
/// assert_eq!(sin(1.0471975511965976), 0.8660254);
/// assert_eq!(sin(1.5707963267948966), 1.0);
/// assert_eq!(sin(3.141592653589793), 0.0);
/// assert_eq!(sin(4.71238898038469), -1.0);
/// assert_eq!(sin(6.283185307179586), 0.0);
/// // If x is in degrees, use the deg_to_rad function.
/// assert_eq!(sin(deg_to_rad(0.0)), 0.0);
/// assert_eq!(sin(deg_to_rad(5.7295775e-13)), 0.0);   // 5.7295775e-13 deg = 9.999999e-15 rad
/// assert_eq!(sin(deg_to_rad(5.7295780e-13)), 1e-14); // 5.7295780e-13 deg = 1e-14 rad
/// assert_eq!(sin(deg_to_rad(30.0)), 0.5);
/// assert_eq!(sin(deg_to_rad(45.0)), 0.70710677);
/// assert_eq!(sin(deg_to_rad(60.0)), 0.8660254);
/// assert_eq!(sin(deg_to_rad(90.0)), 1.0);
/// assert_eq!(sin(deg_to_rad(180.0)), 0.0);
/// assert_eq!(sin(deg_to_rad(270.0)), -1.0);
/// assert_eq!(sin(deg_to_rad(360.0)), 0.0);
/// ```
/// <small>End Fun Doc</small>
pub fn sin(x: f64) -> f64 {
    if abs(x.sin()) < 1e-14 {
        0.0
    } else {
        fix64(x.sin())
    }
}

/// ### cos(x)
///
/// Trigonometric Function
///
/// The `cos` function computes the cosine of a number (in radians).
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cos, deg_to_rad};
/// // x in radians.
/// assert_eq!(cos(0.0), 1.0);
/// assert_eq!(cos(9.999e-15), 1.0);
/// assert_eq!(cos(1e-14), 1.0);
/// assert_eq!(cos(0.5235987755982988), 0.8660254);
/// assert_eq!(cos(0.7853981633974483), 0.70710677);
/// assert_eq!(cos(1.0471975511965976), 0.5);
/// assert_eq!(cos(1.5707963267948966), 0.0);
/// assert_eq!(cos(3.141592653589793), -1.0);
/// assert_eq!(cos(4.71238898038469), 0.0);
/// assert_eq!(cos(6.283185307179586), 1.0);
/// // If x is in degrees, use the deg_to_rad function.
/// assert_eq!(cos(deg_to_rad(0.0)), 1.0);
/// assert_eq!(cos(deg_to_rad(5.7295775e-13)), 1.0); // 5.7295775e-13 deg = 9.999999e-15 rad
/// assert_eq!(cos(deg_to_rad(5.7295780e-13)), 1.0); // 5.7295780e-13 deg = 1e-14 rad
/// assert_eq!(cos(deg_to_rad(30.0)), 0.8660254);
/// assert_eq!(cos(deg_to_rad(45.0)), 0.70710677);
/// assert_eq!(cos(deg_to_rad(60.0)), 0.5);
/// assert_eq!(cos(deg_to_rad(90.0)), 0.0);
/// assert_eq!(cos(deg_to_rad(180.0)), -1.0);
/// assert_eq!(cos(deg_to_rad(270.0)), 0.0);
/// assert_eq!(cos(deg_to_rad(360.0)), 1.0);
/// ```
/// <small>End Fun Doc</small>
pub fn cos(x: f64) -> f64 {
    if abs(x.cos()) >= 1e-14 {
        fix64(x.cos())
    } else {
        0.0
    }
}

/// ### tan(x)
///
/// Trigonometric Function
///
/// The `tan` function computes the tangent of a number (in radians).
///
/// ### Examples
/// ```rust
/// use mathlab::math::{tan, deg_to_rad, INF_F64 as inf};
/// // x in radians.
/// assert_eq!(tan(0.0), 0.0);
/// assert_eq!(tan(9.999e-15), 0.0);
/// assert_eq!(tan(1e-14), 1e-14);
/// assert_eq!(tan(0.5235987755982988), 0.57735026);
/// assert_eq!(tan(0.7853981633974483), 1.0);
/// assert_eq!(tan(1.0471975511965976), 1.7320508);
/// assert_eq!(tan(1.5707963267948966), inf);
/// assert_eq!(tan(3.141592653589793), 0.0);
/// assert_eq!(tan(4.71238898038469), -inf);
/// assert_eq!(tan(6.283185307179586), 0.0);
/// // If x is in degrees, use the deg_to_rad function.
/// assert_eq!(tan(deg_to_rad(0.0)), 0.0);
/// assert_eq!(tan(deg_to_rad(5.7295775e-13)), 0.0); // 5.7295775e-13 deg = 9.999999e-15 rad
/// assert_eq!(tan(deg_to_rad(5.7295780e-13)), 1e-14); // 5.7295780e-13 deg = 1e-14 rad
/// assert_eq!(tan(deg_to_rad(30.0)), 0.57735026);
/// assert_eq!(tan(deg_to_rad(45.0)), 1.0);
/// assert_eq!(tan(deg_to_rad(60.0)), 1.7320508);
/// assert_eq!(tan(deg_to_rad(90.0)), inf);
/// assert_eq!(tan(deg_to_rad(180.0)), 0.0);
/// assert_eq!(tan(deg_to_rad(270.0)), -inf);
/// assert_eq!(tan(deg_to_rad(360.0)), 0.0);
/// ```
/// <small>End Fun Doc</small>
pub fn tan(x: f64) -> f64 {
    fix64(sin(x) / cos(x))
}
