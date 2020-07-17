use crate::calculator::arithmetic;

pub fn square(num: f64) -> f64 {
    arithmetic::exp(num, 2)
}

pub fn cube(num: f64) -> f64 {
    arithmetic::exp(num, 3)
}

pub fn sqrt(num: f64) -> f64 {
    num.sqrt()
}

pub fn abs(num: f64) -> f64 {
    num.abs()
}
