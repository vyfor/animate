pub fn linear(t: f64) -> f64 {
    t
}

pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

pub fn ease_out_quad(t: f64) -> f64 {
    t * (2.0 - t)
}

pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

pub fn ease_out_cubic(t: f64) -> f64 {
    let t = t - 1.0;
    t * t * t + 1.0
}

pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = t - 1.0;
        1.0 + 4.0 * t * t * t
    }
}
