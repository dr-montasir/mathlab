use super::num::{
    abs, cbrt, ceil, cos, cos_deg, cot, cot_deg, csc, csc_deg, cube, deg_to_rad, exp, f64_to_f32,
    fact, fix64, floor, fround, gamma, i64_to_f64, inv, ln, ln1p, log10, log2, rad_to_deg, round,
    sec, sec_deg, sign, sin, sin_deg, sqr, tan, tan_deg, trunc, u64_to_f64,
};

/// ### abs_vec(x)
///
/// Native Function
///
/// The `abs_vec` function takes a single parameter, a slice of floating-point numbers (`&[f64]`),
/// and returns a vector (`Vec<f64>`) containing the absolute values of each element in the input slice.
/// The function iterates over each element in the slice,
/// computes its absolute value, and collects the results into a new vector.
///
/// The `abs_vec` function can be written in two ways:
///
/// ```rust
/// use mathlab::math::abs;
/// let my_x_f64_array = [0.0, -1.0, 3.33, -3.33];
/// // with a reference (&)
/// pub fn abs_vec(x: &[f64]) -> Vec<f64> {
///     x.iter().map(|&x| x.abs()).collect()
/// }
///
/// assert_eq!(abs_vec(&my_x_f64_array), [0.0, 1.0, 3.33, 3.33]);
/// ```
///
/// or directly without a reference `&`
///
/// ```rust
/// use mathlab::math::abs;
/// let my_x_f64_array = [0.0, -1.0, 3.33, -3.33];
/// pub fn abs_vec<const N: usize>(x: [f64; N]) -> Vec<f64> {
///     x.iter().map(|&x| x.abs()).collect()
/// }
///
/// assert_eq!(abs_vec(my_x_f64_array), vec![0.0, 1.0, 3.33, 3.33]);
/// ```
///
/// For most cases, using a slice (`&[f64]`) is more efficient and flexible,
/// especially for larger arrays or when the function needs to handle variable-length input.
/// It avoids the overhead of copying the entire array and allows the function
/// to work with any contiguous memory block.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{abs, abs_vec};
/// let my_x_f64_array = [0.0, -1.0, 3.33, -3.33];
/// assert_eq!(abs(-1.0), 1.0);
/// assert_eq!(abs_vec(&my_x_f64_array), [0.0, 1.0, 3.33, 3.33]);
/// ```
/// <small>End Fun Doc</small>
pub fn abs_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| abs(x)).collect()
}

/// ### sign_vec(x)
///
/// Native Function
///
/// The `sign_vec` function takes a slice of floating-point numbers (`&[f64]`)
/// and returns a vector (`Vec<f64>`) containing the sign of each element in the input slice.
/// It iterates over each element, applies the sign function, and collects the results into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sign, sign_vec};
/// let my_x_f64_array = [-9.0, 9.0, --9.5, 6.0 - 15.0, 0.0, 0.0 / 0.0];
/// assert_eq!(sign(-9.0), -1.0);
/// assert_eq!(sign_vec(&my_x_f64_array), [-1.0, 1.0, 1.0, -1.0, 0.0, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn sign_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sign(x)).collect()
}

/// ### fact_vec(x)
///
/// Native Function
///
/// The `fact_vec` function takes a slice of `unsigned` `64-bit` integers
/// and returns a vector containing the factorial of each element in the input slice.
/// It iterates over the input slice, computes the factorial of each element using the fact function,
/// and collects the results into a vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{fact, fact_vec, u64_to_f64_vec};
/// let my_x_u64_array = [0, 1, 2, 3, 16, 18];
/// assert_eq!(fact(3), 6);
/// assert_eq!(fact(3) as f64, 6.0);
/// assert_eq!(fact_vec(&my_x_u64_array), [1, 1, 2, 6, 20922789888000, 6402373705728000]);
/// assert_eq!(fact_vec(&my_x_u64_array).iter().map(|&x| x as f64).collect::<Vec<f64>>(), vec![1.0, 1.0, 2.0, 6.0, 20922789888000.0, 6402373705728000.0]);
/// assert_eq!(u64_to_f64_vec(&fact_vec(&my_x_u64_array)), [1.0, 1.0, 2.0, 6.0, 20922789888000.0, 6402373705728000.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn fact_vec(x: &[u64]) -> Vec<u64> {
    x.iter().map(|&x| fact(x)).collect()
}

