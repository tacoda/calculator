pub fn mult(first: usize, second: usize) {
    println!("{} * {} = {}", first, second, multiply(first, second));
}

fn multiply(first: usize, second: usize) -> usize {
    first * second
}
