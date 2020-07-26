use crate::identbimap;
use crate::lex;
use crate::parse;

type Line = (usize, String);

#[derive(Clone)]
struct Env {
    ans_counter: usize,
    rand_counter: usize,
    indent_level: usize,
    variables_not_yet_named: Vec<String>,
    ident_map: identbimap::IdentBiMap,
}

fn compile_optional_literal(
    env: &Env,
    lit: Option<&parse::Data>,
    default_type: lex::Type,
) -> String {
    match lit {
        None => match default_type {
            lex::Type::Shu4 => "0.0".to_string(),
            lex::Type::Lie4 => "vec![]".to_string(),
            lex::Type::Yan2 => "\"\"".to_string(),
            lex::Type::Yao2 => "false".to_string(),
        },
        Some(v) => compile_literal(&env, v),
    }
}

fn compile_literal(env: &Env, v: &parse::Data) -> String {
    match v.clone() {
        parse::Data::BoolValue(true) => "true".to_string(),
        parse::Data::BoolValue(false) => "false".to_string(),
        parse::Data::Identifier(ident) => env.ident_map.translate_from_hanzi(&ident),
        parse::Data::IntNum(intnum) => format!("{}.0", intnum),
        parse::Data::StringLiteral(strlit) => format!("\"{}\"", strlit), // FIXME properly escape
    }
}

/// It is possible to have three conflicting information on the number of variables declared.
/// Let's say we have `吾有三數。曰三。曰九。名之曰「庚」。曰「辛」。曰「壬」。曰「癸」。書之。`
/// Then `how_many_variables` is  `3`, `type_` is `Type::Shu4`, `data_arr` is `vec![3, 9]` and `idents` are the idents.
/// This compiles to
/// ```
/// var 庚 = 3;
/// var 辛 = 9;
/// var 壬 = 0;
/// console.log();
/// ```

/// `吾有三數。曰三。曰九。曰二十七。名之曰「甲」。書之。` becomes
/// ```
/// var 甲 = 3;
/// var _ans1 = 9;
/// var _ans2 = 27;
/// console.log(_ans1, _ans2);
/// ```

/// `吾有三數。曰三。曰九。曰二十七。名之曰「乙」。曰「丙」。書之。` is
/// ```
/// var 乙 = 3;
/// var 丙 = 9;
/// var _ans3 = 27;
/// console.log(_ans3);
/// ```

/// and `吾有三數。曰三。曰九。曰二十七。名之曰「丁」。曰「戊」。曰「己」。書之。` is, naturally,
/// ```
/// var 丁 = 3;
/// var 戊 = 9;
/// var 己 = 27;
/// console.log();
/// ```

/// Therefore, `how_many_variables` always determines how many variables are to be defined;
/// `data_arr` is truncated or padded so that its length matches `how_many_variables`,
/// `idents` fills the open spots,
/// and remaining spots (if any) will be accessible by 書之 .
fn compile_define(
    mut env: &mut Env,
    decl: &parse::DeclareStatement,
    idents: &[parse::Identifier],
) -> Vec<Line> {
    let parse::DeclareStatement {
        how_many_variables,
        type_,
        data_arr,
    } = decl;
    let mut ans = vec![];

    for i in 0..*how_many_variables {
        match idents.get(i) {
            None => {
                // no more ident; ans_counter and variables_not_yet_named come into play
                ans.push((
                    env.indent_level,
                    format!(
                        "let _ans{} = {};",
                        get_new_unnamed_var(&mut env),
                        compile_optional_literal(&env, data_arr.get(i), *type_)
                    ),
                ));
            }
            Some(ident) => {
                ans.push((
                    env.indent_level,
                    format!(
                        "let {}{} = {};",
                        ifmutable_thenmut(&env, &ident),
                        env.ident_map.translate_from_hanzi(&ident),
                        compile_optional_literal(&env, data_arr.get(i), *type_)
                    ),
                ));
            }
        }
    }

    ans
}

fn compile_forenum(mut env: &mut Env, num: i64, statements: &[parse::Statement]) -> Vec<Line> {
    let mut r = vec![(env.indent_level, format!("for _ in 0..{} {{", num,))];
    env.indent_level += 1;
    for st in statements {
        r.append(&mut compile_statement(&mut env, &st));
    }
    env.indent_level -= 1;
    r.push((env.indent_level, "}".to_string()));
    r
}

