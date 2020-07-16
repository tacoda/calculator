pub fn square(num: f64) {
    println!("{}", do_square(num));
}

fn do_square(num: f64) -> f64 {
    num * num
}

pub fn sqrt(num: f64) {
    println!("{}", do_sqrt(num));
}

fn do_sqrt(num: f64) -> f64 {
    num.sqrt()
}

pub fn abs(num: f64) {
    println!("{}", do_abs(num));
}

fn do_abs(num: f64) -> f64 {
    num.abs()
}
