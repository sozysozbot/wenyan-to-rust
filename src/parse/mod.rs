use crate::lex;
type LexIter<'a> = peek_nth::PeekableNth<std::slice::Iter<'a, lex::Lex>>;
pub type CondPlusStatements = (IfCond, Vec<Statement>);

#[derive(Debug)]
pub enum Lvalue {
    Simple(Identifier),
    Index(Identifier, i64),
    IndexByIdent(Identifier, Identifier),
}

#[derive(Debug, Clone)]
pub enum Value<T> {
    Simple(T),
    Index(T, i64),
    IndexByIdent(T, Identifier),
    Length(T),
}

#[derive(Debug)]
pub enum Statement {
    Declare(DeclareStatement),
    Print,
    ForEnum {
        num: i64,
        statements: Vec<Statement>,
    },
    ForEnumIdent {
        ident: OrQi2<Identifier>,
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
    Assignment {
        lvalue: Lvalue,
        rvalue: Value<OrQi2<Data>>,
    },
    // Import,
    // Object,
    Reference {
        rvalue: Value<Data>,
    },
    ReferenceWhatIsLeft {
        data: Data,
    },
    ArrayFill {
        what_to_fill: OrQi2<Identifier>,
        elems: Vec<Data>,
    },
    ArrayCat {
        append_to: OrQi2<Identifier>,
        elems: Vec<Identifier>,
    },
    Flush,
    Break,

    /// not found in the spec
    Continue,

    // Comment,
    /// not found in the spec, but since `名之曰「戊」` is compiled to `var WU4 = undefined;`, we need this
    NameMulti {
        idents: Vec<Identifier>,
    },
}

#[derive(Debug, Clone)]
pub enum IfCond {
    Unary(UnaryIfExpr),
    Binary(UnaryIfExpr, lex::IfLogicOp, UnaryIfExpr),
    NotQi2,
}

#[derive(Debug, Clone)]
pub enum UnaryIfExpr {
    Simple(OrQi2<Data>),
    Complex(Value<Data>),
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
    ArithBinaryMath(
        lex::ArithBinaryOp,
        OrQi2<Data>,
        lex::Preposition,
        OrQi2<Data>,
    ),
    ArithUnaryMath(OrQi2<Data>),
    BooleanAlgebra(Identifier, Identifier, lex::LogicBinaryOp),
    ModMath(DivBinaryOp, OrQi2<Data>, lex::Preposition, OrQi2<Data>),
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

use position::{here, Position};

#[derive(Debug)]
pub enum Error {
    SomethingWentWrong(Position),
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
pub enum OrQi2<T> {
    NotQi2(T),
    Qi2,
}

impl From<&OrQi2<Identifier>> for OrQi2<Data> {
    fn from(identorqi2: &OrQi2<Identifier>) -> Self {
        match identorqi2 {
            OrQi2::NotQi2(ident) => OrQi2::NotQi2(Data::Identifier(ident.clone())),
            OrQi2::Qi2 => OrQi2::Qi2,
        }
    }
}

fn parse_data_or_qi2(iter: &mut LexIter<'_>) -> Result<OrQi2<Data>, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::StringLiteral(strlit) => {
            Ok(OrQi2::NotQi2(Data::StringLiteral(strlit.to_string())))
        }
        lex::Lex::BoolValue(bv) => Ok(OrQi2::NotQi2(Data::BoolValue(bv.interpret()))),
        lex::Lex::Identifier(ident) => Ok(OrQi2::NotQi2(Data::Identifier(Identifier(
            ident.to_string(),
        )))),
        lex::Lex::IntNum(intnum) => Ok(OrQi2::NotQi2(Data::IntNum(interpret_intnum(intnum)))), /* FIXME: must handle float */
        lex::Lex::Qi2 => Ok(OrQi2::Qi2),
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}

fn parse_ident_or_qi2(iter: &mut LexIter<'_>) -> Result<OrQi2<Identifier>, Error> {
    let token = match iter.next() {
        None => return Err(Error::UnexpectedEOF),
        Some(a) => a,
    };

    match token {
        lex::Lex::Identifier(ident) => Ok(OrQi2::NotQi2(Identifier(ident.to_string()))),
        lex::Lex::Qi2 => Ok(OrQi2::Qi2),
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}

fn parse_preposition(iter: &mut LexIter<'_>) -> Result<lex::Preposition, Error> {
    if let lex::Lex::Preposition(p) = iter.next().ok_or(Error::UnexpectedEOF)? {
        Ok(*p)
    } else {
        Err(Error::SomethingWentWrong(here!()))
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
        _ => Err(Error::SomethingWentWrong(here!())),
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
                            _ =>  Err(Error::SomethingWentWrong(here!())),
                        }
                    }
                    _ => Err(Error::SomethingWentWrong(here!())),
                }
            }
            None | Some(..) => panic!("If this message is obtained by a wenyan program that successfully compiles in the original edition, please submit an issue."),
        }
    } else {
        Err(Error::SomethingWentWrong(here!()))
    }
}

