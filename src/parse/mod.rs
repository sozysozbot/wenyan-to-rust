use crate::lex;
#[derive(Debug)]
pub enum Statement {
    Declare(DeclareStatement),
    Print,
    ForEnum {
        num: i64,
        statements: Vec<Statement>,
    },
    ForEnumIdent {
        ident: Identifier,
        statements: Vec<Statement>,
    },
    InitDefine {
        type_: lex::Type,
        data: Data,
        name: Identifier,
    },
    Define {
        decl: DeclareStatement,
        idents: Vec<Identifier>,
    },
    // Function,
    // If,
    // Return,
    // Math,
    Assign {
        ident: Identifier,
        data: Data,
    },
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
    pub how_many_variables: usize,
    pub type_: lex::Type,
    pub data_arr: Vec<Data>,
}

#[derive(Debug, Clone)]
pub enum Data {
    StringLiteral(String),
    BoolValue(bool),
    Identifier(Identifier),
    IntNum(i64),
    // FloatNum(f64),
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Error {
    SomethingWentWrong,
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
        &[lex::IntNumKeywords::Shi2] => 10,
        &[lex::IntNumKeywords::Qian1] => 1000,
        _ => unimplemented!("parsing integer"),
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
        lex::Lex::Identifier(ident) => Ok(Data::Identifier(Identifier(ident.to_string()))),
        lex::Lex::IntNum(intnum) => Ok(Data::IntNum(interpret_intnum(intnum))), /* FIXME: must handle float */
        _ => unimplemented!(),
    }
}

