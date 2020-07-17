extern crate structopt;

mod calculator;

use structopt::StructOpt;
use crate::calculator as calc;

#[derive(StructOpt)]
#[structopt(name = "calculator", about = "A simple command line calculator")]
enum Calculator {
    #[structopt(name = "add")]
    /// Add a list of at least two numbers
    Add {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to add
        nums: Vec<f64>,
    },
    #[structopt(name = "sub")]
    /// Subtract two numbers
    Sub {
        first: f64,
        second: f64,
    },
    #[structopt(name = "mult")]
    /// Multiply a list of at least two numbers
    Mult {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to multiply
        nums: Vec<f64>,
    },
    #[structopt(name = "div")]
    /// Divide two numbers
    Div {
        dividend: f64,
        divisor: f64,
    },
    #[structopt(name = "exp")]
    /// Find the power of a base and exponent
    Exp {
        base: f64,
        exponent: i32,
    },
    #[structopt(name = "percent")]
    /// Compute a percentage of a value
    Percent {
        base: f64,
        percent: f64,
    },
    #[structopt(name = "mean")]
    /// Find the mean of a list of numbers
    Mean {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to compute the mean of
        nums: Vec<f64>,
    },
    #[structopt(name = "median")]
    /// Find the median of a list of numbers
    Median {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to compute the median of
        nums: Vec<i32>,
    },
    #[structopt(name = "mode")]
    /// Find the mode of a list of numbers
    Mode {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to compute the mode of
        nums: Vec<i32>,
    },
    #[structopt(name = "abs")]
    /// Find the absolute value of a number
    AbsoluteValue {
        /// Number to compute the square of
        num: f64,
    },
    #[structopt(name = "square")]
    /// Find the square of a number
    Square {
        /// Number to compute the square of
        num: f64,
    },
    #[structopt(name = "sqrt")]
    /// Find the square root of a number
    SquareRoot {
        /// Number to compute the square root of
        num: f64,
    },
    #[structopt(name = "cube")]
    /// Find the cube of a number
    Cube {
        /// Number to compute the cube of
        num: f64,
    },
}

// Parens, AC, memory, swap sign, square, root, cube, cube root,
// nth power, nth root, e exponent, ln, base 10 exponent, log(10),
// factorial, sin, cos, tan, EE, rad/deg, pi, e, rand

fn main() {
    let command = Calculator::from_args();

    let result = match command {
        Calculator::Add { nums } => { calc::arithmetic::add(&nums) },
        Calculator::Sub { first, second } => { calc::arithmetic::sub(first, second) },
        Calculator::Mult { nums } => { calc::arithmetic::mult(&nums) },
        Calculator::Div { dividend, divisor } => { calc::arithmetic::div(dividend, divisor) },
        Calculator::Exp { base, exponent } => { calc::arithmetic::exp(base, exponent) },
        Calculator::Percent { base, percent } => { calc::arithmetic::percent(base, percent) },
        Calculator::Mean { nums } => { calc::statistics::mean(&nums) },
        Calculator::Median { mut nums } => {
            nums.sort();
            calc::statistics::median(&nums)
        },
        Calculator::Mode { nums } => { calc::statistics::mode(&nums) },
        Calculator::AbsoluteValue { num } => { calc::functions::abs(num) },
        Calculator::Square { num } => { calc::functions::square(num) },
        Calculator::SquareRoot { num } => { calc::functions::sqrt(num) },
        Calculator::Cube { num } => { calc::functions::cube(num) },
    };

    println!("{}", result);
}
