pub fn div(first: usize, second: usize) {
    println!("{} / {} = {}", first, second, divide(first, second));
}

fn divide(first: usize, second: usize) -> usize {
    first / second
}
