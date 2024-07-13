/// _abs(x)
///
/// Native function
///
/// The abs function returns the absolute value of a number.
pub fn _abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// _sign(x)
///
/// Native function
///
/// The sign function returns only one of three possible values: −1, 0 or 1.
pub fn _sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// _add(x, y)
///
/// Native function
///
/// The add(x, y) function returns the sum of x and y.
pub fn _add(x: f64, y: f64) -> f64 {
    x + y
}

/// _subt(x, y)
///
/// Native function
///
/// The subt(x, y) function is a mathematical operation that subtracts the value of y from x.
pub fn _subt(x: f64, y: f64) -> f64 {
    x - y
}

/// _mult(x, y)
///
/// Native function
///
/// The mult(x, y) function is a mathematical operation that multiplies the value of x by y.
pub fn _mult(x: f64, y: f64) -> f64 {
    x * y
}

/// _divi(x, y)
///
/// Native function
///
/// The divi(x, y) function is a mathematical operation that divides the value of x by y.
pub fn _divi(x: f64, y: f64) -> f64 {
    x / y
}

/// _pow(x, y)
///
/// Native function
///
/// The pow function is a mathematical function that computes the power of a number.
pub fn _pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}