/// ### gamma_vec(x)
///
/// Extended Factorial Function
///
/// The `gamma_vec` function takes a slice of `64-bit unsigned` integers (`&[u64]`)
/// and returns a vector of `64-bit unsigned` integers (`Vec<u64]`).
/// It applies the gamma function to each element in the input slice and collects the results into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{gamma, gamma_vec, u64_to_f64_vec};
/// let my_x_u64_array = [1, 2, 3, 4, 17, 19];
/// assert_eq!(gamma(3), 2);
/// assert_eq!(gamma(3) as f64, 2.0);
/// assert_eq!(gamma_vec(&my_x_u64_array), [1, 1, 2, 6, 20922789888000, 6402373705728000]);
/// assert_eq!(gamma_vec(&my_x_u64_array).iter().map(|&x| x as f64).collect::<Vec<f64>>(), vec![1.0, 1.0, 2.0, 6.0, 20922789888000.0, 6402373705728000.0]);
/// assert_eq!(u64_to_f64_vec(&gamma_vec(&my_x_u64_array)), [1.0, 1.0, 2.0, 6.0, 20922789888000.0, 6402373705728000.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn gamma_vec(x: &[u64]) -> Vec<u64> {
    x.iter().map(|&x| gamma(x)).collect()
}

/// ### inv_vec(x)
///
/// Native Function
///
/// The `inv_vec` function takes a slice of floating-point numbers `x` and
/// returns a new vector containing the inverses of each element in `x`
///
/// ### Examples
/// ```rust
/// use mathlab::math::{inv, inv_vec, INF_F64, INF_F32, fround_vec};
/// assert_eq!(inv(0.0), INF_F64);
/// assert_eq!(inv(0.1), 10.0);
/// assert_eq!(inv_vec(&[0.0, 0.1, 0.2, 0.3]), [INF_F64, 10.0, 5.0, 3.3333333333333335]);
/// assert_eq!(fround_vec(&inv_vec(&[0.0, 0.1, 0.2, 0.3])), [INF_F32, 10.0, 5.0, 3.3333333]);
/// assert_eq!(fround_vec(&inv_vec(&[0.0, 0.1, 0.2, 0.3])), [INF_F32, 10.0, 5.0, 3.3333333333333335]);
/// ```
/// <small>End Fun Doc</small>
pub fn inv_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| inv(x)).collect()
}

/// ### floor_vec(x)
///
/// Rounding Function
///
/// The `floor_vec` function takes a slice of `f64` values as input and returns
/// a `Vec<f64>` where each element is the floor value of the corresponding input element.
/// The floor value of a number is the largest integer less than or equal to that number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{floor, floor_vec};
/// let my_x_f64_array = [0.0, 0.99, 1.01, 1.99, -0.99, -1.01, -1.99];
/// assert_eq!(floor(1.99), 1.0);
/// assert_eq!(floor_vec(&my_x_f64_array), [0.0, 0.0, 1.0, 1.0, -1.0, -2.0, -2.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn floor_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| floor(x)).collect()
}

/// ### ceil_vec(x)
///
/// Rounding Function
///
/// The `ceil_vec` function takes a slice of `f64` values as input and returns
/// a `Vec<f64>` where each element is the ceiling value of the corresponding input element.
/// The ceiling value of a number is the smallest integer greater than or equal to that number.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{ceil, ceil_vec};
/// let my_x_f64_array = [0.0, 0.99, 1.01, 1.99, -0.99, -1.01, -1.99];
/// assert_eq!(ceil(1.99), 2.0);
/// assert_eq!(ceil_vec(&my_x_f64_array), [0.0, 1.0, 2.0, 2.0, 0.0, -1.0, -1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn ceil_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| ceil(x)).collect()
}

/// ### round_vec(x)
///
/// Rounding Function
///
/// The `round_vec` function takes a slice of `f64` values as input and returns
/// a `Vec<f64>` where each element is the rounded value of the corresponding input element.
/// The `round` function aligns a number to the closest integer,
/// adjusting fractions of `0.5` or greater up, and less than `0.5` down.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{round, round_vec};
/// let my_x_f64_array = [0.0, 0.5, 1.01, 1.49, 1.99, -0.5, -1.01, -1.49, -1.99];
/// assert_eq!(round(1.49), 1.0);
/// assert_eq!(round(1.5), 2.0);
/// assert_eq!(round_vec(&my_x_f64_array), [0.0, 1.0, 1.0, 1.0, 2.0, -1.0, -1.0, -1.0, -2.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn round_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| round(x)).collect()
}

