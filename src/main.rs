#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::prelude::*;
mod lex;
mod parse;
mod compile;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs::File;
    fn test(s: &str) {
        let mut file = File::open(format!("{}.wy", s)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let lex = lex::lex(&contents).unwrap();
        let parsed = parse::parse(&lex).unwrap();
        let compiled = compile::compile(&parsed);

        let mut file2 = File::open(format!("{}.rs", s)).unwrap();
        let mut contents2 = String::new();
        file2.read_to_string(&mut contents2).unwrap();

        assert_eq!(compiled, contents2)
    }

    #[test]
    fn test000() {
        test("test000")
    }

    #[test]
    fn test001() {
        test("test001")
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
        let parsed = parse::parse(&lex);
        println!("\nparser output: \n----------------------");
        println!("{:?}", parsed);
        println!("----------------------");
        if let Ok(parsed) = parsed {
            let compiled = compile::compile(&parsed);
            println!("\ncompiler output: \n----------------------");
            println!("{}", compiled);
            println!("----------------------");
        }
    }
    Ok(())
}
