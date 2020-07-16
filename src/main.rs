#[macro_use]
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
        nums: Vec<usize>,
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
        nums: Vec<usize>,
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
    }
}

fn main() {
    let calculator = Calculator::from_args();

    match calculator {
        Calculator::Add { nums } => { calc::arithmetic::add(nums) },
        Calculator::Sub { first, second } => { calc::arithmetic::sub(first, second) },
        Calculator::Mult { nums } => { calc::arithmetic::mult(nums) },
        Calculator::Div { dividend, divisor } => { calc::arithmetic::div(dividend, divisor) },
        Calculator::Exp { base, exponent } => { calc::arithmetic::exp(base, exponent) },
    }
}
