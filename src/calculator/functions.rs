use crate::calculator::arithmetic;

pub fn abs(num: f64) -> f64 {
    num.abs()
}

pub fn square(num: f64) -> f64 {
    arithmetic::pow(num, 2)
}

pub fn cube(num: f64) -> f64 {
    arithmetic::pow(num, 3)
}

pub fn sqrt(num: f64) -> f64 {
    num.sqrt()
}

pub fn cbrt(num: f64) -> f64 {
    num.cbrt()
}

pub fn exp(num: f64) -> f64 {
    num.exp()
}

pub fn ln(num: f64) -> f64 {
    num.ln()
}

pub fn sin(num: f64) -> f64 {
    num.sin()
}

pub fn cos(num: f64) -> f64 {
    num.cos()
}

pub fn tan(num: f64) -> f64 {
    num.tan()
}

