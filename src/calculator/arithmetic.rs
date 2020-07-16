pub fn add(nums: Vec<usize>) {
    println!("{}", addition(nums));
}

fn addition(nums: Vec<usize>) -> usize {
    nums.iter().fold(0, |acc, n| acc + n)
}

pub fn sub(first: usize, second: usize) {
    println!("{}", subtract(first, second));
}

fn subtract(first: usize, second: usize) -> usize {
    first - second
}

pub fn mult(nums: Vec<usize>) {
    println!("{}", multiply(nums));
}

fn multiply(nums: Vec<usize>) -> usize {
    nums.iter().fold(0, |acc, n| acc * n)
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
