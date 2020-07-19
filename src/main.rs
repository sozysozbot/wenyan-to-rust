#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::prelude::*;
mod lex;
#[derive(Debug)]
enum Statement {
    Declare(DeclareStatement),
    Print,
    // Define,
    // For,
    // Function,
    // If,
    // Return,
    // Math,
    // Assign,
    // Import,
    // Object,
    // Reference,
    // Array,
    // Flush,
    // Break,
    // Comment,
}

#[derive(Debug)]
struct DeclareStatement {
    int_num: i64,
    type_: lex::Type,
    data_arr: Vec<Data>,
}

#[derive(Debug)]
enum Data {
    StringLiteral(String),
    BoolValue(bool),
    Identifier(String),
    IntNum(i64),
    // FloatNum(f64),
}

#[derive(Debug)]
enum ParseError {
    UnresolvableTokens,
    UnexpectedEOF,
}

fn interpret_intnum(num: &lex::IntNum) -> i64 {
    let lex::IntNum(v) = num;
    match v.as_slice() {
        &[lex::IntNumKeywords::Ling2] => 0,
        &[lex::IntNumKeywords::Yi1] => 1,
        &[lex::IntNumKeywords::Er4] => 2,
        &[lex::IntNumKeywords::San1] => 3,
        &[lex::IntNumKeywords::Si4] => 4,
        &[lex::IntNumKeywords::Wu3] => 5,
        &[lex::IntNumKeywords::Liu4] => 6,
        &[lex::IntNumKeywords::Qi1] => 7,
        &[lex::IntNumKeywords::Ba1] => 8,
        &[lex::IntNumKeywords::Jiu3] => 9,
        _ => unimplemented!(),
    }
}

fn parse_data(
    iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Data, ParseError> {
    let token = match iter.next() {
        None => return Err(ParseError::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::StringLiteral(strlit) => Ok(Data::StringLiteral(strlit.to_string())),
        lex::Lex::BoolValue(bv) => Ok(Data::BoolValue(bv.interpret())),
        lex::Lex::Identifier(ident) => Ok(Data::Identifier(ident.to_string())),
        lex::Lex::IntNum(intnum) => Ok(Data::IntNum(interpret_intnum(intnum))), /* FIXME: must handle float */
        _ => unimplemented!(),
    }
}

fn parse_statement(
    mut iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Statement, ParseError> {
    let token = match iter.next() {
        None => return Err(ParseError::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::Shu1Zhi1 => return Ok(Statement::Print),
        lex::Lex::Jin1You3 | lex::Lex::Wu2You3 => {
            let next = iter.next();
            match next {
                None => return Err(ParseError::UnexpectedEOF),
                Some(lex::Lex::IntNum(num)) => {
                    match iter.next() {
                        None => return Err(ParseError::UnexpectedEOF),
                        Some(lex::Lex::Type(t)) => {
                            let mut ans = vec![];
                            let vec = loop {
                                if iter.peek() != Some(&&lex::Lex::Yue1) {
                                    break ans;
                                }
                                iter.next();
                                let data = parse_data(&mut iter)?;
                                ans.push(data);
                            };

                            return Ok(Statement::Declare(DeclareStatement {
                                int_num: interpret_intnum(num),
                                type_: *t,
                                data_arr: vec,
                            }));
                        }
                        _ => unimplemented!(), // 術, 物
                    }
                }
                _ => return Err(ParseError::UnresolvableTokens),
            }
        }
        _ => unimplemented!(),
    }
}

use peek_nth::IteratorExt;
fn parse(lex: &[lex::Lex]) -> Result<Vec<Statement>, ParseError> {
    let mut iter = lex.iter().peekable_nth();

    let mut ans = vec![];
    loop {
        if iter.peek().is_none() {
            return Ok(ans);
        }

        ans.push(parse_statement(&mut iter)?);
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test000.wy")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("src: \n----------------------");
    println!("{}", contents);
    println!("----------------------");

    let lex = lex::lex(&contents);
    println!("\nlexer output: \n----------------------");
    println!("{:?}", lex.clone());
    println!("----------------------");
    if let Ok(lex) = lex {
        let res = parse(&lex);
        println!("\nparser output: \n----------------------");
        println!("{:?}", res);
        println!("----------------------");
    }
    Ok(())
}
