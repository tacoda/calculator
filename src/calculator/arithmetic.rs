pub fn add(first: usize, second: usize) {
    println!("{} + {} = {}", first, second, addition(first, second));
}

fn addition(first: usize, second: usize) -> usize {
    first + second
}

pub fn sub(first: usize, second: usize) {
    println!("{} - {} = {}", first, second, subtract(first, second));
}

fn subtract(first: usize, second: usize) -> usize {
    first - second
}

pub fn mult(first: usize, second: usize) {
    println!("{} * {} = {}", first, second, multiply(first, second));
}

fn multiply(first: usize, second: usize) -> usize {
    first * second
}

pub fn div(first: usize, second: usize) {
    println!("{} / {} = {}", first, second, divide(first, second));
}

fn divide(first: usize, second: usize) -> usize {
    first / second
}
