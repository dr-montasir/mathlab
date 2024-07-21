/// ### monolist(x, size)
///
/// Generating function
///
/// The `monolist` function generates a list of specified length containing only a given float value `x`.
/// The maximum allowed length is limited to 1 million to prevent excessive resource consumption.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{monolist, f64_to_f32_vec, fround_vec};
/// assert_eq!(monolist(0.1, 0), []);
/// assert_eq!(monolist(-1.0, 2), [-1.0, -1.0]);
/// assert_eq!(monolist(0.0, 2), [0.0, 0.0]);
/// assert_eq!(monolist(1.0, 2), [1.0, 1.0]);
/// assert_eq!(monolist(0.30000000000000004, 2), [0.30000000000000004, 0.30000000000000004]);
/// assert_eq!(f64_to_f32_vec(&monolist(0.30000000000000004, 2)), [0.3, 0.3]);
/// assert_eq!(fround_vec(&monolist(0.30000000000000004, 2)), [0.3, 0.3]);
/// assert_eq!(monolist(0.1, 1000000000), monolist(0.1, 1000000000)); // if size > 1 million {  replace size with 1 million } -> [0.1, ... 0.1]
/// ```
/// <small>End Fun Doc</small>
pub fn monolist(x: f64, mut size: usize) -> Vec<f64> {
    if size > 1_000_000 {
        size = 1_000_000
    }
    let mut vector = Vec::with_capacity(size);
    for _i in 0..size {
        vector.push(x)
    }
    vector
}
