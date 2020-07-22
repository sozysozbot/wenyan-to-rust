use crate::lex;
type LexIter<'a> = peek_nth::PeekableNth<std::slice::Iter<'a, lex::Lex>>;
pub type CondPlusStatements = (IfCond, Vec<Statement>);

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
    ForArr {
        list: Identifier,
        elem: Identifier,
        stmts: Vec<Statement>,
    },
    Loop {
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
    If {
        ifcase: CondPlusStatements,
        elseifcases: Vec<CondPlusStatements>,
        elsecase: Vec<Statement>,
    },
    // Return,
    Math {
        math: MathKind,
    },
    Assign {
        ident: Identifier,
        data: DataOrQi2,
    },
    // Import,
    // Object,
    Reference {
        data: Data,
        ident: Option<Identifier>,
    },
    ArrayFill {
        what_to_fill: DataOrQi2,
        elems: Vec<Data>,
    },
    Flush,
    // Break,
    // Comment,
    /// not found in the spec, but since `名之曰「戊」` is compiled to `var WU4 = undefined;`, we need this
    NameMulti {
        idents: Vec<Identifier>,
    },
}

#[derive(Debug, Clone)]
pub enum IfCond {
    Unary(DataOrQi2),
    Binary(DataOrQi2, lex::IfLogicOp, DataOrQi2),
    NotQi2,
}

//#[derive(Debug)]
//pub enum LogicBinaryOp {
//}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum DivBinaryOp {
    Div,
    Mod,
}

impl DivBinaryOp {
    pub fn to_str(self) -> &'static str {
        match self {
            DivBinaryOp::Div => "/",
            DivBinaryOp::Mod => "%",
        }
    }
}

#[derive(Debug)]
pub enum MathKind {
    ArithBinaryMath(lex::ArithBinaryOp, DataOrQi2, lex::Preposition, DataOrQi2),
    ArithUnaryMath(DataOrQi2),
    BooleanAlgebra(Identifier, Identifier, lex::LogicBinaryOp),
    ModMath(DivBinaryOp, DataOrQi2, lex::Preposition, DataOrQi2),
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

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Error {
    SomethingWentWrong,
    UnexpectedEOF,
    InvalidVariableCount,
}

#[allow(clippy::enum_glob_use)]
fn interpret_intnum(num: &lex::IntNum) -> i64 {
    use lex::IntMult::*;
    use lex::IntNumKeywords::*;
    let lex::IntNum(v) = num;
    match *v.as_slice() {
        [Ling2] => 0,
        [IntDigit(d)] => d.to_num(),

        [IntMult(Shi2)] => 10,
        [IntMult(Shi2), IntDigit(d)] => 10 + d.to_num(),
        [IntDigit(d), IntMult(Shi2)] => 10 * d.to_num(),
        [IntDigit(d), IntMult(Shi2), IntDigit(e)] => 10 * d.to_num() + e.to_num(),

        [IntMult(Bai3)] => 100,

        [IntDigit(c), IntMult(Bai3)] => 100 * c.to_num(),

        [IntDigit(c), IntMult(Bai3), IntDigit(d), IntMult(Shi2)] => {
            100 * c.to_num() + 10 * d.to_num()
        }

        [IntDigit(c), IntMult(Bai3), IntMult(Shi2), IntDigit(e)] => {
            100 * c.to_num() + 10 + e.to_num()
        }

        [IntDigit(c), IntMult(Bai3), IntDigit(d), IntMult(Shi2), IntDigit(e)] => {
            100 * c.to_num() + 10 * d.to_num() + e.to_num()
        }
        [IntDigit(b), IntMult(Qian1), IntDigit(c), IntMult(Bai3), IntDigit(d), IntMult(Shi2), IntDigit(e)] => {
            1000 * b.to_num() + 100 * c.to_num() + 10 * d.to_num() + e.to_num()
        }
        [IntDigit(a), IntMult(Wan4), IntDigit(b), IntMult(Qian1), IntDigit(c), IntMult(Bai3), IntDigit(d), IntMult(Shi2), IntDigit(e)] => {
            10000 * a.to_num() + 1000 * b.to_num() + 100 * c.to_num() + 10 * d.to_num() + e.to_num()
        }

        [IntMult(Qian1)] => 1000,
        _ => unimplemented!("parsing integer"),
    }
}

#[derive(Debug, Clone)]
pub enum DataOrQi2 {
    Data(Data),
    Qi2,
}

fn parse_data_or_qi2(iter: &mut LexIter<'_>) -> Result<DataOrQi2, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::StringLiteral(strlit) => {
            Ok(DataOrQi2::Data(Data::StringLiteral(strlit.to_string())))
        }
        lex::Lex::BoolValue(bv) => Ok(DataOrQi2::Data(Data::BoolValue(bv.interpret()))),
        lex::Lex::Identifier(ident) => Ok(DataOrQi2::Data(Data::Identifier(Identifier(
            ident.to_string(),
        )))),
        lex::Lex::IntNum(intnum) => Ok(DataOrQi2::Data(Data::IntNum(interpret_intnum(intnum)))), /* FIXME: must handle float */
        lex::Lex::Qi2 => Ok(DataOrQi2::Qi2),
        _ => Err(Error::SomethingWentWrong),
    }
}

