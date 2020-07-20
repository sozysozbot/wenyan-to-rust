use crate::identbimap;
use crate::lex;
use crate::parse;
#[derive(Clone)]
struct Env {
    ans_counter: usize,
    rand_counter: usize,
    indent_level: usize,
    shu1zhi1_reference: Vec<String>,
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
            lex::Type::Lie4 => unimplemented!(),
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
    env: &mut Env,
    decl: &parse::DeclareStatement,
    idents: &Vec<parse::Identifier>,
) -> String {
    let parse::DeclareStatement {
        how_many_variables,
        type_,
        data_arr,
    } = decl;
    let mut ans = String::new();

    let mut new_shu1zhi1 = vec![];
    for i in 0..*how_many_variables {
        match idents.get(i) {
            None => {
                // no more ident; ans_counter and shu1zhi1_reference come into play
                env.ans_counter += 1;
                ans.push_str(&format!(
                    "{}let _ans{} = {};\n",
                    "    ".repeat(env.indent_level),
                    env.ans_counter,
                    compile_optional_literal(&env, data_arr.get(i), *type_)
                ));
                new_shu1zhi1.push(format!("_ans{}", env.ans_counter));
            }
            Some(ident) => {
                ans.push_str(&format!(
                    "{}let {}{} = {};\n",
                    "    ".repeat(env.indent_level),
                    if env.ident_map.is_mutable(&ident) {
                        "mut "
                    } else {
                        ""
                    },
                    env.ident_map.translate_from_hanzi(&ident),
                    compile_optional_literal(&env, data_arr.get(i), *type_)
                ));
            }
        }
    }
    env.shu1zhi1_reference = new_shu1zhi1;

    ans
}

fn compile_dataorqi2(env: &Env, data1: &parse::DataOrQi2) -> String {
    match data1 {
        parse::DataOrQi2::Qi2 => env
            .shu1zhi1_reference
            .last()
            .unwrap_or(&"f64::NAN".to_string())
            .to_string(),
        parse::DataOrQi2::Data(data) => compile_literal(&env, &data),
    }
}

