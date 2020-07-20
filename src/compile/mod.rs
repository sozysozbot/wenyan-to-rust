use crate::lex;
use crate::parse;
#[derive(Clone)]
struct Env {
    anon_counter: usize,
    indent_level: usize,
    shu1zhi1_reference: Vec<String>,
}

fn compile_literal(
    lit: Option<&parse::Data>,
    default_type: lex::Type,
    conversion_table: &HashMap<String, String>,
) -> String {
    match lit {
        None => match default_type {
            lex::Type::Shu4 => "0.0".to_string(),
            lex::Type::Lie4 => unimplemented!(),
            lex::Type::Yan2 => "\"\"".to_string(),
            lex::Type::Yao2 => "false".to_string(),
        },
        Some(v) => match v.clone() {
            parse::Data::BoolValue(true) => "true".to_string(),
            parse::Data::BoolValue(false) => "false".to_string(),
            parse::Data::Identifier(ident) => to_pinyin(ident, &conversion_table),
            parse::Data::IntNum(intnum) => format!("{}.0", intnum),
            parse::Data::StringLiteral(strlit) => format!("\"{}\"", strlit), // FIXME properly escape
        },
    }
}

fn compile_statement(
    env: &mut Env,
    st: &parse::Statement,
    conversion_table: &HashMap<String, String>,
) -> String {
    let mut ans = String::new();
    match st {
        parse::Statement::Declare(parse::DeclareStatement {
            int_num,
            type_,
            data_arr,
        }) => {
            let mut new_shu1zhi1 = vec![];
            for i in 0..*int_num {
                env.anon_counter += 1;
                ans.push_str(&format!(
                    "{}let _ans{} = {};\n",
                    "    ".repeat(env.indent_level),
                    env.anon_counter,
                    compile_literal(data_arr.get(i), *type_, &conversion_table)
                ));
                new_shu1zhi1.push(format!("_ans{}", env.anon_counter));
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
        parse::Statement::InitDefine { type_, data, name } => {
            ans = format!(
                "{}let {} = {};\n",
                "    ".repeat(env.indent_level),
                to_pinyin(name.clone(), &conversion_table),
                compile_literal(Some(data), *type_, &conversion_table)
            );
            env.shu1zhi1_reference = vec![];
        }
        parse::Statement::ForEnum { num, statements } => {
            let mut inner = String::new();
            let mut new_env = Env {
                indent_level: env.indent_level + 1,
                anon_counter: env.anon_counter,

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
                inner.push_str(&compile_statement(&mut new_env, &st, &conversion_table));
            }
            ans = format!(
                "{}for _ in 0..{} {{\n{}{}}}\n",
                "    ".repeat(env.indent_level),
                num,
                inner,
                "    ".repeat(env.indent_level),
            );
        }
    }

    ans
}

fn to_pinyin(ident: parse::Identifier, conversion_table: &HashMap<String, String>) -> String {
    let parse::Identifier(i) = ident;
    let vec = i
        .chars()
        .map(|c| {
            conversion_table
                .get(&format!("{:X}", c as u32).to_string())
                .unwrap_or(&"_".to_string())
                .to_string()
        })
        .collect::<Vec<_>>();
    vec.join("")
}
use std::collections::HashMap;
pub fn compile(
    parsed: &Vec<parse::Statement>,
    conversion_table: &HashMap<String, String>,
) -> String {
    let mut ans = "fn main() {\n".to_string();
    let mut env = Env {
        anon_counter: 0,
        indent_level: 1,
        shu1zhi1_reference: vec![],
    };

    for st in parsed {
        ans.push_str(&compile_statement(&mut env, &st, &conversion_table));
    }

    ans.push_str(r#"}"#);
    ans.push_str("\n");

    ans
}
