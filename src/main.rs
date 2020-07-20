extern crate structopt;

mod calculator;

use structopt::StructOpt;
use crate::calculator as calc;

#[derive(StructOpt)]
#[structopt(name = "calculator", about = "A simple command line calculator")]
enum Calculator {
    #[structopt(name = "add")]
    /// Add a list of numbers
    Add {
        #[structopt(required = true, min_values = 2)]
        /// Space-delimited list of numbers to add
        nums: Vec<f64>,
    },
    #[structopt(name = "sub")]
    /// Subtract two numbers
    Sub {
        /// Number to subtract from
        first: f64,
        /// Number to subtract
        second: f64,
    },
    #[structopt(name = "mult")]
    /// Multiply a list of numbers
    Mult {
        #[structopt(required = true, min_values = 2)]
        /// Space-delimited list of numbers to multiply
        nums: Vec<f64>,
    },
    #[structopt(name = "div")]
    /// Divide two numbers
    Div {
        /// Dividend (value to divide into)
        dividend: f64,
        /// Divisor (value to divide by)
        divisor: f64,
    },
    #[structopt(name = "pow")]
    /// Find the power of a base and exponent
    Pow {
        /// Base of the power
        base: f64,
        /// Exponent of the power
        exponent: i32,
    },
    #[structopt(name = "percent")]
    /// Compute a percentage of a value
    Percent {
        /// Base to compute the percentage from
        base: f64,
        /// Percentage to calculate
        percent: f64,
    },
    #[structopt(name = "mean")]
    /// Find the mean of a list of numbers
    Mean {
        #[structopt(required = true, min_values = 2)]
        /// Space-delimited list of numbers to compute the mean
        nums: Vec<f64>,
    },
    #[structopt(name = "median")]
    /// Find the median of a list of numbers
    Median {
        #[structopt(required = true, min_values = 2)]
        /// Space-delimited list of numbers to compute the median
        nums: Vec<i32>,
    },
    #[structopt(name = "mode")]
    /// Find the mode of a list of numbers
    Mode {
        #[structopt(required = true, min_values = 2)]
        /// Space-delimited list of numbers to compute the mode
        nums: Vec<i32>,
    },
    #[structopt(name = "abs")]
    /// Find the absolute value of a number
    AbsoluteValue {
        /// Number to compute the absolute value
        num: f64,
    },
    #[structopt(name = "square")]
    /// Find the square of a number
    Square {
        /// Number to compute the square
        num: f64,
    },
    #[structopt(name = "sqrt")]
    /// Find the square root of a number
    SquareRoot {
        /// Number to compute the square root
        num: f64,
    },
    #[structopt(name = "cube")]
    /// Find the cube of a number
    Cube {
        /// Number to compute the cube
        num: f64,
    },
    #[structopt(name = "cbrt")]
    /// Find the cube root of a number
    CubeRoot {
        /// Number to compute the cube root
        num: f64,
    },
    #[structopt(name = "root")]
    /// Find the nth root of a number
    Root {
        /// Number to compute the nth root
        num: f64,
        /// Number to use as n to compute the root
        n: u32,
    },
    #[structopt(name = "exp")]
    /// Find the value of the exp function of a number
    Exp {
        /// Number to compute the exponential function
        num: f64,
    },
    #[structopt(name = "ln")]
    /// Find the natural logarithm of a number
    Ln {
        /// Number to compute the natural logarithm
        num: f64,
    },
    #[structopt(name = "log")]
    /// Find the base 10 logarithm of a number
    Log {
        /// Number to compute the base 10 logarithm
        num: f64,
    },
    #[structopt(name = "sin")]
    /// Find the sine of a number
    Sin {
        /// Number to compute the sine (in radians)
        num: f64,
    },
    #[structopt(name = "cos")]
    /// Find the cosine of a number
    Cos {
        /// Number to compute the cosine (in radians)
        num: f64,
    },
    #[structopt(name = "tan")]
    /// Find the tangent of a number
    Tan {
        /// Number to compute the tangent (in radians)
        num: f64,
    },
    #[structopt(name = "fact")]
    /// Find the factorial of a number
    Fact {
        /// Number to compute the factorial
        num: u32,
    },
}

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
        Calculator::Root { num, n } => { calc::functions::root(num, n) },
        Calculator::Exp { num } => { calc::functions::exp(num) },
        Calculator::Ln { num } => { calc::functions::ln(num) },
        Calculator::Log { num } => { calc::functions::log(num) },
        Calculator::Sin { num } => { calc::functions::sin(num) },
        Calculator::Cos { num } => { calc::functions::cos(num) },
        Calculator::Tan { num } => { calc::functions::tan(num) },
        Calculator::Fact { num } => { calc::functions::fact(num) },
    };

    println!("{}", result);
}