fn parse_preposition(iter: &mut LexIter<'_>) -> Result<lex::Preposition, Error> {
    if let lex::Lex::Preposition(p) = iter.next().ok_or(Error::UnexpectedEOF)? {
        Ok(*p)
    } else {
        Err(Error::SomethingWentWrong)
    }
}

fn parse_data(iter: &mut LexIter<'_>) -> Result<Data, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::StringLiteral(strlit) => Ok(Data::StringLiteral(strlit.to_string())),
        lex::Lex::BoolValue(bv) => Ok(Data::BoolValue(bv.interpret())),
        lex::Lex::Identifier(ident) => Ok(Data::Identifier(Identifier(ident.to_string()))),
        lex::Lex::IntNum(intnum) => Ok(Data::IntNum(interpret_intnum(intnum))), /* FIXME: must handle float */
        _ => Err(Error::SomethingWentWrong),
    }
}

fn parse_init_define_statement_after_you3(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
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
            Some(lex::Lex::Ming2Zhi1) => {
                let next = iter.next();
                match next.ok_or(Error::UnexpectedEOF)? {
                    lex::Lex::Yue1 => {
                        let next = iter.next();
                        match next.ok_or(Error::UnexpectedEOF)? {
                            lex::Lex::Identifier(ident) => {
                                 Ok(Statement::InitDefine {
                                    type_: *t,
                                    name: Identifier(ident.to_string()),
                                    data
                                })
                            }
                            _ =>  Err(Error::SomethingWentWrong),
                        }
                    }
                    _ => Err(Error::SomethingWentWrong),
                }
            }
            None | Some(..) => panic!("If this message is obtained by a wenyan program that successfully compiles in the original edition, please submit an issue."),
        }
    } else {
        Err(Error::SomethingWentWrong)
    }
}

fn parse_for_enum_statement_after_wei2shi4(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::IntNum(num) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4Loop => {
                let mut inner = vec![];
                loop {
                    if iter.peek() == Some(&&lex::Lex::Yun2Yun2OrYe3) {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                Ok(Statement::ForEnum {
                    num: interpret_intnum(num),
                    statements: inner,
                })
            }
            _ => Err(Error::SomethingWentWrong),
        },
        lex::Lex::Identifier(ident) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4Loop => {
                let mut inner = vec![];
                loop {
                    if iter.peek() == Some(&&lex::Lex::Yun2Yun2OrYe3) {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                Ok(Statement::ForEnumIdent {
                    ident: Identifier(ident.to_string()),
                    statements: inner,
                })
            }
            _ => Err(Error::SomethingWentWrong),
        },
        _ => Err(Error::SomethingWentWrong),
    }
}

fn parse_assign_after_xi1zhi1(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
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
                    let data = parse_data_or_qi2(&mut iter)?;
                    match iter.next().ok_or(Error::UnexpectedEOF)? {
                        lex::Lex::Zhi1 => unimplemented!("昔之 ... 者今data之INT_NUM是矣"),
                        lex::Lex::Shi4Yi3 => Ok(Statement::Assign {
                            ident: Identifier(ident.clone()),
                            data,
                        }),
                        _ => Err(Error::SomethingWentWrong),
                    }
                }
                _ => Err(Error::SomethingWentWrong),
            },
            _ => Err(Error::SomethingWentWrong),
        }
    } else {
        Err(Error::SomethingWentWrong)
    }
}

