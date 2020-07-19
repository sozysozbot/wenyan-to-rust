use crate::lex;
use crate::parse;
struct Env {
    anon_counter: usize,
    indent_level: usize,
    shu1zhi1_reference: Vec<String>,
}

fn compile_literal(lit: Option<&parse::Data>, default_type: lex::Type) -> String {
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
            parse::Data::Identifier(_) => unimplemented!(),
            parse::Data::IntNum(intnum) => format!("{}.0", intnum),
            parse::Data::StringLiteral(strlit) => format!("\"{}\"", strlit), // FIXME properly escape
        },
    }
}

fn compile_statement(env: &mut Env, st: &parse::Statement) -> String {
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
                    compile_literal(data_arr.get(i), *type_)
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
    }

    ans
}

pub fn compile(parsed: &Vec<parse::Statement>) -> String {
    let mut ans = "fn main() {\n".to_string();
    let mut env = Env {
        anon_counter: 0,
        indent_level: 1,
        shu1zhi1_reference: vec![],
    };

    for st in parsed {
        ans.push_str(&compile_statement(&mut env, &st));
    }

    ans.push_str(r#"}"#);
    ans.push_str("\n");

    ans
}
