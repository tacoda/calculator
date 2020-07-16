pub fn add(nums: &[i32]) {
    println!("{}", addition(nums));
}

pub fn addition(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, n| acc + n)
}

pub fn sub(first: usize, second: usize) {
    println!("{}", subtract(first, second));
}

fn subtract(first: usize, second: usize) -> usize {
    first - second
}

pub fn mult(nums: &[i32]) {
    println!("{}", multiply(nums));
}

fn multiply(nums: &[i32]) -> i32 {
    nums.iter().fold(1, |acc, n| acc * n)
}

pub fn div(dividend: usize, divisor: usize) {
    println!("{}", divide(dividend, divisor));
}

fn divide(dividend: usize, divisor: usize) -> usize {
    dividend / divisor
}

pub fn exp(base: usize, exponent: usize) {
    println!("{}", exponential(base, exponent));
}

fn exponential(base: usize, exponent: usize) -> usize {
    base.pow(exponent as u32)
}

pub fn prcnt(base: i32, percent: f64) {
    println!("{}", percentage(base as f64, percent));
}

fn percentage(base: f64, percent: f64) -> f64 {
    base * (percent / 100.0)
}