fn parse_reference_statement_after_fu2(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    // reference_statement         : '夫' data ('之' (STRING_LITERAL|INT_NUM|'其餘'|IDENTIFIER|'長'))? name_single_statement? ;
    let data = parse_data(&mut iter)?;
    match iter.peek() {
        Some(lex::Lex::Zhi1) => unimplemented!("夫 data 之 ..."),
        Some(lex::Lex::Ming2Zhi1) => {
            iter.next();
            if let lex::Lex::Yue1 = iter.next().ok_or(Error::UnexpectedEOF)? {
                if let lex::Lex::Identifier(ident) = iter.next().ok_or(Error::UnexpectedEOF)? {
                    Ok(Statement::Reference {
                        data,
                        ident: Some(Identifier(ident.to_string())),
                    })
                } else {
                    Err(Error::SomethingWentWrong)
                }
            } else {
                Err(Error::SomethingWentWrong)
            }
        }
        _ => Ok(Statement::Reference { data, ident: None }),
    }
}

fn parse_elseif(mut iter: &mut LexIter<'_>) -> Result<CondPlusStatements, Error> {
    if let lex::Lex::Huo4Ruo4 = iter.next().ok_or(Error::UnexpectedEOF)? {
        let cond = parse_ifexpression_plus_zhe3(&mut iter)?;
        let mut stmts = vec![parse_statement(&mut iter)?];
        loop {
            // loop until you see either 或若, 若非, or FOR_IF_END
            match iter.peek() {
                Some(lex::Lex::Huo4Ruo4)
                | Some(lex::Lex::Ruo4Fei1)
                | Some(lex::Lex::Yun2Yun2OrYe3) => return Ok((cond, stmts)),
                _ => {}
            }
            stmts.push(parse_statement(&mut iter)?);
        }
    } else {
        Err(Error::SomethingWentWrong)
    }
}

fn parse_after_ruo4fei1(mut iter: &mut LexIter<'_>) -> Result<Vec<Statement>, Error> {
    let mut elsecase = vec![parse_statement(&mut iter)?];
    loop {
        match iter.peek() {
            Some(lex::Lex::Yun2Yun2OrYe3) => {
                iter.next();
                return Ok(elsecase);
            }
            None => return Err(Error::UnexpectedEOF),
            Some(..) => {}
        }
        elsecase.push(parse_statement(&mut iter)?)
    }
}

fn parse_if_statement_after_zhe3(
    mut iter: &mut LexIter<'_>,
) -> Result<(Vec<Statement>, Vec<CondPlusStatements>, Vec<Statement>), Error> {
    // FIXME:
    // currently: statement+ ('若非' statement+)? FOR_IF_END ;
    // want: statement+ ('或若' if_expression '者' statement+)* ('若非' statement+)? FOR_IF_END ;
    let mut ifcase = vec![parse_statement(&mut iter)?];
    loop {
        match iter.peek() {
            Some(lex::Lex::Huo4Ruo4) => {
                // 或若 ...
                let mut condstmt_vec = vec![parse_elseif(&mut iter)?];
                loop {
                    match iter.peek() {
                        Some(lex::Lex::Yun2Yun2OrYe3) => {
                            iter.next();
                            return Ok((ifcase, condstmt_vec, vec![]));
                        }
                        Some(lex::Lex::Huo4Ruo4) => {}
                        Some(lex::Lex::Ruo4Fei1) => {
                            iter.next();
                            return Ok((ifcase, condstmt_vec, parse_after_ruo4fei1(&mut iter)?));
                        }
                        _ => unreachable!(),
                    }
                    condstmt_vec.push(parse_elseif(&mut iter)?);
                }
            }
            Some(lex::Lex::Ruo4Fei1) => {
                iter.next();
                return Ok((ifcase, vec![], parse_after_ruo4fei1(&mut iter)?));
            }
            Some(lex::Lex::Yun2Yun2OrYe3) => {
                iter.next();
                return Ok((ifcase, vec![], vec![]));
            }
            None => return Err(Error::UnexpectedEOF),
            Some(..) => {}
        }
        ifcase.push(parse_statement(&mut iter)?);
    }
}

