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
