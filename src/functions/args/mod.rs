use super::num::{fix, fix64};

/// ### cross(a, b)
///
/// **Mathematical vector operation**
///
/// Calculates the cross product of two 3-dimensional vectors represented as slices of `f64`.
///
/// The cross product of two vectors results in a third vector that is perpendicular to both of the original vectors.
/// It is calculated as follows:
///
/// cross_product = (a_j * b_k - a_k * b_j,  // x-component
///                  a_k * b_i - a_i * b_k,  // y-component
///                  a_i * b_j - a_j * b_i)  // z-component
///
/// where:
/// - The first vector has components a_i, a_j, a_k
/// - The second vector has components b_i, b_j, b_k
///
/// This operation produces a new vector with components calculated from these formulas.
/// 
/// ### Panics
///
/// This function will panic if the slices `a` and `b` are not of length 3, due to the `assert_eq!` checks.
///
/// ### Usage
///
/// The developer must ensure that `a` and `b` are slices of length 3, representing vectors in 3D space.
/// The function returns a new vector that is the cross product of the two input vectors.
///
/// ### Example
///
/// ```rust
/// use mathlab::math::cross;
/// 
/// // First cross product
/// let a = vec![1.0, 0.0, 0.0]; // Represents vector a = 1*i + 0*j + 0*k
/// let b = vec![0.0, 1.0, 0.0]; // Represents vector b = 0*i + 1*j + 0*k
/// let a_cross_b = cross(&a, &b);
/// println!("{:?}", a_cross_b); // Outputs [0.0, 0.0, 1.0], which is i x j = k
/// 
/// // Second cross product
/// let c = vec![0.0, 1.0, 0.0]; // Represents vector c = 0*i + 1*j + 0*k
/// let d = vec![1.0, 0.0, 0.0]; // Represents vector d = 1*i + 0*j + 0*k
/// let c_cross_d = cross(&c, &d);
/// println!("{:?}", c_cross_d); // Outputs [0.0, 0.0, -1.0], which is j x i = -k
/// 
/// // Third cross product
/// let e = vec![0.0, 1.0, 2.0]; // Represents vector e = 0*i + 1*j + 2*k
/// let f = vec![3.0, 4.0, 5.0]; // Represents vector f = 3*i + 4*j + 5*k
/// let e_cross_f = cross(&e, &f);
/// println!("{:?}", e_cross_f); // Outputs [-3.0, 6.0, -3.0], which is e x f. 
///                              // Calculation: (1*5 - 2*4, 2*3 - 0*5, 0*4 - 1*3) = (-3, 6, -3)
/// ```
/// <small>End Function Documentation</small>
pub fn cross(a: &[f64], b: &[f64]) -> Vec<f64> {
    // Ensure both slices are of length 3
    assert_eq!(a.len(), 3, "First vector must be of length 3");
    assert_eq!(b.len(), 3, "Second vector must be of length 3");
    
    vec![
        a[1] * b[2] - a[2] * b[1], // j*k - k*j component (x component)
        a[2] * b[0] - a[0] * b[2], // k*i - i*k component (y component)
        a[0] * b[1] - a[1] * b[0], // i*j - j*i component (z component)
    ]
}

/// ### dot(a, b)
///
/// **Mathematical vector operation**
/// 
/// Calculates the dot product of two slices of `f64`.
///
/// ### Panics
///
/// This function will panic if the lengths of the slices `a` and `b` are not equal,
/// due to the `assert_eq!` check.
///
/// ### Usage
///
/// The developer is responsible for ensuring that `a` and `b` are of the same length before calling this function.
/// If there's a possibility of different lengths, handle that case externally or use an alternative approach.
///
/// ### Example
///
/// ```rust
/// use mathlab::math::dot;
/// let a = vec![0.0, 1.0, 2.0, -1.0];
/// let b = vec![3.0, 4.0, 5.0, 5.0];
/// let result = dot(&a, &b);
/// println!("{}", result); // Outputs 9.0
/// ```
/// /// <small>End Fun Doc</small>
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    // Ensure both slices are the same length
    assert_eq!(a.len(), b.len(), "Vectors must be the same length");
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

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
/// The `range` function generates a sequence of floating-point numbers starting from x, with a specified step value, 
/// producing up to size elements, ordered either ascending ("asc") or descending ("desc").
///
/// The function returns an empty vector if:
/// - size exceeds 1,000,000 (to prevent excessively large sequences)
/// - step is nonpositive
/// - order is not "asc" or "desc"
///
/// This function is useful when you need a sequence of a specific size, especially in cases where
/// the total range is not explicitly known or when you want to generate a fixed number of evenly spaced points.
///
/// ### Arguments
///
/// * x - The starting value of the sequence.
/// * step - The interval between consecutive numbers; must be positive.
/// * size - The number of elements to generate; capped at 1,000,000 for safety.
/// * order - A string slice indicating the sequence order: "asc" for ascending, "desc" for descending.
///
/// ### Returns
///
/// A vector containing the sequence of f64 values, ordered according to the order parameter.
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