fn compile_dataorqi2(env: &mut Env, a: &parse::DataOrQi2) -> String {
    match a {
        parse::DataOrQi2::Qi2 => {
            let qi = env
                .variables_not_yet_named
                .last()
                .unwrap_or(&"f64::NAN".to_string())
                .to_string();

            //《文言陰符》曰『言「其」者。取至近之魚而棄其餘。』
            env.variables_not_yet_named = vec![];
            qi
        }
        parse::DataOrQi2::Data(data) => compile_literal(&env, &data),
    }
}

/// 吾有三數。曰三曰五曰二名之曰「甲」。加其以五。
/// is to be translated as
/// ```
/// var 甲 = 3;
/// var _ans1 = 5;
/// var _ans2 = 2;
/// const _ans3 = _ans2 + 5;
/// ```

/// 加其以五。書之。
/// is to be translated as
/// ```
/// const _ans1 = undefined + 5;
/// console.log(_ans1);
/// ```
/// Thus, when we do not have anything to reference, I must pad with `f64::NAN`

/// Both
/// 加一以三。加二以三。減其以其
/// and
/// 加一以三。加二以三。減其於其
/// are compiled to
/// ```
/// const _ans1 = 1 + 3;
/// const _ans2 = 2 + 3;
/// const _ans3 = _ans2 - undefined;
/// ```

fn compile_math(mut env: &mut Env, math: &parse::MathKind) -> Vec<Line> {
    match math {
        parse::MathKind::BooleanAlgebra(ident1, ident2, op) => {
            let data1 = parse::DataOrQi2::Data(parse::Data::Identifier(ident1.clone()));
            let data2 = parse::DataOrQi2::Data(parse::Data::Identifier(ident2.clone()));
            compile_math_binary(
                &mut env,
                op.to_str(),
                &data1,
                lex::Preposition::Yi3, /* whichever is fine */
                &data2,
            )
        }
        parse::MathKind::ArithBinaryMath(op, data1, prep, data2) => {
            compile_math_binary(&mut env, op.to_str(), &data1, *prep, &data2)
        }
        parse::MathKind::ModMath(op, data1, prep, data2) => {
            compile_math_binary(&mut env, op.to_str(), &data1, *prep, &data2)
        }
        parse::MathKind::ArithUnaryMath(data) => {
            let a = compile_dataorqi2(&mut env, data);
            let r = vec![(
                env.indent_level,
                format!("let _ans{} = !{};", get_new_unnamed_var(&mut env), a,),
            )];

            r
        }
    }
}

fn compile_math_binary(
    mut env: &mut Env,
    opstr: &str,
    data1: &parse::DataOrQi2,
    prep: lex::Preposition,
    data2: &parse::DataOrQi2,
) -> Vec<Line> {
    let left = compile_dataorqi2(
        &mut env,
        match prep {
            lex::Preposition::Yi3 => &data1,
            lex::Preposition::Yu2 => &data2,
        },
    );

    let right = compile_dataorqi2(
        &mut env,
        match prep {
            lex::Preposition::Yi3 => &data2,
            lex::Preposition::Yu2 => &data1,
        },
    );

    let r = vec![(
        env.indent_level,
        format!(
            "let _ans{} = {} {} {};",
            get_new_unnamed_var(&mut env),
            left,
            opstr,
            right,
        ),
    )];

    r
}

/// 加一以三。加六以九。名之曰「甲」。曰「乙」。
/// is to be translated as
/// ```
/// const _ans1 = 1 + 3;
/// const _ans2 = 6 + 9;
/// var JIA3 = _ans1;
/// var YI3 = _ans2;
/// ```

/// 加二以三。加一以三。加三以三。名之曰「甲」。名之曰「乙」。書之
/// is to be translated as
/// ```
/// const _ans1 = 2 + 3;
/// const _ans2 = 1 + 3;
/// const _ans3 = 3 + 3;
/// var JIA3 = _ans3;
/// var YI3 = _ans2;
/// console.log(_ans1);
/// ```
/// That is, [_ans1, _ans2, _ans3] is matched from the end by the first 名之曰,
/// leaving [_ans1, _ans2]; then, this is matched from the end by the second 名之曰,
/// leaving [_ans1].

