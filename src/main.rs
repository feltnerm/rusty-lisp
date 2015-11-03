extern crate combine;

use std::io::prelude::*;

use combine::{spaces, string, try, choice, parser, Parser, ParserExt, ParseError};
use combine::char as combine_char;
use combine::primitives::{Stream, State, ParseResult};

#[derive(Debug, PartialEq)]
enum LispVal {
    Atom(String),
    Bool(bool)
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

fn atom<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char>
{
    let atom = (parser(symbol::<I>))
        .parse_state(input);

    atom
}

fn bool<I>(input: State<I>) -> ParseResult<bool, I>
where I: Stream<Item=char>
{
    let bools = try(string("#t").map(|_| return true)).or(
            string("#f").map(|_| return false)
        )
       .parse_state(input);

    bools
}

fn rusty_lisp<I>(input: State<I>) -> ParseResult<LispVal, I>
where I: Stream<Item=char>
{
    let lispval = parser(atom::<I>)
                   .map(LispVal::Atom)
        .or(parser(bool::<I>)
            .map(LispVal::Bool))
        .parse_state(input);

    lispval
}

fn parse(program: String) {
    let program = program.as_ref();

    let result: Result<(LispVal, &str), ParseError<&str>> = parser(rusty_lisp).parse(program);
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