fn parse_for_enum_statement_after_wei2shi4(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::IntNum(num) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4Loop => {
                let mut inner = vec![];
                loop {
                    if let Some(&&lex::Lex::Yun2Yun2OrYe3(_)) = iter.peek() {
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
            _ => Err(Error::SomethingWentWrong(here!())),
        },
        lex::Lex::Identifier(ident) => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4Loop => {
                let mut inner = vec![];
                loop {
                    if let Some(&&lex::Lex::Yun2Yun2OrYe3(_)) = iter.peek() {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                Ok(Statement::ForEnumIdent {
                    ident: OrQi2::NotQi2(Identifier(ident.to_string())),
                    statements: inner,
                })
            }
            _ => Err(Error::SomethingWentWrong(here!())),
        },

        // not found in spec.html
        lex::Lex::Qi2 => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::Bian4Loop => {
                let mut inner = vec![];
                loop {
                    if let Some(&&lex::Lex::Yun2Yun2OrYe3(_)) = iter.peek() {
                        iter.next();
                        break;
                    }

                    inner.push(parse_statement(&mut iter)?);
                }
                Ok(Statement::ForEnumIdent {
                    ident: OrQi2::Qi2,
                    statements: inner,
                })
            }
            _ => Err(Error::SomethingWentWrong(here!())),
        },
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}

fn parse_identifier(iter: &mut LexIter<'_>) -> Result<Identifier, Error> {
    if let lex::Lex::Identifier(ident) = iter.next().ok_or(Error::UnexpectedEOF)? {
        Ok(Identifier(ident.to_string()))
    } else {
        Err(Error::SomethingWentWrong(here!()))
    }
}

fn parse_optional_indexer<T>(iter: &mut LexIter<'_>, data: T) -> Result<Value<T>, Error> {
    let next_token = iter.peek();
    if Some(&&lex::Lex::Zhi1) == next_token {
        iter.next();
        match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::IntNum(int_num) => Ok(Value::Index(data, interpret_intnum(int_num))),
            lex::Lex::StringLiteral(lit) => unimplemented!("data之STRING_LITERAL"),
            lex::Lex::Identifier(id) => Ok(Value::IndexByIdent(data, Identifier(id.to_string()))),
            lex::Lex::Chang2 => Ok(Value::Length(data)),
            _ => Err(Error::SomethingWentWrong(here!())),
        }
    } else {
        Ok(Value::Simple(data))
    }
}

fn parse_assign_after_zhe3(mut iter: &mut LexIter<'_>) -> Result<Value<OrQi2<Data>>, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::Jin1Bu4Fu4Cun2Yi3 => unimplemented!("昔之 ... 者今不復存矣"),
        lex::Lex::Jin1 => {
            let data = parse_data_or_qi2(&mut iter)?;
            let res = parse_optional_indexer(&mut iter, data)?;
            if let lex::Lex::Shi4Yi3 = iter.next().ok_or(Error::UnexpectedEOF)? {
                Ok(res)
            } else {
                Err(Error::SomethingWentWrong(here!()))
            }
        }
        _ => Err(Error::SomethingWentWrong(here!())),
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
    let ident = parse_identifier(&mut iter)?;
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::Zhi1 => match iter.next().ok_or(Error::UnexpectedEOF)? {
            lex::Lex::IntNum(int_num) => {
                if let lex::Lex::Zhe3 = iter.next().ok_or(Error::UnexpectedEOF)? {
                    let rvalue = parse_assign_after_zhe3(&mut iter)?;
                    Ok(Statement::Assignment {
                        lvalue: Lvalue::Index(ident, interpret_intnum(&int_num)),
                        rvalue,
                    })
                } else {
                    Err(Error::SomethingWentWrong(here!()))
                }
            }
            lex::Lex::StringLiteral(lit) => unimplemented!("昔之 IDENTIFIER 之 STRING"),
            lex::Lex::Identifier(id) => {
                if let lex::Lex::Zhe3 = iter.next().ok_or(Error::UnexpectedEOF)? {
                    let rvalue = parse_assign_after_zhe3(&mut iter)?;
                    Ok(Statement::Assignment {
                        lvalue: Lvalue::IndexByIdent(ident, Identifier(id.to_string())),
                        rvalue,
                    })
                } else {
                    Err(Error::SomethingWentWrong(here!()))
                }
            }
            _ => Err(Error::SomethingWentWrong(here!())),
        },
        lex::Lex::Zhe3 => {
            let rvalue = parse_assign_after_zhe3(&mut iter)?;
            Ok(Statement::Assignment {
                lvalue: Lvalue::Simple(ident),
                rvalue,
            })
        }
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}

fn parse_reference_statement_after_fu2(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    // reference_statement         : '夫' data ('之' (STRING_LITERAL|INT_NUM|'其餘'|IDENTIFIER|'長'))? name_single_statement? ;
    // but no need to handle name_single_statement;
    // since
    // ```
    // 加二以四。夫「丙」。名之曰「戊」曰「己」。
    // ```
    // compiles, it must be that name_single_statement can just as validly treated as a separate entity.
    let data = parse_data(&mut iter)?;
    match iter.peek() {
        Some(lex::Lex::Zhi1) => {
            // ('之' (STRING_LITERAL|INT_NUM|'其餘'|IDENTIFIER|'長'))?
            iter.next();
            match iter
                .next()
                .ok_or_else(|| Error::SomethingWentWrong(here!()))?
            {
                lex::Lex::StringLiteral(lit) => unimplemented!("夫 data 之 STRING_LITERAL"),
                lex::Lex::IntNum(index) => Ok(Statement::Reference {
                    rvalue: Value::Index(data, interpret_intnum(&index)),
                }),
                lex::Lex::Qi2Yu2 => Ok(Statement::ReferenceWhatIsLeft { data }),
                lex::Lex::Identifier(ident) => Ok(Statement::Reference {
                    rvalue: Value::IndexByIdent(data, Identifier(ident.clone())),
                }),
                lex::Lex::Chang2 => Ok(Statement::Reference {
                    rvalue: Value::Length(data),
                }),
                _ => Err(Error::SomethingWentWrong(here!())),
            }
        }
        _ => Ok(Statement::Reference {
            rvalue: Value::Simple(data),
        }),
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
                | Some(lex::Lex::Yun2Yun2OrYe3(_)) => return Ok((cond, stmts)),
                _ => {}
            }
            stmts.push(parse_statement(&mut iter)?);
        }
    } else {
        Err(Error::SomethingWentWrong(here!()))
    }
}

