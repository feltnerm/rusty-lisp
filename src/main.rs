extern crate combine;

use std::io::prelude::*;

use combine::{between, many1, digit, letter, spaces, string, try, choice, parser, Parser, ParserExt, ParseError};
use combine::char as combine_char;
use combine::primitives::{Stream, State, ParseResult};

#[derive(Debug, PartialEq)]
enum LispVal {
    Atom(String),
    Bool(bool),
    Number(i32),
    Character(char),
    String(String)
}

fn parse_symbol<I>(input: State<I>) -> ParseResult<String, I>
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

fn parse_bool<I>(input: State<I>) -> ParseResult<bool, I>
where I: Stream<Item=char>
{
    let bewl = try(string("#t").map(|_| return true)).or(
            string("#f").map(|_| return false)
        )
       .parse_state(input);

    bewl
}

fn parse_atom<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char>
{
    let atom = (parser(parse_symbol::<I>))
        .parse_state(input);

    atom
}

fn parse_string<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char>
{
    let string = between(combine_char('"'), combine_char('"'), many1(letter()))
        .parse_state(input);
    string
}

fn parse_char<I>(input: State<I>) -> ParseResult<char, I>
where I: Stream<Item=char>
{
    between(combine_char('\''), combine_char('\''), letter())
        .parse_state(input)
}

fn parse_number<I>(input: State<I>) -> ParseResult<i32, I>
where I: Stream<Item=char>
{
    let number = many1(digit())
        .map(|string: String| string.parse::<i32>().unwrap())
        .parse_state(input);
    number
}

fn rusty_lisp<I>(input: State<I>) -> ParseResult<LispVal, I>
where I: Stream<Item=char>
{
    let lispval = parser(parse_atom::<I>)
                   .map(LispVal::Atom)
        .or(parser(parse_bool::<I>)
            .map(LispVal::Bool))
        .or(parser(parse_number::<I>)
            .map(LispVal::Number))
        .or(parser(parse_char::<I>)
            .map(LispVal::Character))
        .or(parser(parse_string::<I>)
            .map(LispVal::String))
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