/// ### fround_vec(x)
///
/// Rounding Function
///
/// The `fround_vec` function takes a slice of `f64` values as input and returns
/// a `Vec<f32>` where each element is the `f32` representation of the corresponding input element.
/// The `fround` function converts a `f64` value to a `f32` value.
///
/// The `fround_vec` function performs the same operation as the `f64_to_f32_vec` function.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{fround, fround_vec};
/// let my_x_f64_array = [0.6666666666666666, 0.30000000000000004, 0.020000000000000004, 0.09999999999999998];
/// assert_eq!(fround(0.30000000000000004), 0.3);
/// assert_eq!(fround(0.09999999999999998), 0.1);
/// assert_eq!(fround_vec(&my_x_f64_array), [0.6666667, 0.3, 0.02, 0.1]);
/// ```
/// <small>End Fun Doc</small>
pub fn fround_vec(x: &[f64]) -> Vec<f32> {
    x.iter().map(|&x| fround(x)).collect()
}

/// ### f64_to_f32_vec(x)
///
/// Conversion Function
///
/// The `f64_to_f32_vec` function takes a slice of `f64` values as input and returns
/// a `Vec<f32>` where each element is the `f32` representation of the corresponding input element.
/// The `f64_to_f32` function converts a `f64` value to a `f32` value.
///
/// The `f64_to_f32_vec` function performs the same operation as the `fround_vec` function.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{f64_to_f32, f64_to_f32_vec};
/// let my_x_f64_array = [0.6666666666666666, 0.30000000000000004, 0.020000000000000004, 0.09999999999999998];
/// assert_eq!(f64_to_f32(0.30000000000000004), 0.3);
/// assert_eq!(f64_to_f32(0.09999999999999998), 0.1);
/// assert_eq!(f64_to_f32_vec(&my_x_f64_array), [0.6666667, 0.3, 0.02, 0.1]);
/// ```
/// <small>End Fun Doc</small>
pub fn f64_to_f32_vec(x: &[f64]) -> Vec<f32> {
    x.iter().map(|&x| f64_to_f32(x)).collect()
}

/// ### u64_to_f64_vec(x)
///
/// Conversion Function
///
/// The `u64_to_f64_vec` function takes a slice of `u64` values as input
/// and returns a `Vec<f64>` where each element is the `f64`
/// representation of the corresponding input element using the `u64_to_f64` function.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{u64_to_f64, u64_to_f64_vec};
/// let my_x_f64_array = [0, 1, 2, 3];
/// assert_eq!(u64_to_f64(0), 0.0);
/// assert_eq!(u64_to_f64_vec(&my_x_f64_array), [0.0, 1.0, 2.0, 3.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn u64_to_f64_vec(x: &[u64]) -> Vec<f64> {
    x.iter().map(|&x| u64_to_f64(x)).collect()
}

/// ### i64_to_f64_vec(x)
///
/// Conversion Function
///
/// The `i64_to_f64_vec` function takes a slice of `i64` values as input
/// and returns a `Vec<f64>` where each element is the `f64`
/// representation of the corresponding input element using the `i64_to_f64` function.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{i64_to_f64, i64_to_f64_vec};
/// let my_x_f64_array = [-1, -2, 0, 1, 2];
/// assert_eq!(i64_to_f64(0), 0.0);
/// assert_eq!(i64_to_f64_vec(&my_x_f64_array), [-1.0, -2.0, 0.0, 1.0, 2.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn i64_to_f64_vec(x: &[i64]) -> Vec<f64> {
    x.iter().map(|&x| i64_to_f64(x)).collect()
}

/// ### deg_to_rad_vec(x)
///
/// Conversion Function
///
/// The `deg_to_rad_vec` function takes a slice of `f64` values
/// representing `angles` in `degrees` and returns a `Vec<f64>`
/// containing the corresponding `angles` in `radians`.
/// It uses the `deg_to_rad` function to convert each `angle` from `degrees` to `radians`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{deg_to_rad, deg_to_rad_vec};
/// let my_x_f64_array = [0.0, 1.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0, -360.0];
/// assert_eq!(deg_to_rad(30.0), 0.5235987755982988);
/// assert_eq!(deg_to_rad_vec(&my_x_f64_array), [0.0, 0.017453292519943295, 0.5235987755982988, 0.7853981633974483, 1.0471975511965976, 1.5707963267948966, 3.141592653589793, 6.283185307179586, -6.283185307179586]);
/// ```
/// <small>End Fun Doc</small>
pub fn deg_to_rad_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| deg_to_rad(x)).collect()
}

