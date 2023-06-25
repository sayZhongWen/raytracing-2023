// use std::f64::consts::PI;
use rand::Rng;
// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * PI / 180.0
// }
pub fn random_f64() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}
pub fn random(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
