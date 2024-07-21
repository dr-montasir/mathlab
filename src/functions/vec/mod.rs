use super::num::{
    abs, ceil, deg_to_rad, f64_to_f32, fact, floor, fround, gamma, i64_to_f64, rad_to_deg, round,
    sign, u64_to_f64,
};

/// ### abs_vec(x)
///
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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

/// ### floor_vec(x)
///
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Array Function with One Parameter
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
/// Appending a Float Value to a Vector of Floats
///
/// The `rad_to_deg_vec` function takes a slice of `f64` values
/// representing `angles` in `radians` and returns a `Vec<f64>`
/// containing the corresponding `angles` in `degrees`.
/// It uses the `rad_to_deg` function to convert each `angle` from `radians` to `degrees`.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{rad_to_deg, rad_to_deg_vec, f64_to_f32_vec, fround_vec};
/// let my_x_f64_array = [0.0, 0.017453292519943295, 0.5235987755982988, 0.7853981633974483, 1.0471975511965976, 1.5707963267948966, 3.141592653589793, 6.283185307179586, -6.283185307179586];
/// assert_eq!(rad_to_deg(0.5235987755982988), 29.999999999999996);
/// assert_eq!(rad_to_deg(0.5235987755982988) as f32, 30.0);
/// assert_eq!(rad_to_deg_vec(&my_x_f64_array), [0.0, 1.0, 29.999999999999996, 45.0, 59.99999999999999, 90.0, 180.0, 360.0, -360.0]);
/// assert_eq!(f64_to_f32_vec(&rad_to_deg_vec(&my_x_f64_array)), [0.0, 1.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0, -360.0]);
/// assert_eq!(fround_vec(&rad_to_deg_vec(&my_x_f64_array)), [0.0, 1.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0, -360.0]);
/// ```
/// <small>End Fun Doc</small>
pub fn rad_to_deg_vec(x: &[f64]) -> Vec<f64> {
    x.iter().map(|&x| rad_to_deg(x)).collect()
}