fn compile_name_multi_statement(mut env: &mut Env, idents: &[parse::Identifier]) -> Vec<Line> {
    let mut res = vec![];
    for i in 0..idents.len() {
        res.push((
            env.indent_level,
            if env.variables_not_yet_named.len() + i < idents.len() {
                // negative index is to be filled with undefined
                format!(
                    "let {}{} : (); // undefined",
                    ifmutable_thenmut(&env, &idents[i]),
                    env.ident_map.translate_from_hanzi(&idents[i])
                )
            } else {
                let tmpvarname = env.variables_not_yet_named
                    [env.variables_not_yet_named.len() + i - idents.len()]
                .clone();

                format!(
                    "let {}{} = {};",
                    ifmutable_thenmut(&env, &idents[i]),
                    env.ident_map.translate_from_hanzi(&idents[i]),
                    tmpvarname.clone()
                )
            },
        ));
    }
    if env.variables_not_yet_named.len() > idents.len() {
        env.variables_not_yet_named
            .truncate(env.variables_not_yet_named.len() - idents.len());
    } else {
        env.variables_not_yet_named = vec![]
    }

    res
}

fn compile_rvalue_noqi2(
    mut env: &mut Env,
    rv: &parse::RvalueNoQi2,
    paren_when_casted: bool,
) -> String {
    match rv {
        parse::RvalueNoQi2::Simple(d) => {
            compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(d.clone()))
        }
        parse::RvalueNoQi2::Length(d) => {
            if paren_when_casted {
                format!(
                    "({}.len() as f64)",
                    compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(d.clone()))
                )
            } else {
                format!(
                    "{}.len() as f64",
                    compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(d.clone()))
                )
            }
        }
        parse::RvalueNoQi2::Index(d, ind) => format!(
            "{}[{} - 1]",
            compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(d.clone())),
            ind
        ),
        parse::RvalueNoQi2::IndexByIdent(d, ident) => format!(
            "{}[({} as usize) - 1]",
            compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(d.clone())),
            env.ident_map.translate_from_hanzi(&ident)
        ),
    }
}

fn compile_unaryifexpr(mut env: &mut Env, unary: &parse::UnaryIfExpr) -> String {
    match unary {
        parse::UnaryIfExpr::Simple(data1) => compile_dataorqi2(&mut env, data1),
        parse::UnaryIfExpr::Complex(rv) => compile_rvalue_noqi2(&mut env, &rv, true),
    }
}

fn compile_ifcond(mut env: &mut Env, ifcond: &parse::IfCond, keyword: &str) -> Line {
    match ifcond {
        parse::IfCond::Binary(data1, op, data2) => (
            env.indent_level,
            format!(
                "{} {} {} {} {{",
                keyword,
                compile_unaryifexpr(&mut env, data1),
                op.to_str(),
                compile_unaryifexpr(&mut env, data2),
            ),
        ),
        parse::IfCond::Unary(data1) => (
            env.indent_level,
            format!("{} {} {{", keyword, compile_unaryifexpr(&mut env, data1)),
        ),
        parse::IfCond::NotQi2 => (
            env.indent_level,
            format!(
                "{} !{} {{",
                keyword,
                compile_dataorqi2(&mut env, &parse::DataOrQi2::Qi2),
            ),
        ),
    }
}

fn compile_if(
    mut env: &mut Env,
    ifcase: &parse::CondPlusStatements,
    elseifcases: &[parse::CondPlusStatements],
    elsecase: &[parse::Statement],
) -> Vec<Line> {
    let (ifcond, ifstmts) = ifcase;
    let mut r = vec![compile_ifcond(&mut env, ifcond, "if")];
    compile_indent(&mut env, &mut r, &ifstmts);
    for (elseifcond, elseifstmts) in elseifcases {
        r.push(compile_ifcond(&mut env, elseifcond, "} else if"));
        compile_indent(&mut env, &mut r, &elseifstmts)
    }

    if !elsecase.is_empty() {
        r.push((env.indent_level, "} else {".to_string()));
        compile_indent(&mut env, &mut r, &elsecase)
    }
    r.push((env.indent_level, "}".to_string()));
    r
}

