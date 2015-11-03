extern crate combine;

use std::io::prelude::*;

use combine::{spaces, choice, parser, Parser, ParserExt, ParseError};
use combine::char as combine_char;
use combine::primitives::{Stream, State, ParseResult};

#[derive(Debug, PartialEq)]
enum Atom {
    Value(String)
}

fn symbol<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char>
{

    let lex_char = |c| combine_char(c).skip(spaces());

    let symbols = choice([lex_char('!'), lex_char('$'), lex_char('%'), lex_char('$'), lex_char('&'),
                    lex_char('|'), lex_char('*'), lex_char('+'), lex_char('-'), lex_char(':'),
                    lex_char('<'), lex_char('='), lex_char('>'), lex_char('?'), lex_char('@'),
                    lex_char('^'), lex_char('_'), lex_char('~')]);

    symbols.map(|c: char| {
            let mut s = String::new();
            s.push(c);
            return s;
        })
        .parse_state(input)

}

fn atom<I>(input: State<I>) -> ParseResult<Atom, I>
where I: Stream<Item=char>
{

    let atom = (parser(symbol::<I>))
        .map(|t| Atom::Value(t))
        .parse_state(input);

    atom
}

fn rusty_lisp<I>(input: State<I>) -> ParseResult<Atom, I>
where I: Stream<Item=char>
{

    let lispval = parser(atom::<I>)
        .parse_state(input);

    lispval

}

fn parse(program: String) {
    let program = program.as_ref();

    let result: Result<(Atom, &str), ParseError<&str>> = parser(rusty_lisp).parse(program);
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
