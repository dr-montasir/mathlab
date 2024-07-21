use super::num::{add, divi, mult, pow, subt};

/// ### add_vec_num(x, y)
///
/// Operation Function
///
/// The `add_vec_num` function takes a slice `x` of floating-point numbers and a float `y`,
/// performs an element-wise addition on all elements of `x` and `y`, and returns a new vector containing the resulting sums.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{add, add_vec_num, fround_vec};
/// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f32, 0.3);
/// assert_eq!(add_vec_num(&[0.0, 0.1, 0.2], 0.1), [0.1, 0.2, 0.30000000000000004]);
/// assert_eq!(fround_vec(&add_vec_num(&[0.0, 0.1, 0.2], 0.1)), [0.1, 0.2, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn add_vec_num(x: &[f64], y: f64) -> Vec<f64> {
    x.iter().map(|&x| add(x, y)).collect()
}

/// ### subt_vec_num(x, y)
///
/// Operation Function
///
/// The `subt_vec_num` subtracts a given float `y` from every element in a slice `x` of floats,
/// returning a new vector containing the differences.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{subt, subt_vec_num, fround_vec};
/// assert_eq!(subt(0.1, 0.2), -0.1);
/// assert_eq!(subt(0.1, 0.2) as f32, -0.1);
/// assert_eq!(subt_vec_num(&[0.0, 0.1, 0.2, 0.3], 0.3), [-0.3, -0.19999999999999998, -0.09999999999999998, 0.0]);
/// assert_eq!(fround_vec(&subt_vec_num(&[0.0, 0.1, 0.2, 0.3], 0.3)), [-0.3, -0.2, -0.1, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn subt_vec_num(x: &[f64], y: f64) -> Vec<f64> {
    x.iter().map(|&x| subt(x, y)).collect()
}

/// ### mult_vec_num(x, y)
///
/// Operation Function
///
/// The `mult_vec_num` function multiplies a given float `y` with every element in a slice `x` of floats,
/// returning a new vector containing the result.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{mult, mult_vec_num, fround_vec};
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
/// assert_eq!(mult_vec_num(&[0.0, 0.1, 0.2], 0.1), [0.0, 0.010000000000000002, 0.020000000000000004]);
/// assert_eq!(fround_vec(&mult_vec_num(&[0.0, 0.1, 0.2], 0.1)), [0.0, 0.01, 0.02]);
/// ```
/// <small>End Fun Doc</small>
pub fn mult_vec_num(x: &[f64], y: f64) -> Vec<f64> {
    x.iter().map(|&x| mult(x, y)).collect()
}

/// ### divi_vec_num(x, y)
///
/// Operation Function
///
/// The `divi_vec_num` function divides every element in a slice `x` of floats by a given float `y`,
/// returning a new vector containing the result.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{divi, divi_vec_num, fround_vec};
/// assert_eq!(divi(0.2, 0.3), 0.6666666666666667);
/// assert_eq!(divi(0.2, 0.3) as f32, 0.6666667);
/// //assert_eq!(divi_vec_num(&[0.0, 0.1, 0.2, 0.3], 0.2), [0.0, 0.5, 1.0, 1.4999999999999998]);
/// //assert_eq!(fround_vec(divi_vec_num(&[0.0, 0.1, 0.2, 0.3], 0.2)), [0.0, 0.5, 1.0, 1.5]);
/// ```
/// <small>End Fun Doc</small>
pub fn divi_vec_num(x: &[f64], y: f64) -> Vec<f64> {
    x.iter().map(|&x| divi(x, y)).collect()
}

/// ### pow_vec_num(x, y)
///
/// Operation Function
///
/// The `pow_vec_num` function computes the power of every element in a slice `x` of positive floats using a
/// given float `y` as exponent, returning a new vector containing the results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{pow, pow_vec_num, INF_F64};
/// assert_eq!(pow(0.0, 1.0), 0.0);
/// assert_eq!(pow(2.0 , -3.0), 0.125);
/// assert_eq!(pow_vec_num(&[3.0, 0.0, 4.0, INF_F64], 2.0), [9.0, 0.0, 16.0, INF_F64]);
/// ```
/// <small>End Fun Doc</small>
pub fn pow_vec_num(x: &[f64], y: f64) -> Vec<f64> {
    x.iter().map(|&x| pow(x, y)).collect()
}
