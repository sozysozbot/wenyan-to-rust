use crate::lex;
#[derive(Debug)]
pub enum Statement {
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
pub struct DeclareStatement {
    pub int_num: usize,
    pub type_: lex::Type,
    pub data_arr: Vec<Data>,
}

#[derive(Debug, Clone)]
pub enum Data {
    StringLiteral(String),
    BoolValue(bool),
    Identifier(String),
    IntNum(i64),
    // FloatNum(f64),
}

#[derive(Debug)]
pub enum Error {
    UnresolvableTokens,
    UnexpectedEOF,
    InvalidVariableCount,
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
) -> Result<Data, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
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
) -> Result<Statement, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::Shu1Zhi1 => return Ok(Statement::Print),
        lex::Lex::Jin1You3 | lex::Lex::Wu2You3 => {
            let next = iter.next();
            match next {
                None => return Err(Error::UnexpectedEOF),
                Some(lex::Lex::IntNum(num)) => {
                    match iter.next() {
                        None => return Err(Error::UnexpectedEOF),
                        Some(lex::Lex::Type(t)) => {
                            use std::convert::TryFrom;

                            let mut ans = vec![];
                            let vec = loop {
                                if iter.peek() != Some(&&lex::Lex::Yue1) {
                                    break ans;
                                }
                                iter.next();
                                let data = parse_data(&mut iter)?;
                                ans.push(data);
                            };

                            let interpret = match usize::try_from(interpret_intnum(num)) {
                                Err(_) => return Err(Error::InvalidVariableCount),
                                Ok(a) => a
                            };

                            if interpret == 0 {
                                return Err(Error::InvalidVariableCount);
                            }

                            return Ok(Statement::Declare(DeclareStatement {
                                int_num: interpret as usize,
                                type_: *t,
                                data_arr: vec,
                            }));
                        }
                        _ => unimplemented!(), // 術, 物
                    }
                }
                _ => return Err(Error::UnresolvableTokens),
            }
        }
        _ => unimplemented!(),
    }
}

use peek_nth::IteratorExt;
pub fn parse(lex: &[lex::Lex]) -> Result<Vec<Statement>, Error> {
    let mut iter = lex.iter().peekable_nth();

    let mut ans = vec![];
    loop {
        if iter.peek().is_none() {
            return Ok(ans);
        }

        ans.push(parse_statement(&mut iter)?);
    }
}
