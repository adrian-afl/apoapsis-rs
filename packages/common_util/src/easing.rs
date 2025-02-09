use std::f64::consts::PI;

// https://gitlab.com/chpio/simple-easing/-/tree/master?ref_type=heads
// at hash 85c788ba5741af708ec602ef7890da2ab2e4f6d6
// Borrowed as MIT

pub fn ease_sine_in(t: f64) -> f64 {
    1.0 - (t * PI / 2.0).cos()
}

pub fn ease_sine_out(t: f64) -> f64 {
    (t * PI / 2.0).sin()
}

pub fn ease_sine_in_out(t: f64) -> f64 {
    -((PI * t).cos() - 1.0) / 2.0
}

pub fn ease_cubic_in(t: f64) -> f64 {
    t * t * t
}

pub fn ease_cubic_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_cubic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_quad_in(t: f64) -> f64 {
    t * t
}

pub fn ease_quad_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(2)
}

pub fn ease_quad_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn ease_circ_in(t: f64) -> f64 {
    1.0 - (1.0 - t.powi(2)).sqrt()
}

pub fn ease_circ_out(t: f64) -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

pub fn ease_circ_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

pub fn ease_expo_in(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else {
        2f64.powf(10.0 * t - 10.0)
    }
}

pub fn ease_expo_out(t: f64) -> f64 {
    if 1.0 <= t {
        1.0
    } else {
        1.0 - 2f64.powf(-10.0 * t)
    }
}

pub fn ease_expo_in_out(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        2f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2f64.powf(-20.0 * t + 10.0)) / 2.0
    }
}
