use std::f64::consts::PI;

pub fn to_radians(angle: f64) -> f64 {
    angle * PI / 180.
}

pub fn cilinder_area(radius: f64) -> f64 {
    PI * radius.powi(2)
}