/// ```
/// if_expression               : unary_if_expression|binary_if_expression ;
/// unary_if_expression         : data|(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))|'其' ;
/// binary_if_expression        : unary_if_expression IF_LOGIC_OP unary_if_expression ;
/// ```
fn parse_ifexpression_plus_zhe3(mut iter: &mut LexIter<'_>) -> Result<IfCond, Error> {
    let data = parse_data_or_qi2(&mut iter)?; // FIXME: the possibility of `(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))` is ignored
    match iter.peek() {
        Some(lex::Lex::Zhe3) => {
            iter.next();
            Ok(IfCond::Unary(data))
        }
        Some(lex::Lex::IfLogicOp(op)) => {
            iter.next();
            let data2 = parse_data_or_qi2(&mut iter)?; // FIXME: the possibility of `(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))` is ignored
            match iter.next().ok_or(Error::UnexpectedEOF)? {
                lex::Lex::Zhe3 => Ok(IfCond::Binary(data, *op, data2)),
                _ => Err(Error::SomethingWentWrong),
            }
        }
        _ => Err(Error::SomethingWentWrong),
    }
}

fn parse_statement(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::Fan2 => {
            if let lex::Lex::Identifier(list) = iter.next().ok_or(Error::UnexpectedEOF)? {
                if let lex::Lex::Zhong1Zhi1 = iter.next().ok_or(Error::UnexpectedEOF)? {
                    if let lex::Lex::Identifier(elem) = iter.next().ok_or(Error::UnexpectedEOF)? {
                        let mut stmts = vec![];
                        loop {
                            match iter.peek().ok_or(Error::SomethingWentWrong)? {
                                lex::Lex::Yun2Yun2OrYe3 => {
                                    iter.next();
                                    return Ok(Statement::ForArr {
                                        list: Identifier(list.to_string()),
                                        elem: Identifier(elem.to_string()),
                                        stmts,
                                    });
                                }
                                _ => {}
                            }
                            stmts.push(parse_statement(&mut iter)?);
                        }
                    } else {
                        Err(Error::SomethingWentWrong)
                    }
                } else {
                    Err(Error::SomethingWentWrong)
                }
            } else {
                Err(Error::SomethingWentWrong)
            }
        }
        lex::Lex::Chong1 => {
            // array_push_statement        : '充' (IDENTIFIER|'其') (PREPOSITION_RIGHT data)+ name_single_statement?;
            let what_to_fill = parse_data_or_qi2(&mut iter)?;
            if let lex::Lex::Preposition(lex::Preposition::Yi3) =
                iter.next().ok_or(Error::UnexpectedEOF)?
            {
                let mut elems = vec![parse_data(&mut iter)?];
                while let Some(lex::Lex::Preposition(lex::Preposition::Yi3)) = iter.peek() {
                    iter.next();
                    elems.push(parse_data(&mut iter)?);
                }
                match iter.peek() {
                    Some(lex::Lex::Ming2Zhi1) => unimplemented!("ming2zhi1 after array"), // remaining: name_single_statement?
                    _ => Ok(Statement::ArrayFill {
                        what_to_fill,
                        elems,
                    }),
                }
            } else {
                Err(Error::SomethingWentWrong)
            }
        }
        lex::Lex::Ruo4Qi2Bu4Ran2Zhe3 => {
            let (ifstmts, elseifcases, elsecase) = parse_if_statement_after_zhe3(&mut iter)?;
            Ok(Statement::If {
                ifcase: (IfCond::NotQi2, ifstmts),
                elseifcases,
                elsecase,
            })
        }
        lex::Lex::Ruo4Qi2Ran2Zhe3 => {
            let (ifstmts, elseifcases, elsecase) = parse_if_statement_after_zhe3(&mut iter)?;
            Ok(Statement::If {
                ifcase: (IfCond::Unary(DataOrQi2::Qi2), ifstmts),
                elseifcases,
                elsecase,
            })
        }
        lex::Lex::Ruo4 => {
            // if_statement                : '若' if_expression '者' statement+ ('或若' if_expression '者' statement+)* ('若非' statement+)? FOR_IF_END ;
            let ifexpr = parse_ifexpression_plus_zhe3(&mut iter)?;
            let (ifstmts, elseifcases, elsecase) = parse_if_statement_after_zhe3(&mut iter)?;
            Ok(Statement::If {
                ifcase: (ifexpr, ifstmts),
                elseifcases,
                elsecase,
            })
        }
        lex::Lex::Fu2 => parse_after_fu2(&mut iter),
        lex::Lex::Chu2 => {
            let data1 = parse_data_or_qi2(&mut iter)?;
            let prep = parse_preposition(&mut iter)?;
            let data2 = parse_data_or_qi2(&mut iter)?; // spec.html does not allow qi2 here, but the implementation seems to allow it
            match iter.peek() {
                Some(lex::Lex::Suo3Yu2Ji3He2) => {
                    iter.next();
                    Ok(Statement::Math {
                        math: MathKind::ModMath(DivBinaryOp::Mod, data1, prep, data2),
                    })
                }
                _ => Ok(Statement::Math {
                    math: MathKind::ModMath(DivBinaryOp::Div, data1, prep, data2),
                }),
            }
        }
        lex::Lex::Ming2Zhi1 => {
            let idents = parse_name_multi_statement_after_ming2zhi1(&mut iter)?;
            Ok(Statement::NameMulti { idents })
        }
        lex::Lex::Yi1Flush => Ok(Statement::Flush),
        lex::Lex::ArithBinaryOp(op) => {
            let data1 = parse_data_or_qi2(&mut iter)?;
            let prep = parse_preposition(&mut iter)?;
            let data2 = parse_data_or_qi2(&mut iter)?;
            // Cases where 名之 ... follows is treated as a separate NameMulti statement.
            Ok(Statement::Math {
                math: MathKind::ArithBinaryMath(*op, data1, prep, data2),
            })
        }
        lex::Lex::Bian4Change => Ok(Statement::Math {
            math: MathKind::ArithUnaryMath(parse_data_or_qi2(&mut iter)?),
        }),
        lex::Lex::You3 => parse_init_define_statement_after_you3(&mut iter),
        lex::Lex::Heng2Wei2Shi4 => {
            let mut statements = vec![];
            loop {
                match **iter.peek().ok_or(Error::UnexpectedEOF)? {
                    lex::Lex::Yun2Yun2OrYe3 => {
                        iter.next();
                        return Ok(Statement::Loop { statements });
                    }
                    _ => statements.push(parse_statement(&mut iter)?),
                }
            }
        }
        lex::Lex::Wei2Shi4 => parse_for_enum_statement_after_wei2shi4(&mut iter),
        lex::Lex::Shu1Zhi1 => Ok(Statement::Print),
        lex::Lex::Xi1Zhi1 => parse_assign_after_xi1zhi1(&mut iter),
        lex::Lex::Wu2You3 => parse_after_wu2you3(&mut iter),
        a => unimplemented!("Parser encountered {:?}", a),
    }
}

