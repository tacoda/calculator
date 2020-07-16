pub fn sub(first: usize, second: usize) {
    println!("{} - {} = {}", first, second, subtract(first, second));
}

fn subtract(first: usize, second: usize) -> usize {
    first - second
}
