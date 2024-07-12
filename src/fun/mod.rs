/// _abs(x)
///
/// Native function
///
/// The abs function returns the absolute value of a number.
pub fn _abs(x: f64) -> f64 {
    if x < 0.0 {
        return -x;
    } else {
        return x;
    }
}

/// _add(x, y)
///
/// Native function
///
/// The add(x, y) function returns the sum of x and y.
pub fn _add(x: f64, y: f64) -> f64 {
    return x + y;
}

/// _subt(x, y)
///
/// Native function
///
/// The subt(x, y) function is a mathematical operation that subtracts the value of y from x.
pub fn _subt(x: f64, y: f64) -> f64 {
    return x - y;
}

/// _mult(x, y)
///
/// Native function
///
/// The mult(x, y) function is a mathematical operation that multiplies the value of x by y.
pub fn _mult(x: f64, y: f64) -> f64 {
    return x * y;
}

/// _divi(x, y)
///
/// Native function
///
/// The divi(x, y) function is a mathematical operation that divides the value of x by y.
pub fn _divi(x: f64, y: f64) -> f64 {
    return x / y;
}

/// _pow(x, y)
///
/// Native function
///
/// The pow function is a mathematical function that computes the power of a number.
pub fn _pow(x: f64, y: f64) -> f64 {
    return x.powf(y);
}
