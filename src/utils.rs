use std::cmp::Ordering;

pub fn cmp_float(a: f64, b: f64) -> Ordering {
    const EPSILON: f64 = 0.1;

    if (a - b).abs() < EPSILON {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

