use super::num::{add, divi, mult, pow, rem, subt};

/// ### add_vec_vec(x, y)
///
/// Operation Function
///
/// The `add_vec_vec` function adds corresponding elements of vectors `x` and `y` using the add function,
/// then returns a new vector containing their sums.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{add, add_vec_vec, fround_vec};
/// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f32, 0.3);
/// assert_eq!(add_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0]), [0.3, 0.30000000000000004, 0.30000000000000004, 0.3]);
/// assert_eq!(fround_vec(&add_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0])), [0.3, 0.3, 0.3, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn add_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| add(x, y)).collect()
}

/// ### subt_vec_vec(x, y)
///
/// Operation Function
///
/// The `subt_vec_vec` function subtracts corresponding elements of vectors `x` and `y` using the subt function,
/// then returns a new vector containing their differences.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{subt, subt_vec_vec};
/// assert_eq!(subt(0.1, 0.2), -0.1);
/// assert_eq!(subt_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0]), [-0.3, -0.1, 0.1, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn subt_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| subt(x, y)).collect()
}

/// ### mult_vec_vec(x, y)
///
/// Operation Function
///
/// The `mult_vec_vec` function multiplies corresponding elements of vectors `x` and `y` using the mult function,
/// then returns a new vector containing their results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{mult, mult_vec_vec, fround_vec};
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
/// assert_eq!(mult_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0]), [0.0, 0.020000000000000004, 0.020000000000000004, 0.0]);
/// assert_eq!(fround_vec(&mult_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0])), [0.0, 0.02, 0.02, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn mult_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| mult(x, y)).collect()
}

/// ### divi_vec_vec(x, y)
///
/// Operation Function
///
/// The `divi_vec_vec` function divides corresponding elements of vectors `x` and `y` using the divi function,
/// then returns a new vector containing their results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{divi, divi_vec_vec, INF_F64};
/// assert_eq!(divi(0.1, 0.2), 0.5);
/// assert_eq!(divi_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0]), [0.0, 0.5, 2.0, INF_F64]);
/// ```
/// <small>End Fun Doc</small>
pub fn divi_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| divi(x, y)).collect()
}

/// ### pow_vec_vec(x, y)
///
/// Operation Function
///
/// The `pow_vec_vec` function computes the power of corresponding elements of vectors `x` and `y` using the pow function,
/// then returns a new vector containing their exponents.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{pow, pow_vec_vec};
/// assert_eq!(pow(0.1, 0.2), 0.6309573444801932);
/// assert_eq!(pow_vec_vec(&[0.0, 0.1, 0.2, 0.3], &[0.3, 0.2, 0.1, 0.0]), [0.0, 0.6309573444801932, 0.8513399225207846, 1.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn pow_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| pow(x, y)).collect()
}

/// ### rem_vec_vec(x, y)
///
/// Operation Function
///
/// The `rem_vec_vec` function takes in two slices of floating-point numbers `x` and `y` and generates
/// a new vector of remainders by applying the remainder operator rem to their corresponding elements.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rem, rem_vec_vec, is_nan_f64, INF_F64};
/// assert!(is_nan_f64(rem(0.0, 0.0)));
/// assert!(is_nan_f64(rem(1.0, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 2.0)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert_eq!(rem_vec_vec(&[1.0, 2.0, 3.0, 4.0], &[4.0, 3.0, 2.0, 1.0]), [1.0, 2.0, 1.0, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn rem_vec_vec(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(&x, &y)| rem(x, y)).collect()
}
