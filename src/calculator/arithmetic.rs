pub fn add(nums: &[f64]) -> f64 {
    nums.iter().fold(0.0, |acc, n| acc + n)
}

pub fn sub(first: f64, second: f64) -> f64 {
    first - second
}

pub fn mult(nums: &[f64]) -> f64 {
    nums.iter().fold(1.0, |acc, n| acc * n)
}

pub fn div(dividend: f64, divisor: f64) -> f64 {
    dividend / divisor
}

pub fn pow(base: f64, exponent: i32) -> f64 {
    base.powi(exponent)
}

pub fn percent(base: f64, percent: f64) -> f64 {
    base * (percent / 100.0)
}