/// ### rad_to_deg_vec(x)
///
/// Conversion Function
///
/// The `rad_to_deg_vec` function takes a slice of `f64` values
/// representing `angles` in `radians` and returns a `Vec<f64>`
/// containing the corresponding `angles` in `degrees`.
/// It uses the `rad_to_deg` function to convert each `angle` from `radians` to `degrees`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rad_to_deg, rad_to_deg_vec};
/// let my_x_f64_array = [0.0, 0.017453292519943295, 0.5235987755982988, 0.7853981633974483, 1.0471975511965976, 1.5707963267948966, 3.141592653589793, 6.283185307179586, -6.283185307179586];
/// assert_eq!(rad_to_deg(0.5235987755982988), 30.0);
/// assert_eq!(rad_to_deg_vec(&my_x_f64_array), [0.0, 1.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0, -360.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn rad_to_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| rad_to_deg(x)).collect()
}

/// ### sqr_vec(x)
///
/// Native Function
///
/// The `sqr_vec` function applies the `sqr` operation to every element of a provided slice of floats,
/// collecting their squares into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sqr, sqr_vec, INF_F64, INF_F32, fround_vec};
/// assert_eq!(sqr(0.1), 0.010000000000000002);
/// assert_eq!(sqr(0.1) as f32, 0.01);
/// assert_eq!(sqr_vec(&[0.0, 0.1, 1.0, 2.0, 10.0, INF_F64]), [0.0, 0.010000000000000002, 1.0, 4.0, 100.0, INF_F64]);
/// assert_eq!(fround_vec(&sqr_vec(&[0.0, 0.1, 1.0, 2.0, 10.0, INF_F64])), [0.0, 0.01, 1.0, 4.0, 100.0, INF_F32]);
/// ```
/// <small>End Fun Doc</small>
pub fn sqr_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sqr(x)).collect()
}

/// ### exp_vec(x)
///
/// Operation Function
///
/// The `exp_vec` function applies the exponential function `exp(x)` to each element
/// of a provided slice of floats, collects their results into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{exp, exp_vec, E};
/// assert_eq!(exp(0.0), 1.0);
/// assert_eq!(exp(-1.0) as f32, 0.36787945);
/// assert_eq!(exp_vec(&[-1.0, 0.0, 1.0]), &[0.36787944117144233, 1.0, E]);
/// ```
/// <small>End Fun Doc</small>
pub fn exp_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| exp(x)).collect()
}

/// ### ln_vec(x)
///
/// Logarithm Function
///
/// The `ln_vec` function applies the natural logarithm `ln` to each element of a given slice of floats,
/// producing a new vector containing the resultant values.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{ln, ln_vec, is_nan_f64, E, INF_F64 as inf, LN10};
/// assert!(is_nan_f64(ln(-inf)));
/// assert_eq!(ln(10.0), 2.302585092994046);
/// assert_eq!(ln_vec(&[0.0, 1.0, E, 10.0, 1.5]), &[-inf, 0.0, 1.0, LN10, 0.4054651081081644]);
/// ```
/// <small>End Fun Doc</small>
pub fn ln_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| ln(x)).collect()
}

/// ### ln1p_vec(x)
///
/// Logarithm Function
///
/// The `ln1p_vec` function computes the natural logarithm (base e) of all elements in a slice,
/// accounting for numerically stable evaluation near zero through the use of the "Lambert W" function,
/// returns the results as a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{ln1p, ln1p_vec, is_nan_f64, E, INF_F64 as inf, LN10};
/// assert!(is_nan_f64(ln1p(-inf)));
/// assert_eq!(ln1p(0.0), 0.0);
/// assert_eq!(ln1p(1.0), 0.6931471805599453);
/// assert_eq!(ln1p_vec(&[0.0, 1.0, E, 10.0]), &[0.0, 0.6931471805599453, 1.3132616875182228, 2.3978952727983707]);
/// ```
/// <small>End Fun Doc</small>
pub fn ln1p_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| ln1p(x)).collect()
}

