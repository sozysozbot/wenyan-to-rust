#![warn(clippy::pedantic)]
use std::fs::File;
use std::io::prelude::*;
mod lex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test000.wy")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("src: \n----------------------");
    println!("{}", contents);
    println!("----------------------");
    println!("\nlexer output: \n----------------------");
    println!("{:?}", lex::lex(&contents));
    println!("----------------------");

    Ok(())
}
