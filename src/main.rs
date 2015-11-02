extern crate combine;

use std::io::prelude::*;

use combine::{many, letter, Parser, ParseError};

fn parse(program: String) {
    let program = program.as_ref();

    let mut grammar = many(letter());
    let result: Result<(String, &str), ParseError<&str>> = grammar.parse(program);
    match result {
        Ok((value, _remaining_input)) => println!("{:?}", value),
        Err(err) => println!("{}", err)
    }
}

fn main() {
    println!("rusty-lisp v0.1.0");
    println!("Press 'Ctrl+C' to exit");
    println!("");

    let mut stdin = std::io::stdin();
    let mut input = String::new();

    stdin.read_to_string(&mut input).unwrap();
    parse(input);

    println!("Thanks for using!");
}
