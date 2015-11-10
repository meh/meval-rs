extern crate math_eval;

use std::env::args;
use math_eval::eval_str;

const USAGE: &'static str = r"Simple math expression evaluation.

Usage: eval EXPR1 EXPR2 ...";

fn main() {
    let args = args().skip(1);
    if args.len() == 0 {
        println!("{}", USAGE);
    }
    for arg in args.skip(1) {
        match eval_str(&arg) {
            Ok(f) => println!("{} = {}", arg, f),
            Err(e) => println!("Error when evaluating `{}`: {}", arg, e),
        }
    }
}
