pub fn _abs(x: f64) -> f64 {
    if x < 0.0 {
        return -x;
    } else {
        return x;
    }
}

pub fn _add(x: f64, y: f64) -> f64 {
    return x + y;
}

pub fn _pow(x: f64, y: f64) -> f64 {
    return x.powf(y);
}
