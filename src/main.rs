#[macro_use]
extern crate structopt;

mod calculator;


use structopt::StructOpt;
use crate::calculator as calc;

#[derive(StructOpt)]
#[structopt(name = "calculator", about = "A simple command line calculator")]
enum Calculator {
    #[structopt(name = "add")]
    /// Add two numbers
    Add {
        first: usize,
        second: usize,
    },
    #[structopt(name = "sub")]
    /// Subtract two numbers
    Sub {
        first: usize,
        second: usize,
    },
    #[structopt(name = "mult")]
    /// Multiply two numbers
    Mult {
        first: usize,
        second: usize,
    }
}

fn main() {
    let calculator = Calculator::from_args();

    match calculator {
        Calculator::Add { first, second } => { println!("{} + {} = {}", first, second, calc::add::add(first, second)) },
        Calculator::Sub { first, second } => { println!("{} - {} = {}", first, second, calc::sub::sub(first, second)) },
        Calculator::Mult { first, second } => { println!("{} * {} = {}", first, second, calc::mult::mult(first, second)) },
    }
}