/// ### log2_vec(x)
///
/// Logarithm Function
///
/// The `log2_vec` function applies the base-2 logarithm to each element of `x`,
/// returning a new vector containing the results of these computations.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{log2, log2_vec, is_nan_f64, E, INF_F64 as inf, LN10};
/// assert!(is_nan_f64(log2(-inf)));
/// assert_eq!(log2(0.0), -inf);
/// assert_eq!(log2(1.0), 0.0);
/// assert_eq!(log2(E), 1.4426950408889634);
/// assert_eq!(log2_vec(&[0.0, 1.0, E, LN10]), &[-inf, 0.0, 1.4426950408889634, 1.2032544726997219]);
/// ```
/// <small>End Fun Doc</small>
pub fn log2_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| log2(x)).collect()
}

/// ### log10_vec(x)
///
/// Logarithm Function
///
/// The `log10_vec` function applies the base-10 logarithm to each element of `x`,
/// returning a new vector containing the results of these computations.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{log10, log10_vec, is_nan_f64, E, INF_F64 as inf,};
/// assert!(is_nan_f64(log10(-inf)));
/// assert_eq!(log10(E), 0.4342944819032518);
/// assert_eq!(log10_vec(&[0.0, 1.5, 10.0]), &[-inf, 0.17609125905568124, 1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn log10_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| log10(x)).collect()
}

/// ### fix64_vec(x)
///
/// Fixation Function
///
/// The `fix64_vec` function converts a slice of `double precision floats` to their equivalent
/// `fixed-point values` with `the same bit width`, returning them as a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{fix64, fix64_vec};
/// // assert_eq!(fix64("abc"), 0.3); // error: expected `f64`, found `&str`
/// // assert_eq!(fix64("0.1"), 0.3); // error: expected `f64`, found `&str`
/// // assert_eq!(fix64(1), 0.3); // error: expected `f64`, found integer
/// assert_eq!(fix64(0.1 + 0.2), 0.3);
/// assert_eq!(fix64(0.30000000000000004), 0.3);
/// assert_eq!(fix64_vec(&[0.3, 0.30000000000000004, 0.1 + 0.2]), [0.3, 0.3, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn fix64_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| fix64(x)).collect()
}

/// ### cube_vec(x)
///
/// Native Function
///
/// The `cube_vec` function applies the cube operation elementwise to each
/// number in input vector x, returning a new vector containing the results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cube_vec, fix64_vec};
/// assert_eq!(cube_vec(&[0.0, 0.1, 0.2]), [0.0, 0.0010000000000000002, 0.008000000000000002]);
/// assert_eq!(fix64_vec(&cube_vec(&[0.0, 0.1, 0.2])), [0.0, 0.001, 0.008]);
/// ```
/// <small>End Fun Doc</small>
pub fn cube_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cube(x)).collect()
}

/// ### cbrt_vec(x)
///
/// Native Function
///
/// The `cbrt_vec` function maps the cbrt function over each element in the provided float slice,
/// collecting the resulting values into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cbrt_vec, fix64_vec};
/// assert_eq!(cbrt_vec(&[0.0, 0.001, 0.008]), [0.0, 0.1, 0.2]);
/// assert_eq!(cbrt_vec(&[8.0, 27.0]), [2.0, 3.0000000000000004]);
/// assert_eq!(fix64_vec(&cbrt_vec(&[8.0, 27.0])), [2.0, 3.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn cbrt_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cbrt(x)).collect()
}

/// ### trunc_vec(x)
///
/// Rounding Function
///
/// The `trunc_vec` function performs rounding downwards (i.e. Truncation) on every single-precision
/// floating-point number in the input slice, gathering the outcomes into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::trunc_vec;
/// assert_eq!(trunc_vec(&[-0.37, 0.37, -3.7, 3.7]), [0.0, 0.0, -3.0, 3.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn trunc_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| trunc(x)).collect()
}

/// ### sin_vec(x)
///
/// Trigonometric Function
///
/// The `sin_vec` function calculates the sine value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sin_vec, deg_to_rad_vec};
/// assert_eq!(sin_vec(&[0.0, 9.999e-15, 1e-14, 0.5235987755982988]), [0.0, 0.0, 1e-14, 0.5]);
/// ```
/// <small>End Fun Doc</small>
pub fn sin_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sin(x)).collect()
}