/// two candidates:
/// `boolean_algebra_statement   : '夫' IDENTIFIER IDENTIFIER LOGIC_BINARY_OP ;`
/// `reference_statement         : '夫' data ('之' (STRING_LITERAL|INT_NUM|'其餘'|IDENTIFIER|'長'))? name_single_statement? ;`
fn parse_after_fu2(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    match iter.peek() {
        Some(lex::Lex::Identifier(ident)) => match iter.peek_nth(1) {
            Some(lex::Lex::Identifier(ident2)) => match iter.peek_nth(2) {
                Some(lex::Lex::LogicBinaryOp(op)) => {
                    iter.next(); // first ident
                    iter.next(); // second ident
                    iter.next(); // operator
                    Ok(Statement::Math {
                        math: MathKind::BooleanAlgebra(
                            Identifier(ident.to_string()),
                            Identifier(ident2.to_string()),
                            *op,
                        ),
                    })
                }
                _ => parse_reference_statement_after_fu2(&mut iter),
            },
            _ => parse_reference_statement_after_fu2(&mut iter),
        },
        None => Err(Error::UnexpectedEOF),
        Some(_) => parse_reference_statement_after_fu2(&mut iter),
    }
}

fn parse_after_wu2you3(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
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
                        let idents = parse_name_multi_statement_after_ming2zhi1(&mut iter)?;
                        Ok(Statement::Define {
                            decl: declare,
                            idents,
                        })
                    } else {
                        Ok(Statement::Declare(declare))
                    }
                }
                _ => unimplemented!(), // 術, 物
            }
        }
        _ => Err(Error::SomethingWentWrong),
    }
}

fn parse_name_multi_statement_after_ming2zhi1(
    iter: &mut LexIter<'_>,
) -> Result<Vec<Identifier>, Error> {
    // ('曰' IDENTIFIER)+

    let mut idents = vec![];

    while let Some(lex::Lex::Yue1) = iter.peek() {
        iter.next();
        if let lex::Lex::Identifier(ident) = iter.next().ok_or(Error::UnexpectedEOF)? {
            idents.push(Identifier(ident.clone()));
        } else {
            return Err(Error::SomethingWentWrong);
        }
    }

    if idents.is_empty() {
        return Err(Error::SomethingWentWrong); // we need at least one 曰 now that we have seen 名之
    }

    Ok(idents)
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