fn parse_after_ruo4fei1(mut iter: &mut LexIter<'_>) -> Result<Vec<Statement>, Error> {
    let mut elsecase = vec![parse_statement(&mut iter)?];
    loop {
        match iter.peek() {
            Some(lex::Lex::Yun2Yun2OrYe3(_)) => {
                iter.next();
                return Ok(elsecase);
            }
            None => return Err(Error::UnexpectedEOF),
            Some(..) => {}
        }
        elsecase.push(parse_statement(&mut iter)?)
    }
}

struct IfStmtAfterZhe3 {
    ifstmts: Vec<Statement>,
    elseifcases: Vec<CondPlusStatements>,
    elsecase: Vec<Statement>,
}

impl IfStmtAfterZhe3 {
    pub fn into_stmt_with_cond(self, cond: IfCond) -> Statement {
        let IfStmtAfterZhe3 {
            ifstmts,
            elseifcases,
            elsecase,
        } = self;
        Statement::If {
            ifcase: (cond, ifstmts),
            elseifcases,
            elsecase,
        }
    }
}

fn parse_if_statement_after_zhe3(mut iter: &mut LexIter<'_>) -> Result<IfStmtAfterZhe3, Error> {
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
                        Some(lex::Lex::Yun2Yun2OrYe3(_)) => {
                            iter.next();
                            return Ok(IfStmtAfterZhe3 {
                                ifstmts: ifcase,
                                elseifcases: condstmt_vec,
                                elsecase: vec![],
                            });
                        }
                        Some(lex::Lex::Huo4Ruo4) => {}
                        Some(lex::Lex::Ruo4Fei1) => {
                            iter.next();
                            return Ok(IfStmtAfterZhe3 {
                                ifstmts: ifcase,
                                elseifcases: condstmt_vec,
                                elsecase: parse_after_ruo4fei1(&mut iter)?,
                            });
                        }
                        _ => unreachable!(),
                    }
                    condstmt_vec.push(parse_elseif(&mut iter)?);
                }
            }
            Some(lex::Lex::Ruo4Fei1) => {
                iter.next();
                return Ok(IfStmtAfterZhe3 {
                    ifstmts: ifcase,
                    elseifcases: vec![],
                    elsecase: parse_after_ruo4fei1(&mut iter)?,
                });
            }
            Some(lex::Lex::Yun2Yun2OrYe3(_)) => {
                iter.next();
                return Ok(IfStmtAfterZhe3 {
                    ifstmts: ifcase,
                    elseifcases: vec![],
                    elsecase: vec![],
                });
            }
            None => return Err(Error::UnexpectedEOF),
            Some(..) => {}
        }
        ifcase.push(parse_statement(&mut iter)?);
    }
}