fn compile_statement(mut env: &mut Env, st: &parse::Statement) -> String {
    let mut ans = String::new();
    match st {
        parse::Statement::Math {
            math: parse::MathKind::ArithBinaryMath(op, data1, prep, data2),
        } => {
            // 吾有三數。曰三曰五曰二名之曰「甲」。加其以五。
            // is to be translated as
            // ```
            // var 甲 = 3;
            // var _ans1 = 5;
            // var _ans2 = 2;
            // const _ans3 = _ans2 + 5;
            // ```

            // 加其以五。書之。
            // is to be translated as
            // ```
            // const _ans1 = undefined + 5;
            // console.log(_ans1);
            // ```

            match prep {
                &lex::Preposition::Yi3 => {
                    env.ans_counter += 1;
                    ans.push_str(&format!(
                        "{}let _ans{} = {} {} {};\n",
                        "    ".repeat(env.indent_level),
                        env.ans_counter,
                        compile_dataorqi2(&env, data1),
                        op.to_str(),
                        compile_dataorqi2(&env, data2),
                    ));
                    env.shu1zhi1_reference = vec![format!("_ans{}", env.ans_counter)];
                }
                &lex::Preposition::Yu2 => {
                    env.ans_counter += 1;
                    ans.push_str(&format!(
                        "{}let _ans{} = {} {} {};\n",
                        "    ".repeat(env.indent_level),
                        env.ans_counter,
                        compile_dataorqi2(&env, data2),
                        op.to_str(),
                        compile_dataorqi2(&env, data1),
                    ));
                    env.shu1zhi1_reference = vec![format!("_ans{}", env.ans_counter)];
                }
            }
        }
        parse::Statement::Declare(parse::DeclareStatement {
            how_many_variables,
            type_,
            data_arr,
        }) => {
            let mut new_shu1zhi1 = vec![];
            for i in 0..*how_many_variables {
                env.ans_counter += 1;
                ans.push_str(&format!(
                    "{}let _ans{} = {};\n",
                    "    ".repeat(env.indent_level),
                    env.ans_counter,
                    compile_optional_literal(&env, data_arr.get(i), *type_)
                ));
                new_shu1zhi1.push(format!("_ans{}", env.ans_counter));
            }
            env.shu1zhi1_reference = new_shu1zhi1
        }
        parse::Statement::Print => {
            ans = format!(
                "{}println!(\"{}\"",
                "    ".repeat(env.indent_level),
                "{} ".repeat(env.shu1zhi1_reference.len()).trim_end(),
            );

            for varname in &env.shu1zhi1_reference {
                ans.push_str(", ");
                ans.push_str(varname);
            }

            ans.push_str(");\n");
            env.shu1zhi1_reference = vec![];
        }
        parse::Statement::Assign { ident, data } => {
            ans = format!(
                "{}{} = {};\n",
                "    ".repeat(env.indent_level),
                env.ident_map.translate_from_hanzi(&ident),
                compile_literal(&env, data)
            )
        }
        parse::Statement::InitDefine { type_, data, name } => {
            ans = format!(
                "{}let {}{} = {};\n",
                "    ".repeat(env.indent_level),
                if env.ident_map.is_mutable(&name) {
                    "mut "
                } else {
                    ""
                },
                env.ident_map.translate_from_hanzi(&name),
                compile_optional_literal(&env, Some(data), *type_)
            );
            env.shu1zhi1_reference = vec![];
        }
        parse::Statement::Define { decl, idents } => {
            ans = compile_define(&mut env, decl, &idents);
        }
        parse::Statement::ForEnum { num, statements } => {
            let mut inner = String::new();
            let mut new_env = Env {
                indent_level: env.indent_level + 1,
                rand_counter: env.rand_counter,
                ans_counter: env.ans_counter,
                ident_map: env.ident_map.clone(),

                /// shu1zhi1_reference must be inherited, since in the original compiler
                ///
                /// ```
                /// 吾有二言。曰「「天地。」」。
                /// 為是三遍。
                /// 書之。
                /// 吾有一言。曰「「問天地好在。」」。書之。
                /// 云云。
                /// ```
                ///
                /// is translated into
                ///
                /// ```
                /// var _ans1 = "天地。";
                /// var _ans2 = "";
                /// for (let _rand1 = 0; _rand1 < 3; _rand1++) {
                ///   console.log(_ans1, _ans2);
                ///   var _ans3 = "問天地好在。";
                ///   console.log(_ans3);
                /// };
                /// ```
                shu1zhi1_reference: env.shu1zhi1_reference.clone(),
            };
            for st in statements {
                inner.push_str(&compile_statement(&mut new_env, &st));
            }
            ans = format!(
                "{}for _ in 0..{} {{\n{}{}}}\n",
                "    ".repeat(env.indent_level),
                num,
                inner,
                "    ".repeat(env.indent_level),
            );
        }

        parse::Statement::ForEnumIdent { ident, statements } => {
            let mut inner = String::new();
            env.rand_counter += 1;
            let rand_n = env.rand_counter;
            let mut new_env = Env {
                indent_level: env.indent_level + 1,
                ans_counter: env.ans_counter,
                rand_counter: env.rand_counter,
                ident_map: env.ident_map.clone(),

                // shu1zhi1_reference must be inherited
                shu1zhi1_reference: env.shu1zhi1_reference.clone(),
            };
            for st in statements {
                inner.push_str(&compile_statement(&mut new_env, &st));
            }

            ans = format!(
                "{}let mut _rand{} = 0.0;\n{}while _rand{} < {} {{\n{}{}_rand{} += 1.0;\n{}}}\n",
                "    ".repeat(env.indent_level),
                rand_n,
                "    ".repeat(env.indent_level),
                rand_n,
                env.ident_map.translate_from_hanzi(&ident),
                inner,
                "    ".repeat(env.indent_level + 1),
                rand_n,
                "    ".repeat(env.indent_level),
            );
        }
    }

    ans
}

use std::collections::HashMap;
pub fn compile(
    parsed: &Vec<parse::Statement>,
    conversion_table: &HashMap<String, String>,
) -> String {
    let mut ans = "fn main() {\n".to_string();
    let mut env = Env {
        ans_counter: 0,
        rand_counter: 0,
        indent_level: 1,
        shu1zhi1_reference: vec![],
        ident_map: identbimap::IdentBiMap::new(&parsed, &conversion_table),
    };

    for st in parsed {
        ans.push_str(&compile_statement(&mut env, &st));
    }

    ans.push_str(r#"}"#);
    ans.push_str("\n");

    ans
}
