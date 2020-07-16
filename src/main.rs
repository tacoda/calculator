#[macro_use]
extern crate structopt;

mod calculator;


use structopt::StructOpt;
use crate::calculator as calc;

#[derive(StructOpt)]
#[structopt(name = "calculator", about = "A simple command line calculator")]
enum Calculator {
    #[structopt(name = "add")]
    Add {
        first: usize,
        second: usize,
    }
}

fn main() {
    let calculator = Calculator::from_args();

    match calculator {
        Calculator::Add { first, second } => { println!("{}", calc::add::add(first, second)) },
    }
}
