use rand::prelude::*;

pub static INFINITY: f64 = f64::MAX;

#[allow(dead_code)]
pub static PI: f64 = 3.1415926535897932385;

#[allow(dead_code)]
pub fn degrees_to_radians(degrees:f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_one() -> f64 {
    random_float(0.0, 1.0)
}

pub fn random_float(min:f64, max:f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}