/// ### range_from_to(from, to, step)
///
/// Generating function
/// 
/// The `range_from_to` function generates a sequence of f64 numbers from from to to with a specified step.
/// 
/// This function automatically determines the direction of the sequence (ascending or descending)
/// based on the input parameters and adjusts the step sign accordingly to produce a correct range.
/// 
/// # Arguments
/// 
/// * from - The starting value of the sequence.
/// * to - The ending value of the sequence.
/// * step - The increment/decrement between consecutive elements; must be positive. The function will adjust the sign based on direction.
/// 
/// # Returns
/// 
/// A vector containing the generated sequence of f64 values, starting from from and proceeding towards to,
/// inclusive of the endpoint if the range aligns exactly with the step increments.
///
/// # Notes
/// - If step is less than or equal to zero, the function returns an empty vector.
/// - The function handles both ascending and descending ranges.
/// - The sequence is generated with floating-point precision, and the values are processed through fix64 for formatting or rounding.
/// - The number of elements in the returned vector depends on the distance between from and to and the provided step.
/// 
/// # Examples
///
/// ```rust
/// use mathlab::math::range_from_to;
/// // Ascending range from 0.0 to 1.0 with step 0.2
/// assert_eq!(
///     range_from_to(0.0, 1.0, 0.2),
///     vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0]
/// );
///
/// // Descending range from 1.0 to 0.0 with step 0.2
/// assert_eq!(
///     range_from_to(1.0, 0.0, 0.2),
///     vec![1.0, 0.8, 0.6, 0.4, 0.2, 0.0]
/// );
///
/// // Range from -1.0 to 1.0 with step 0.5
/// assert_eq!(
///     range_from_to(-1.0, 1.0, 0.5),
///     vec![-1.0, -0.5, 0.0, 0.5, 1.0]
/// );
///
/// // When to equals from, should produce a single-element vector
/// assert_eq!(
///     range_from_to(2.0, 2.0, 0.1),
///     vec![2.0]
/// );
///
/// // Negative step should produce an empty vector
/// assert_eq!(
///     range_from_to(0.0, 1.0, -0.1),
///     Vec::<f64>::new()
/// );
/// 
/// assert_eq!(range_from_to(0.0, 1000000.0, 0.5), []); // The maximum size of the vector must not be more than 1 million.
/// ```
/// <small>End Fun Doc</small>
pub fn range_from_to(from: f64, to: f64, step: f64) -> Vec<f64> {
    if step <= 0.0 {
        return Vec::new();
    }

    // Determine the direction and adjust step sign accordingly
    let step_sign = if to > from { step.abs() } else { -step.abs() };

    // Calculate the number of steps
    let steps = ((to - from) / step_sign).abs().ceil() as usize;

    if steps > 1_000_000 {
        return Vec::new();
    }

    // Generate the sequence
    (0..=steps)
        .map(|i| fix64(from + (i as f64) * step_sign))
        .collect()
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

/// ### hypot(x)
///
/// Geometric Function
///
/// The `hypot` function calculates the Euclidean norm (also known as the magnitude or length) of a vector in n-dimensional space.
///
/// ### Examples
/// ```rust
/// use mathlab::math::{hypot, INF_F64 as inf, is_nan_f64 as is_nan};
/// assert_eq!(hypot(&[0.0]), 0.0);
/// assert_eq!(hypot(&[1.0]), 1.0);
/// assert_eq!(hypot(&[1.0 / 0.0]), inf);
/// assert!(is_nan(hypot(&[0.0 / 0.0])));
/// assert_eq!(hypot(&[4.0]), 4.0);
/// assert_eq!(hypot(&[3.0, 4.0]), 5.0);
/// assert_eq!(hypot(&[4.0, 2.0, 4.0]), 6.0);
/// assert_eq!(hypot(&[-3.0, -4.0]), 5.0);
/// assert_eq!(hypot(&[-4.0]), 4.0);
/// ```
/// <small>End Fun Doc</small>
pub fn hypot(x: &[f64]) -> f64 {
    if x.len() > 0 && x.iter().all(|i| i as *const f64 != std::ptr::null()) {
        x.iter().map(|i| i.powi(2)).sum::<f64>().sqrt()
    } else {
        std::f64::NAN
    }
}

/////////////////// RAND FUNCTION ///////////////////

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the current UTC time as a `SystemTime`.
///
/// This function uses the standard library function to get the current time.
fn utc_now() -> SystemTime {
    SystemTime::now()
}

/// Returns the duration since the UNIX epoch (January 1, 1970).
///
/// This function retrieves the current time and calculates how long it has been since
/// the UNIX epoch, returning it as a `Duration`.
fn duration_since() -> Duration {
    utc_now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

/// Returns the current time in nanoseconds (within the last second).
///
/// The result is the remainder of the nanoseconds since the epoch, modulo 1,000,000,
/// effectively returning nanoseconds as a value between 0 and 999,999.
fn get_nanos() -> u64 {
    let nanos = duration_since().as_nanos() % 1_000_000;
    nanos as u64
}

/// Returns the current time in milliseconds (within the last second).
///
/// The result is the remainder of the milliseconds since the epoch, modulo 1,000,
/// effectively returning milliseconds as a value between 0 and 999.
fn get_millis() -> u64 {
    let millis = duration_since().as_millis() % 1000;
    millis as u64
}

/// Returns the current second (within the last minute).
///
/// This function retrieves the current seconds since the UNIX epoch
/// and returns it as a value between 0 and 59.
fn get_second() -> u64 {
    let second = duration_since().as_secs() % 60;
    second
}

/// ### string_to_u64(s)
///
/// Converts a string into a `u64`.
///
/// This function attempts to parse the provided string `s` into a `u64`.
/// If successful, it returns `Ok(u64)`, otherwise an error of type `ParseIntError`.
///
/// ### Examples
/// ```
/// use mathlab::math::string_to_u64;
///
/// let result = string_to_u64("12345".to_string());
/// assert_eq!(result, Ok(12345));
/// ```
/// <small>End Fun Doc</small>
pub fn string_to_u64(s: String) -> Result<u64, std::num::ParseIntError> {
    s.parse::<u64>()
}

/// Formats a `u64` number to ensure it is a 10-digit representation.
///
/// If the number has more than 10 digits, it truncates the leftmost digits.
/// If it has fewer than 10 digits, it pads the number with leading zeros
/// to ensure it has exactly 10 digits. format_number_10d(123) ==> 0000000123
fn format_number_10d(num: u64) -> u64 {
    let number_str = num.to_string();
    let length = number_str.len();
    if length > 10 {
        let truncated_str = &number_str[length - 10..];
        return truncated_str.parse::<u64>().unwrap();
    } else {
        let padded_str = format!("{}{:0>10}", &number_str, "0".repeat(10 - length));
        return padded_str[..10].parse::<u64>().unwrap();
    }
}

/// Generates a formatted string combining nanos, millis, and seconds.
///
/// This function creates a string representation of the current time using nanoseconds,
/// milliseconds, and seconds, then formats it as a `u64` ensuring it is 10 digits.
pub fn formatted() -> String {
    let num = format!("{:?}{:?}{:?}", get_nanos(), get_millis(), get_second());
    format!("{}", format_number_10d(string_to_u64(num).expect("REASON")))
}

/// ### rand(size)
///
/// Generates a pseudo-random `u64` number within the specified digit size.
///
/// The `size` parameter determines the number of digits for the generated random number.
/// The function will cap the `size` at 19 if requested larger to ensure it fits within the
/// limits of `u64`, preventing overflow.
///
/// ### Parameters
/// - `size`: The desired number of digits for the random number. Must be greater than 0.
///
/// ### Returns
/// A `u64` random number with digit count as specified, capped at 19.
///
/// ### Examples
/// ```rust
/// use mathlab::math::rand;
///
/// fn main() {
///    // Generating and printing random numbers of different sizes
///    println!("Random number (size 1): {:?}", rand(1));
///    println!("Random number (size 2): {:?}", rand(2));
///    println!("Random number (size 3): {:?}", rand(3));
///    println!("Random number (size 6): {:?}", rand(6));
///    println!("Random number (size 15): {:?}", rand(15));
///    println!("Random number (size 19): {:?}", rand(19));
///    println!("Random number (size 19, requested 25): {:?}", rand(25)); // Size capped at 19
/// }
/// ```
/// <small>End Fun Doc</small>
pub fn rand(mut size: usize) -> u64 {
    if size == 0 {
        return 0; // Handle size 0 case
    }

    if size > 19 {
        size = 19; // Cap max size to 19 digits
    }

    // Calculate the minimum and maximum ranges
    let min = 10u64.pow((size - 1) as u32); // 10^(size - 1)
    let max = 10u64.pow(size as u32) - 1; // 10^size - 1

    // Generate a seed using system time
    let nanos = get_nanos();
    let millis = get_millis();
    let seconds = get_second();

    // Create a complex random seed using sine and different time-derived components
    let time_combined = (nanos as f64) + ((millis as f64) * 1e-3) + ((seconds as f64) * 1e-6);

    // Use sine function to create some variability
    let sine_value = (time_combined * std::f64::consts::PI * 0.1).sin() * 1e19; // Scale to 19-digit range

    // Combine the sine value into the final random seed
    let random_seed = (sine_value.abs() as u64) % (max - min + 1);

    // Calculate the final random number
    let random_number = min + random_seed;

    random_number
}

/////////////////// END RAND FUNCTION ///////////////////
