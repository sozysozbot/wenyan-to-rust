#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::prelude::*;
mod lex;
mod parse;
mod compile;


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