/// ### sin_deg_vec(x)
///
/// Trigonometric Function
///
/// The `sin_deg_vec` function calculates the sine value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::sin_deg_vec;
/// assert_eq!(sin_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [0.0, 0.5, 0.70710677, 0.8660254, 1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn sin_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sin_deg(x)).collect()
}

/// ### cos_vec(x)
///
/// Trigonometric Function
///
/// The `cos_vec` function calculates the cosine value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cos_vec, deg_to_rad_vec};
/// assert_eq!(cos_vec(&[0.0, 9.999e-15, 1e-14, 1.0471975511965976]), [1.0, 1.0, 1.0, 0.5]);
/// ```
/// <small>End Fun Doc</small>
pub fn cos_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cos(x)).collect()
}

/// ### cos_deg_vec(x)
///
/// Trigonometric Function
///
/// The `cos_deg_vec` function calculates the cosine value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::cos_deg_vec;
/// assert_eq!(cos_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [1.0, 0.8660254, 0.70710677, 0.5, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn cos_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cos_deg(x)).collect()
}

/// ### tan_vec(x)
///
/// Trigonometric Function
///
/// The `tan_vec` function calculates the tangent value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{tan_vec, deg_to_rad_vec};
/// assert_eq!(tan_vec(&[0.0, 9.999e-15, 1e-14, 0.7853981633974483]), [0.0, 0.0, 1e-14, 1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn tan_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| tan(x)).collect()
}

/// ### tan_deg_vec(x)
///
/// Trigonometric Function
///
/// The `tan_deg_vec` function calculates the tangent value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{tan_deg_vec, INF_F64 as inf};
/// assert_eq!(tan_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [0.0, 0.57735026, 1.0, 1.7320508, inf]);
/// ```
/// <small>End Fun Doc</small>
pub fn tan_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| tan_deg(x)).collect()
}

/// ### csc_vec(x)
///
/// Trigonometric Function
///
/// The `csc_vec` function calculates the cosecant value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{csc_vec, INF_F64 as inf};
/// assert_eq!(csc_vec(&[0.0, 9.999e-15, 1e-14, 0.5235987755982988]), [inf, inf, 1e14, 2.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn csc_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| csc(x)).collect()
}

/// ### csc_deg_vec(x)
///
/// Trigonometric Function
///
/// The `csc_deg_vec` function calculates the cosecant value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{csc_deg_vec, INF_F64 as inf};
/// assert_eq!(csc_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [inf, 2.0, 1.4142135, 1.1547005, 1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn csc_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| csc_deg(x)).collect()
}

/// ### sec_vec(x)
///
/// Trigonometric Function
///
/// The `sec_vec` function calculates the secant value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sec_vec, INF_F64 as inf};
/// assert_eq!(sec_vec(&[0.0, 9.999e-15, 1e-14, 1.0471975511965976]), [1.0, 1.0, 1.0, 2.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn sec_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sec(x)).collect()
}

/// ### sec_deg_vec(x)
///
/// Trigonometric Function
///
/// The `sec_deg_vec` function calculates the secant value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{sec_deg_vec, INF_F64 as inf};
/// assert_eq!(sec_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [1.0, 1.1547005, 1.4142135, 2.0, inf]);
/// ```
/// <small>End Fun Doc</small>
pub fn sec_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| sec_deg(x)).collect()
}

/// ### cot_vec(x)
///
/// Trigonometric Function
///
/// The `cot_vec` function calculates the cotangent value of each angle represented as a radian in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cot_vec, INF_F64 as inf};
/// assert_eq!(cot_vec(&[0.0, 9.999e-15, 1e-14, 1.0471975511965976]), [inf, inf, 1e14, 0.57735026]);
/// ```
/// <small>End Fun Doc</small>
pub fn cot_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cot(x)).collect()
}

/// ### cot_deg_vec(x)
///
/// Trigonometric Function
///
/// The `cot_deg_vec` function calculates the cotangent value of each angle represented in degrees in the input iterator,
/// constructing a new vector from these results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{cot_deg_vec, INF_F64 as inf};
/// assert_eq!(cot_deg_vec(&[0.0, 30.0, 45.0, 60.0, 90.0]), [inf, 1.7320508, 1.0, 0.57735026, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn cot_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| cot_deg(x)).collect()
}