fn parse_init_define_statement_after_you3(
    mut iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Statement, Error> {
    if let lex::Lex::Type(t) = iter.next().ok_or(Error::UnexpectedEOF)? {
        let data = parse_data(&mut iter)?;
        // According to https://wy-lang.org/spec.html#init_define_statement
        //  '有' TYPE data (name_single_statement)?
        // and thus name_single_statement seems optional.
        // However,
        //
        // ```
        // 有數一。
        // ```
        //
        // fails to compile and gives the message "TypeError: a.names is undefined".
        // Hence for now I will assume the name_single_statement part obligatory, unless I find any counterexamples.

        let next = iter.next();
        match next {
            None => panic!("If this message is obtained by a wenyan program that successfully compiles in the original edition, please submit an issue."),
            Some(lex::Lex::Ming2Zhi1) => {
                let next = iter.next();
                match next.ok_or(Error::UnexpectedEOF)? {
                    lex::Lex::Yue1 => {
                        let next = iter.next();
                        match next.ok_or(Error::UnexpectedEOF)? {
                            lex::Lex::Identifier(ident) => {
                                return Ok(Statement::InitDefine {
                                    type_: *t,
                                    name: Identifier(ident.to_string()),
                                    data
                                })
                            }
                            _ => return Err(Error::SomethingWentWrong),
                        }
                    }
                    _ => return Err(Error::SomethingWentWrong),
                }
            }
            _ => panic!("If this message is obtained by a wenyan program that successfully compiles in the original edition, please submit an issue."),
        }
    } else {
        return Err(Error::SomethingWentWrong);
    }
}

fn parse_for_enum_statement_after_wei2shi4(
    mut iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Statement, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::IntNum(num) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4 => {
                let mut inner = vec![];
                loop {
                    if iter.peek() == Some(&&lex::Lex::Yun2Yun2) {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                return Ok(Statement::ForEnum {
                    num: interpret_intnum(num),
                    statements: inner,
                });
            }
            _ => return Err(Error::SomethingWentWrong),
        },
        lex::Lex::Identifier(ident) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4 => {
                let mut inner = vec![];
                loop {
                    if iter.peek() == Some(&&lex::Lex::Yun2Yun2) {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                return Ok(Statement::ForEnumIdent {
                    ident: Identifier(ident.to_string()),
                    statements: inner,
                });
            }
            _ => return Err(Error::SomethingWentWrong),
        },
        _ => return Err(Error::SomethingWentWrong),
    }
}

fn parse_assign_after_xi1zhi1(
    mut iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Statement, Error> {
    // '昔之' IDENTIFIER
    // (
    //     '之' (INT_NUM|STRING_LITERAL|IDENTIFIER)
    // )? '者'
    // (
    //     (
    //         '今'
    //          (
    //              (data ('之' INT_NUM)?)|'其'
    //          ) '是矣'
    //     ) |
    //     '今不復存矣'
    // ) ;

    if let lex::Lex::Identifier(ident) = iter.next().ok_or(Error::UnexpectedEOF)? {
        match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Zhi1 => {
                unimplemented!("昔之 IDENTIFIER 之 (INT_NUM|STRING_LITERAL|IDENTIFIER)")
            }
            lex::Lex::Zhe3 => match iter.next().ok_or(Error::UnexpectedEOF)? {
                lex::Lex::Jin1Bu4Fu4Cun2Yi3 => unimplemented!("昔之 ... 者今不復存矣"),
                lex::Lex::Jin1 => {
                    if let lex::Lex::Qi2 = iter.peek().ok_or(Error::UnexpectedEOF)? {
                        unimplemented!("昔之 ... 者今其是矣")
                    } else {
                        let data = parse_data(&mut iter)?;
                        match iter.next().ok_or(Error::UnexpectedEOF)? {
                            lex::Lex::Zhi1 => unimplemented!("昔之 ... 者今data之INT_NUM是矣"),
                            lex::Lex::Shi4Yi3 => {
                                return Ok(Statement::Assign {
                                    ident: Identifier(ident.clone()),
                                    data,
                                })
                            }
                            _ => return Err(Error::SomethingWentWrong),
                        }
                    }
                }
                _ => return Err(Error::SomethingWentWrong),
            },
            _ => return Err(Error::SomethingWentWrong),
        }
    } else {
        return Err(Error::SomethingWentWrong);
    }
}

fn parse_statement(
    mut iter: &mut peek_nth::PeekableNth<std::slice::Iter<'_, lex::Lex>>,
) -> Result<Statement, Error> {
    let token = iter.next().ok_or(Error::UnexpectedEOF)?;
    match token {
        lex::Lex::You3 => {
            return parse_init_define_statement_after_you3(&mut iter);
        }
        lex::Lex::Wei2Shi4 => {
            return parse_for_enum_statement_after_wei2shi4(&mut iter);
        }
        lex::Lex::Shu1Zhi1 => return Ok(Statement::Print),
        lex::Lex::Xi1Zhi1 => {
            return parse_assign_after_xi1zhi1(&mut iter);
        }
        lex::Lex::Wu2You3 => {
            let next = iter.next().ok_or(Error::UnexpectedEOF)?;
            match next {
                lex::Lex::IntNum(num) => {
                    match iter.next().ok_or(Error::UnexpectedEOF)? {
                        lex::Lex::Type(t) => {
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

                            let variable_count = match usize::try_from(interpret_intnum(num)) {
                                Err(_) => return Err(Error::InvalidVariableCount),
                                Ok(a) => a,
                            };

                            if variable_count == 0 {
                                return Err(Error::InvalidVariableCount);
                            }

                            let declare = DeclareStatement {
                                how_many_variables: variable_count as usize,
                                type_: *t,
                                data_arr: vec,
                            };

                            if let Some(lex::Lex::Ming2Zhi1) = iter.peek() {
                                iter.next();

                                // ('曰' IDENTIFIER)+

                                let mut idents = vec![];

                                loop {
                                    if let lex::Lex::Yue1 =
                                        iter.peek().ok_or(Error::UnexpectedEOF)?
                                    {
                                        iter.next();
                                        if let lex::Lex::Identifier(ident) =
                                            iter.next().ok_or(Error::UnexpectedEOF)?
                                        {
                                            idents.push(Identifier(ident.clone()));
                                        } else {
                                            return Err(Error::SomethingWentWrong);
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                if idents.is_empty() {
                                    return Err(Error::SomethingWentWrong); // we need at least one 曰 now that we have seen 名之
                                }

                                return Ok(Statement::Define {
                                    decl: declare,
                                    idents: idents,
                                });
                            } else {
                                return Ok(Statement::Declare(declare));
                            }
                        }
                        _ => unimplemented!(), // 術, 物
                    }
                }
                _ => return Err(Error::SomethingWentWrong),
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
