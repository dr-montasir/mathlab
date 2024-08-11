use super::num::{fix, fix64};

/// ### monolist(x, size)
///
/// Generating function
///
/// The `monolist` function generates a list of specified length containing only a given float value `x`.
/// The maximum allowed length is limited to 1 million to prevent excessive resource consumption.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{monolist, fix64};
/// assert_eq!(monolist(0.1, 0), []);
/// assert_eq!(monolist(0.1, 1000000000), monolist(0.1, 1000000000)); // if size > 1 million {  function will replace size with 0 } -> []
/// assert_eq!(monolist(-1.0, 2), [-1.0, -1.0]);
/// assert_eq!(monolist(0.0, 2), [0.0, 0.0]);
/// assert_eq!(monolist(1.0, 2), [1.0, 1.0]);
/// assert_eq!(monolist(0.1 + 0.2, 2), [0.3, 0.3]);
/// assert_eq!(monolist(0.30000000000000004, 2), [0.3, 0.3]);
/// ```
/// <small>End Fun Doc</small>
pub fn monolist(x: f64, mut size: usize) -> Vec<f64> {
    if size > 1_000_000 {
        size = 0
    }
    let mut vector = Vec::with_capacity(size);
    for _i in 0..size {
        vector.push(fix64(x))
    }
    vector
}

/// ### range(x, step, size, order)
///
/// Generating function
///
/// The `range` function generates a list of up to one million elements representing a sequence of float numbers
/// spaced by the provided step value starting at x, optionally sorted either ascending ("asc") or descending ("desc").
/// Returns an empty vector if size exceeds one million, or if step is nonpositive.
///
/// ### Examples
/// ```rust
/// use mathlab::math::range;
/// // For the order argument, use "asc" for ascending order or "desc" for descending order, otherwise the function will return [].
/// assert_eq!(range(0.0, 0.1, 10, "abcd"), []);
/// assert_eq!(range(0.0, 0.1, 0, "asc"), []); // The parameter size must be from 1 to 1 million.
/// assert_eq!(range(0.0, 0.1, 1000000000, "asc"), []); // The parameter size must be from 1 to 1 million.
/// assert_eq!(range(1.0, 1.0, 10, "asc"), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
/// assert_eq!(range(0.0, 0.1, 10, "asc"), [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
/// assert_eq!(range(0.0, 0.1, 10, "desc"), [0.0, -0.1, -0.2, -0.3, -0.4, -0.5, -0.6, -0.7, -0.8, -0.9]);
/// assert_eq!(range(0.9, 0.1, 10, "desc"), [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0]);
/// assert_eq!(range(0.0, 2.0, 3, "asc"), [0.0, 2.0, 4.0]);
/// assert_eq!(range(4.0, 2.0, 3, "desc"), [4.0, 2.0, 0.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn range(x: f64, step: f64, mut size: usize, order: &str) -> Vec<f64> {
    if size > 1_000_000 || step <= 0.0 {
        size = 0
    }
    let mut vector = Vec::with_capacity(size);
    for _i in 0..size {
        if order == "asc" {
            vector.push(fix64(x + (_i as f64 * step)))
        } else if order == "desc" {
            vector.push(fix64(x - (_i as f64 * step)))
        } else {
            return vector;
        }
    }
    vector
}

/// ### to_fixed(x, decimal_places)
///
/// Fixation Function
///
/// The `to_fixed` function converts a floating-point number `x` to a string with a specified
/// number of decimal places, returning a fixed-point string representation.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{to_fixed, fix, is_nan_f64, INF_F64 as inf, NAN_F64 as NaN};
/// assert_eq!(to_fixed(0.5235987755982988, 3), "0.524");
/// assert_eq!(to_fixed(0.5235987755982928, 15), "0.523598775598293");
/// assert_eq!(to_fixed(0.5235987755982928, 1), "0.5");
/// assert_eq!(to_fixed(0.5235987755982928, 0), "1");
/// assert_eq!(to_fixed(0.0, 0), "0");
/// assert_eq!(to_fixed(inf, 0), "inf");
/// assert_eq!(to_fixed(inf, 0), "inf");
/// assert_eq!(to_fixed(NaN, 0), "NaN");
/// assert!(is_nan_f64(fix(NaN, 0)));
/// assert_eq!(to_fixed(0.1 + 0.2, 15), "0.3");
/// ```
/// <small>End Fun Doc</small>
pub fn to_fixed(x: f64, decimal_places: u32) -> String {
    fix(x, decimal_places).to_string()
}
