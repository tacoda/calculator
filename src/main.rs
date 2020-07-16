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
        nums: Vec<i32>,
    },
    #[structopt(name = "sub")]
    /// Subtract two numbers
    Sub {
        first: usize,
        second: usize,
    },
    #[structopt(name = "mult")]
    /// Multiply a list of at least two numbers
    Mult {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to multiply
        nums: Vec<i32>,
    },
    #[structopt(name = "div")]
    /// Divide two numbers
    Div {
        dividend: usize,
        divisor: usize,
    },
    #[structopt(name = "exp")]
    /// Find the power of a base and exponent
    Exp {
        base: usize,
        exponent: usize,
    },
    #[structopt(name = "percent")]
    /// Compute a percentage of a value
    Percent {
        base: i32,
        percent: f64,
    },
    #[structopt(name = "mean")]
    /// Find the mean of a list of numbers
    Mean {
        #[structopt(required = true, min_values = 2)]
        /// Numbers to compute the mean of
        nums: Vec<i32>,
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
    }
}

fn main() {
    let calculator = Calculator::from_args();

    match calculator {
        Calculator::Add { nums } => { calc::arithmetic::add(&nums) },
        Calculator::Sub { first, second } => { calc::arithmetic::sub(first, second) },
        Calculator::Mult { nums } => { calc::arithmetic::mult(&nums) },
        Calculator::Div { dividend, divisor } => { calc::arithmetic::div(dividend, divisor) },
        Calculator::Percent { base, percent } => { calc::arithmetic::prcnt(base, percent) },
        Calculator::Exp { base, exponent } => { calc::arithmetic::exp(base, exponent) },
        Calculator::Mean { nums } => { calc::statistics::mean(&nums) },
        Calculator::Median { mut nums } => {
            nums.sort();
            calc::statistics::median(&nums)
        },
        Calculator::Mode { nums } => { calc::statistics::mode(&nums) },
    }
}
