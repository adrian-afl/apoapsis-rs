use std::ops::{Add, Mul};

pub fn mix<'a, T: 'a>(a: &'a T, b: &'a T, m: f64) -> T
where
    &'a T: Mul<f64, Output = T> + Add<&'a T, Output = T>,
    T: Add<T, Output = T>,
{
    let term_a = a * (1.0 - m);
    let term_b = b * m;
    term_a + term_b
}

pub fn map(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn usat(a: f64) -> f64 {
    a.min(1.0).max(0.0)
}
