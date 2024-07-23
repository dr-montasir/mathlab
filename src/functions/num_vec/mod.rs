use super::num::{add, divi, mult, pow, rem, subt};

/// ### add_num_vec(x, y)
///
/// Operation Function
///
/// The `add_num_vec` function adds a float `x` to each element in a slice `y` of floats,
/// returning a new vector containing the sums.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{add, add_num_vec, fround_vec};
/// assert_eq!(add(0.1, 0.2), 0.30000000000000004);
/// assert_eq!(add(0.1, 0.2) as f32, 0.3);
/// assert_eq!(add_num_vec(0.1, &[0.0, 0.1, 0.2]), [0.1, 0.2, 0.30000000000000004]);
/// assert_eq!(fround_vec(&add_num_vec(0.1, &[0.0, 0.1, 0.2])), [0.1, 0.2, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn add_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| add(x, y)).collect()
}

/// ### subt_num_vec(x, y)
///
/// Operation Function
///
/// The `subt_num_vec` function subtracts a float `x` from each element in a slice `y` of floats,
/// returning a new vector containing the differences.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{subt, subt_num_vec, fround_vec};
/// assert_eq!(subt(0.1, 0.2), -0.1);
/// assert_eq!(subt(0.1, 0.2) as f32, -0.1);
/// assert_eq!(subt_num_vec(0.1, &[0.0, 0.1, 0.2, 0.3]), [0.1, 0.0, -0.1, -0.19999999999999998]);
/// assert_eq!(fround_vec(&subt_num_vec(0.1, &[0.0, 0.1, 0.2, 0.3])), [0.1, 0.0, -0.1, -0.2]);
/// ```
/// <small>End Fun Doc</small>
pub fn subt_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| subt(x, y)).collect()
}

/// ### mult_num_vec(x, y)
///
/// Operation Function
///
/// The `mult_num_vec` multiplies a float `x` by each element in a slice `y` of floats,
/// returning a new vector containing the results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{mult, mult_num_vec, fround_vec};
/// assert_eq!(mult(0.1, 0.2), 0.020000000000000004);
/// assert_eq!(mult(0.1, 0.2) as f32, 0.02);
/// assert_eq!(mult_num_vec(0.1, &[0.0, 0.1, 0.2]), [0.0, 0.010000000000000002, 0.020000000000000004]);
/// assert_eq!(fround_vec(&mult_num_vec(0.1, &[0.0, 0.1, 0.2])), [0.0, 0.01, 0.02]);
/// ```
/// <small>End Fun Doc</small>
pub fn mult_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| mult(x, y)).collect()
}

/// ### divi_num_vec(x, y)
///
/// Operation Function
///
/// The `divi_num_vec` divides each element in a slice `y` of floats by a float `x`,
/// returning a new vector containing the results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{divi, divi_num_vec, fround_vec};
/// assert_eq!(divi(0.2, 0.3), 0.6666666666666667);
/// assert_eq!(divi(0.2, 0.3) as f32, 0.6666667);
/// //assert_eq!(divi_num_vec(0.2, &[0.0, 0.1, 0.2, 0.3]), [inf, 2.0, 1.0, 0.6666666666666666]);
/// //assert_eq!(fround_vec(&divi_num_vec(0.1, &[0.0, 0.1, 0.2, 0.3])), [inf, 2.0, 1.0, 0.6666667]);
/// ```
/// <small>End Fun Doc</small>
pub fn divi_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| divi(x, y)).collect()
}

/// ### pow_num_vec(x, y)
///
/// Operation Function
///
/// The `pow_num_vec` raises each element in a slice `y` of floats to the power of a float `x`,
/// returning a new vector containing the results.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{pow, pow_num_vec, INF_F64};
/// assert_eq!(pow(0.0, 1.0), 0.0);
/// assert_eq!(pow(2.0 , -3.0), 0.125);
/// assert_eq!(pow_num_vec(2.0, &[-3.0, 0.0, 4.0, INF_F64]), [0.125, 1.0, 16.0, INF_F64]);
/// ```
/// <small>End Fun Doc</small>
pub fn pow_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| pow(x, y)).collect()
}

/// ### rem_num_vec(x, y)
///
/// Operation Function
///
/// The `rem_num_vec` function computes the remainders obtained after dividing each element in `y` by another fixed number `x`, and accumulates these leftovers into a new vector.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rem, rem_num_vec, is_nan_f64, INF_F64};
/// assert!(is_nan_f64(rem(0.0, 0.0)));
/// assert!(is_nan_f64(rem(1.0, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 0.0)));
/// assert!(is_nan_f64(rem(INF_F64, 2.0)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert!(is_nan_f64(rem(INF_F64, INF_F64)));
/// assert_eq!(rem_num_vec(3.0, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]), [0.0, 1.0, 0.0, 3.0, 3.0, 3.0, 3.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn rem_num_vec(x: f64, y: &[f64]) -> Vec<f64> {
    y.iter().map(|&y| rem(x, y)).collect()
}