fn get_new_unnamed_var(mut env: &mut Env) -> usize {
    env.ans_counter += 1;
    env.variables_not_yet_named
        .push(format!("_ans{}", env.ans_counter));
    env.ans_counter
}

fn compile_statement(mut env: &mut Env, st: &parse::Statement) -> Vec<Line> {
    match st {
        parse::Statement::ArrayCat {
            append_to: parse::IdentOrQi2::Ident(ident),
            elems,
        } => vec![(
            env.indent_level,
            format!(
                "let _ans{} = [&{}[..], {}].concat();",
                get_new_unnamed_var(&mut env),
                env.ident_map.translate_from_hanzi(&ident),
                elems
                    .iter()
                    .map(|e| format!("&{}[..]", env.ident_map.translate_from_hanzi(&e)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )],
        parse::Statement::Continue => vec![(env.indent_level, "continue;".to_string())],
        parse::Statement::Break => vec![(env.indent_level, "break;".to_string())],
        parse::Statement::ArrayFill {
            what_to_fill: parse::IdentOrQi2::Ident(ident),
            elems,
        } => vec![(
            env.indent_level,
            if let [e] = elems.as_slice() {
                format!(
                    "{}.push({});",
                    env.ident_map.translate_from_hanzi(&ident),
                    compile_literal(&env, e)
                )
            } else {
                format!(
                    "{}.append(&mut vec![{}]);",
                    env.ident_map.translate_from_hanzi(&ident),
                    elems
                        .iter()
                        .map(|e| compile_literal(&env, e))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
        )],
        parse::Statement::ArrayFill {
            what_to_fill: parse::IdentOrQi2::Qi2,
            elems: _,
        }
        | parse::Statement::ArrayCat {
            append_to: parse::IdentOrQi2::Qi2,
            elems: _,
        } => unimplemented!("filling qi2"),
        parse::Statement::If {
            ifcase,
            elseifcases,
            elsecase,
        } => compile_if(&mut env, ifcase, elseifcases, elsecase),
        parse::Statement::Reference { rvalue } => vec![(
            env.indent_level,
            format!(
                "let _ans{} = {};",
                get_new_unnamed_var(&mut env),
                compile_rvalue_noqi2(&mut env, rvalue, false)
            ),
        )],

        parse::Statement::NameMulti { idents } => compile_name_multi_statement(&mut env, &idents),
        parse::Statement::Flush => {
            env.variables_not_yet_named = vec![];
            vec![]
        }
        parse::Statement::Math { math } => compile_math(&mut env, math),
        parse::Statement::ReferenceWhatIsLeft { data } => vec![(
            env.indent_level,
            format!(
                "let  _ans{} = &{}[1..].to_vec();",
                get_new_unnamed_var(&mut env),
                compile_dataorqi2(&mut env, &parse::DataOrQi2::Data(data.clone()))
            ),
        )],
        parse::Statement::Declare(parse::DeclareStatement {
            how_many_variables,
            type_,
            data_arr,
        }) => (0..*how_many_variables)
            .map(|i| {
                (
                    env.indent_level,
                    format!(
                        "let _ans{} = {};",
                        get_new_unnamed_var(&mut env),
                        compile_optional_literal(&env, data_arr.get(i), *type_)
                    ),
                )
            })
            .collect(),
        parse::Statement::Print => {
            let r = format!(
                "println!(\"{}\"{});",
                "{} ".repeat(env.variables_not_yet_named.len()).trim_end(),
                env.variables_not_yet_named
                    .iter()
                    .map(|varname| format!(", {}", varname))
                    .collect::<Vec<_>>()
                    .join("")
            );
            env.variables_not_yet_named = vec![];
            return vec![(env.indent_level, r)];
        }
        parse::Statement::Assignment { lvalue, rvalue } => vec![(
            env.indent_level,
            format!(
                "{} = {};",
                compile_lvalue(&env, lvalue),
                compile_rvalue(&mut env, rvalue),
            ),
        )],
        parse::Statement::InitDefine { type_, data, name } => vec![(
            env.indent_level,
            format!(
                "let {}{} = {};",
                ifmutable_thenmut(&env, &name),
                env.ident_map.translate_from_hanzi(&name),
                compile_optional_literal(&env, Some(data), *type_)
            ),
        )],
        parse::Statement::Define { decl, idents } => compile_define(&mut env, decl, &idents),
        parse::Statement::ForEnum { num, statements } => {
            compile_forenum(&mut env, *num, &statements)
        }
        parse::Statement::ForEnumIdent { ident, statements } => {
            compile_forenum_ident(&mut env, ident, statements)
        }
        parse::Statement::ForArr { list, elem, stmts } => {
            let mut r = vec![(
                env.indent_level,
                format!(
                    "for {} in {} {{",
                    env.ident_map.translate_from_hanzi(&elem),
                    env.ident_map.translate_from_hanzi(list)
                ),
            )];
            compile_indent(&mut env, &mut r, &stmts);
            r.push((env.indent_level, "}".to_string()));
            r
        }
        parse::Statement::Loop { statements } => compile_loop(&mut env, statements),
    }
}

fn compile_indent(mut env: &mut Env, r: &mut Vec<Line>, stmts: &[parse::Statement]) {
    env.indent_level += 1;
    for st in stmts {
        r.append(&mut compile_statement(&mut env, &st));
    }
    env.indent_level -= 1;
}

fn compile_lvalue(env: &Env, lvalue: &parse::Lvalue) -> String {
    match lvalue {
        parse::Lvalue::Index(ident, index) => format!(
            "{}[{} - 1]",
            env.ident_map.translate_from_hanzi(&ident),
            index
        ),
        parse::Lvalue::Simple(ident) => env.ident_map.translate_from_hanzi(&ident),
        parse::Lvalue::IndexByIdent(ident, index) => format!(
            "{}[({} as usize) - 1]",
            env.ident_map.translate_from_hanzi(&ident),
            env.ident_map.translate_from_hanzi(&index),
        ),
    }
}

fn compile_rvalue(mut env: &mut Env, rvalue: &parse::Rvalue) -> String {
    match rvalue {
        parse::Rvalue::Index(data, index) => {
            format!("{}[{} - 1]", compile_dataorqi2(&mut env, data), index)
        }
        parse::Rvalue::Simple(data) => compile_dataorqi2(&mut env, data),
        parse::Rvalue::Length(data) => {
            format!("({}.len() as f64)", compile_dataorqi2(&mut env, data))
        }
    }
}

fn ifmutable_thenmut(env: &Env, name: &parse::Identifier) -> &'static str {
    if env.ident_map.is_mutable(&name) {
        "mut "
    } else {
        ""
    }
}

fn compile_forenum_ident(
    mut env: &mut Env,
    ident: &parse::IdentOrQi2,
    statements: &[parse::Statement],
) -> Vec<Line> {
    env.rand_counter += 1;
    let rand_n = env.rand_counter;
    let mut r = vec![
        (env.indent_level, format!("let mut _rand{} = 0.0;", rand_n,)),
        (
            env.indent_level,
            format!(
                "while _rand{} < {} {{",
                rand_n,
                compile_dataorqi2(&mut env, &parse::DataOrQi2::from(ident)),
            ),
        ),
    ];
    compile_indent(&mut env, &mut r, statements);
    r.append(&mut vec![
        (env.indent_level + 1, format!("_rand{} += 1.0;", rand_n,)),
        (env.indent_level, "}".to_string()),
    ]);
    r
}

fn compile_loop(mut env: &mut Env, statements: &[parse::Statement]) -> Vec<Line> {
    let mut r = vec![(env.indent_level, "loop {".to_string())];
    compile_indent(&mut env, &mut r, statements);
    r.push((env.indent_level, "}".to_string()));
    r
}

use std::collections::HashMap;
pub fn compile(parsed: &[parse::Statement], conversion_table: &HashMap<String, String>) -> String {
    let mut ans = vec![(0, "fn main() {".to_string())];
    let mut env = Env {
        ans_counter: 0,
        rand_counter: 0,
        indent_level: 1,
        variables_not_yet_named: vec![],
        ident_map: identbimap::IdentBiMap::new(&parsed, &conversion_table),
    };

    for st in parsed {
        ans.append(&mut compile_statement(&mut env, &st));
    }

    ans.push((0, "}".to_string()));

    ans.iter()
        .map(|(indent, src)| format!("{}{}\n", "    ".repeat(*indent), src))
        .collect::<Vec<_>>()
        .join("")
}
