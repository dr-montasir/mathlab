// fn main() {
//     let r = add(1.0, 2.5);
//     println!("{}", r);
// }

// fn add(x: f64, y: f64) -> f32 {
//     return (x + y) as f32;
// }

#[allow(dead_code)]
pub mod math {
    pub fn abs(x: f32) -> f32 {
        if x < 0.0 {
            return -x;
        } else {
            return x;
        }
    }

    pub fn add(x: f32, y: f32) -> f32 {
        return x + y;
    }
}