/// ```
/// unary_if_expression         : data|(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))|'其' ;
/// ```
fn parse_unary_if_expression(mut iter: &mut LexIter<'_>) -> Result<UnaryIfExpr, Error> {
    if let lex::Lex::Identifier(i) = iter.peek().ok_or(Error::UnexpectedEOF)? {
        // either `data` or `(IDENTIFIER '之'('長'|STRING_LITERAL|IDENTIFIER))`
        if let Some(lex::Lex::Zhi1) = iter.peek_nth(1) {
            iter.next(); // Identifier(i)
            let res =
                parse_optional_indexer(&mut iter, Data::Identifier(Identifier(i.to_string())))?;
            Ok(UnaryIfExpr::Complex(res))
        } else {
            let data2 = parse_data_or_qi2(&mut iter)?;
            Ok(UnaryIfExpr::Simple(data2))
        }
    } else {
        let data2 = parse_data_or_qi2(&mut iter)?;
        Ok(UnaryIfExpr::Simple(data2))
    }
}

/// ```
/// if_expression               : unary_if_expression|binary_if_expression ;
/// binary_if_expression        : unary_if_expression IF_LOGIC_OP unary_if_expression ;
/// ```
fn parse_ifexpression_plus_zhe3(mut iter: &mut LexIter<'_>) -> Result<IfCond, Error> {
    let data = parse_unary_if_expression(&mut iter)?;
    match iter.peek() {
        Some(lex::Lex::Zhe3) => {
            iter.next();
            Ok(IfCond::Unary(data))
        }
        Some(lex::Lex::IfLogicOp(op)) => {
            iter.next();
            let data2 = parse_unary_if_expression(&mut iter)?;
            if let lex::Lex::Zhe3 = iter.next().ok_or(Error::UnexpectedEOF)? {
                Ok(IfCond::Binary(data, *op, data2))
            } else {
                Err(Error::SomethingWentWrong(here!()))
            }
        }
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}
///```
///array_push_statement        : '充' (IDENTIFIER|'其') (PREPOSITION_RIGHT data)+ name_single_statement?;
///```
fn parse_arraypush_after_chong1(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    let what_to_fill = parse_ident_or_qi2(&mut iter)?;
    if let lex::Lex::Preposition(lex::Preposition::Yi3) = iter.next().ok_or(Error::UnexpectedEOF)? {
        let mut elems = vec![parse_data(&mut iter)?];
        while let Some(lex::Lex::Preposition(lex::Preposition::Yi3)) = iter.peek() {
            iter.next();
            elems.push(parse_data(&mut iter)?);
        }
        Ok(Statement::ArrayFill {
            what_to_fill,
            elems,
        })
    } else {
        Err(Error::SomethingWentWrong(here!()))
    }
}

/// ```
/// '銜' (IDENTIFIER|'其') (PREPOSITION_RIGHT IDENTIFIER)+ name_single_statement?;
/// ```
/// however, since
/// ```
/// 吾有一列。名之曰「甲」。充「甲」以三。充「甲」以五。
/// 吾有一列。名之曰「乙」。充「乙」以二。以九。以四。以二十二。
/// 加一以三。銜「甲」以「乙」。名之曰「丙」曰「丑」。
/// ```
/// compiles, there seems to be no reason to handle this case separately.

fn parse_arraycat_after_xian2(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    let append_to = parse_ident_or_qi2(&mut iter)?;
    if let lex::Lex::Preposition(lex::Preposition::Yi3) = iter.next().ok_or(Error::UnexpectedEOF)? {
        let mut elems = vec![parse_identifier(&mut iter)?];
        while let Some(lex::Lex::Preposition(lex::Preposition::Yi3)) = iter.peek() {
            iter.next();
            elems.push(parse_identifier(&mut iter)?);
        }
        Ok(Statement::ArrayCat { append_to, elems })
    } else {
        Err(Error::SomethingWentWrong(here!()))
    }
}

fn parse_statement(mut iter: &mut LexIter<'_>) -> Result<Statement, Error> {
    match iter.next().ok_or(Error::UnexpectedEOF)? {
        lex::Lex::Nai3Zhi3Shi4Bian4 => Ok(Statement::Continue),
        lex::Lex::Nai3Zhi3 => Ok(Statement::Break),
        lex::Lex::Fan2 => {
            let list = parse_identifier(&mut iter)?;
            if let lex::Lex::Zhong1Zhi1 = iter.next().ok_or(Error::UnexpectedEOF)? {
                let elem = parse_identifier(&mut iter)?;
                let mut stmts = vec![];
                loop {
                    if let lex::Lex::Yun2Yun2OrYe3(_) = iter
                        .peek()
                        .ok_or_else(|| Error::SomethingWentWrong(here!()))?
                    {
                        iter.next();
                        return Ok(Statement::ForArr { list, elem, stmts });
                    }
                    stmts.push(parse_statement(&mut iter)?);
                }
            } else {
                Err(Error::SomethingWentWrong(here!()))
            }
        }
        lex::Lex::Xian2 => parse_arraycat_after_xian2(&mut iter),
        lex::Lex::Chong1 => parse_arraypush_after_chong1(&mut iter),
        lex::Lex::Ruo4Qi2Bu4Ran2Zhe3 => {
            Ok(parse_if_statement_after_zhe3(&mut iter)?.into_stmt_with_cond(IfCond::NotQi2))
        }
        lex::Lex::Ruo4Qi2Ran2Zhe3 => Ok(parse_if_statement_after_zhe3(&mut iter)?
            .into_stmt_with_cond(IfCond::Unary(UnaryIfExpr::Simple(OrQi2::Qi2)))),
        lex::Lex::Ruo4 => {
            // if_statement                : '若' if_expression '者' statement+ ('或若' if_expression '者' statement+)* ('若非' statement+)? FOR_IF_END ;
            let ifexpr = parse_ifexpression_plus_zhe3(&mut iter)?;
            let IfStmtAfterZhe3 {
                ifstmts,
                elseifcases,
                elsecase,
            } = parse_if_statement_after_zhe3(&mut iter)?;
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
        lex::Lex::Ming2Zhi1 => Ok(Statement::NameMulti {
            idents: parse_name_multi_statement_after_ming2zhi1(&mut iter)?,
        }),
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
            math: MathKind::ArithUnaryMath(OrQi2::from(&parse_ident_or_qi2(&mut iter)?)),
        }),
        lex::Lex::You3 => parse_init_define_statement_after_you3(&mut iter),
        lex::Lex::Heng2Wei2Shi4 => {
            let mut statements = vec![];
            loop {
                match **iter.peek().ok_or(Error::UnexpectedEOF)? {
                    lex::Lex::Yun2Yun2OrYe3(_) => {
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
        _ => Err(Error::SomethingWentWrong(here!())),
    }
}

fn parse_name_multi_statement_after_ming2zhi1(
    mut iter: &mut LexIter<'_>,
) -> Result<Vec<Identifier>, Error> {
    // ('曰' IDENTIFIER)+

    let mut idents = vec![];

    while let Some(lex::Lex::Yue1) = iter.peek() {
        iter.next();
        idents.push(parse_identifier(&mut iter)?);
    }

    if idents.is_empty() {
        return Err(Error::SomethingWentWrong(here!())); // we need at least one 曰 now that we have seen 名之
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
