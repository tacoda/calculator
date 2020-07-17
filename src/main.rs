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
    #[structopt(name = "pow")]
    /// Find the power of a base and exponent
    Pow {
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
    #[structopt(name = "cbrt")]
    /// Find the cube root of a number
    CubeRoot {
        /// Number to compute the cube root of
        num: f64,
    },
    #[structopt(name = "exp")]
    /// Find the value of the exp function of a number
    Exp {
        /// Number to compute the exp function of
        num: f64,
    },
    #[structopt(name = "ln")]
    /// Find the natural logarithm of a number
    Ln {
        /// Number to compute the natural logarithm of
        num: f64,
    },
    #[structopt(name = "sin")]
    /// Find the sine of a number
    Sin {
        /// Number to compute the sine of (in radians)
        num: f64,
    },
    #[structopt(name = "cos")]
    /// Find the cosine of a number
    Cos {
        /// Number to compute the cosine of (in radians)
        num: f64,
    },
    #[structopt(name = "tan")]
    /// Find the tangent of a number
    Tan {
        /// Number to compute the tangent of (in radians)
        num: f64,
    },
    #[structopt(name = "fact")]
    /// Find the factorial of a number
    Fact {
        /// Number to compute the factorial of
        num: u32,
    },
}

// Parens, AC, memory, swap sign, square, root, cube, cube root,
// nth power, nth root, e exponent, ln, base 10 exponent, log(10),
// factorial, sin, cos, tan, EE, rad/deg, pi, e, rand

fn main() {
    let cli = Calculator::from_args();

    let result = match cli {
        Calculator::Add { nums } => { calc::arithmetic::add(&nums) },
        Calculator::Sub { first, second } => { calc::arithmetic::sub(first, second) },
        Calculator::Mult { nums } => { calc::arithmetic::mult(&nums) },
        Calculator::Div { dividend, divisor } => { calc::arithmetic::div(dividend, divisor) },
        Calculator::Pow { base, exponent } => { calc::arithmetic::pow(base, exponent) },
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
        Calculator::CubeRoot { num } => { calc::functions::cbrt(num) },
        Calculator::Exp { num } => { calc::functions::exp(num) },
        Calculator::Ln { num } => { calc::functions::ln(num) },
        Calculator::Sin { num } => { calc::functions::sin(num) },
        Calculator::Cos { num } => { calc::functions::cos(num) },
        Calculator::Tan { num } => { calc::functions::tan(num) },
        Calculator::Fact { num } => { calc::functions::fact(num) },
    };

    println!("{}", result);
}
