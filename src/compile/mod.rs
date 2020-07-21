use crate::identbimap;
use crate::lex;
use crate::parse;
#[derive(Clone)]
struct Env {
    ans_counter: usize,
    rand_counter: usize,
    indent_level: usize,
    shu1zhi1_reference: Vec<String>,
    ming2zhi1_reference: Vec<String>,
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
    env.shu1zhi1_reference = new_shu1zhi1.clone();
    env.ming2zhi1_reference = new_shu1zhi1;

    ans
}

fn compile_forenum(env: &Env, num: i64, statements: &[parse::Statement]) -> String {
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
        ming2zhi1_reference: env.ming2zhi1_reference.clone()
    };
    for st in statements {
        inner.push_str(&compile_statement(&mut new_env, &st));
    }
    return format!(
        "{}for _ in 0..{} {{\n{}{}}}\n",
        "    ".repeat(env.indent_level),
        num,
        inner,
        "    ".repeat(env.indent_level),
    );
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


fn compile_math(mut env: &mut Env, math: &parse::MathKind, idents: &[parse::Identifier]) -> String {
    fn compile_dataorqi2(env: &mut Env, a: &parse::DataOrQi2) -> String {
        match a {
            parse::DataOrQi2::Qi2 => {
                let qi = env
                .shu1zhi1_reference
                .last()
                .unwrap_or(&"f64::NAN".to_string())
                .to_string();

                //《文言陰符》曰『言「其」者。取至近之魚而棄其餘。』
                env.shu1zhi1_reference = vec![];
                qi
            },
            parse::DataOrQi2::Data(data) => compile_literal(&env, &data),
        }
    }

    let parse::MathKind::ArithBinaryMath(op, data1, prep, data2) = math;

    let left = compile_dataorqi2(&mut env, match prep {
        lex::Preposition::Yi3 => data1,
        lex::Preposition::Yu2 => data2,
    });
    
    let right = compile_dataorqi2(&mut env, match prep {
        lex::Preposition::Yi3 => data2,
        lex::Preposition::Yu2 => data1,
    });

    env.ans_counter += 1;
    let r = format!(
        "{}let _ans{} = {} {} {};\n",
        "    ".repeat(env.indent_level),
        env.ans_counter,
        left,
        op.to_str(),
        right,
    );
    env.ming2zhi1_reference.push(format!("_ans{}", env.ans_counter));
    env.shu1zhi1_reference = vec![format!("_ans{}", env.ans_counter)];

    if idents.is_empty() {
        return r;
    } else if idents.len() > env.ming2zhi1_reference.len() {
        return "########poisoning the output########\nhaving more identifiers than there are values results in a mysterious compilation in the original implementation, which I do not intend to implement for now\n####################################".to_string()
    } else {
        let mut res = r;
        for i in 0..idents.len() {
            let tmpvarname = env.ming2zhi1_reference[env.ming2zhi1_reference.len() - idents.len() + i].clone();
            res.push_str(&format!("{}let {}{} = {};\n",
            "    ".repeat(env.indent_level),
            if env.ident_map.is_mutable(&idents[i]) {
                "mut "
            } else {
                ""
            },
            env.ident_map.translate_from_hanzi(&idents[i]),
            tmpvarname.clone()
            ));
        }
        env.ming2zhi1_reference.truncate(env.ming2zhi1_reference.len() - idents.len());
        env.shu1zhi1_reference = env.ming2zhi1_reference.clone();
        return res;
    }
}

fn compile_statement(mut env: &mut Env, st: &parse::Statement) -> String {
    match st {
        parse::Statement::Math { math, name_multi } => {
            return compile_math(&mut env, math, &name_multi);
        }
        parse::Statement::Declare(parse::DeclareStatement {
            how_many_variables,
            type_,
            data_arr,
        }) => {
            let mut r = String::new();
            let mut new_shu1zhi1 = vec![];
            for i in 0..*how_many_variables {
                env.ans_counter += 1;
                r.push_str(&format!(
                    "{}let _ans{} = {};\n",
                    "    ".repeat(env.indent_level),
                    env.ans_counter,
                    compile_optional_literal(&env, data_arr.get(i), *type_)
                ));
                new_shu1zhi1.push(format!("_ans{}", env.ans_counter));
            }
            env.shu1zhi1_reference = new_shu1zhi1;
            return r;
        }
        parse::Statement::Print => {
            let mut r = format!(
                "{}println!(\"{}\"",
                "    ".repeat(env.indent_level),
                "{} ".repeat(env.shu1zhi1_reference.len()).trim_end(),
            );

            for varname in &env.shu1zhi1_reference {
                r.push_str(", ");
                r.push_str(varname);
            }

            r.push_str(");\n");
            env.shu1zhi1_reference = vec![];
            env.ming2zhi1_reference = vec![];
            return r;
        }
        parse::Statement::Assign { ident, data } => {
            return format!(
                "{}{} = {};\n",
                "    ".repeat(env.indent_level),
                env.ident_map.translate_from_hanzi(&ident),
                compile_literal(&env, data)
            )
        }
        parse::Statement::InitDefine { type_, data, name } => {
            let r = format!(
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
            // must inherit shu1zhi1_reference
            // example:
            // ```
            // 吾有一數。曰四。 減其於七。
            // 有數二。名之曰「作c」。書之。
            // ```
            // is compiled to
            // ```
            // var _ans1 = 3;
            // const _ans2 = _ans1 + 5;
            // const _ans3 = _ans2 - 7;
            // var _ans4 = 3;
            // const _ans5 = 2 + _ans4;
            // const _ans6 = 8 - _ans5;
            // var _ans7 = 3;
            // var _ans8 = 0;
            // const _ans9 = _ans8 - 7;
            // var ZUO4_ = 3;
            // console.log(_ans9);
            // var ZUO4__ = 5;
            // console.log();
            // ```
            return r;
        }
        parse::Statement::Define { decl, idents } => {
            return compile_define(&mut env, decl, &idents);
        }
        parse::Statement::ForEnum { num, statements } => {
            return compile_forenum(&env, *num, &statements);
        }
        parse::Statement::ForEnumIdent { ident, statements } => {
            return compile_forenum_ident(&mut env, ident, statements);
        }
    }
}

fn compile_forenum_ident(
    env: &mut Env,
    ident: &parse::Identifier,
    statements: &[parse::Statement],
) -> String {
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
        ming2zhi1_reference: env.ming2zhi1_reference.clone()
    };
    for st in statements {
        inner.push_str(&compile_statement(&mut new_env, &st));
    }

    return format!(
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
        ming2zhi1_reference: vec![],
        ident_map: identbimap::IdentBiMap::new(&parsed, &conversion_table),
    };

    for st in parsed {
        ans.push_str(&compile_statement(&mut env, &st));
    }

    ans.push_str(r#"}"#);
    ans.push_str("\n");

    ans
}
