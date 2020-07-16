pub fn add(first: usize, second: usize) {
    println!("{} + {} = {}", first, second, addition(first, second));
}

fn addition(first: usize, second: usize) -> usize {
    first + second
}
