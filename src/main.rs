#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Statement {
    Declare,
    Define,
    Print,
    For,
    Function,
    If,
    Return,
    Math,
    Assign,
    Import,
    Object,
    Reference,
    Array,
    Flush,
    Break,
    Comment,
}

fn compile(input: &str) -> Vec<Statement> {
    let mut ans = vec![];

    ans
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test001.wy")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("src: \n----------------------");
    println!("{}", contents);
    println!("----------------------");
    println!("\noutput: \n----------------------");
    for st in compile(&contents) {
        println!("{:?}", st);
    }
    println!("----------------------");

    Ok(())
}
