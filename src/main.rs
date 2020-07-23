#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
use std::fs::File;
use std::io::prelude::*;
extern crate clap;
use clap::{App, Arg};
mod compile;
mod identbimap;
mod lex;
mod parse;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::include_str;
fn main() -> std::io::Result<()> {
    let matches = App::new("wenyan-to-rust")
        .version("0.1.0")
        .author("jekto.vatimeliju <jekto.vatimeliju@gmail.com>")
        .about("Tries to convert wenyan to rust")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // let config = matches.value_of("config").unwrap_or("default.conf");

    let pinyin_json = include_str!("hanzi2roman-map-pinyin.json");
    let conversion_table: HashMap<String, String> = serde_json::from_str(pinyin_json)?;

    let verbose_level = matches.occurrences_of("v");

    let mut file = File::open(matches.value_of("INPUT").unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if verbose_level > 0 {
        println!("src: \n----------------------");
        println!("{}", contents);
        println!("----------------------");
    }

    let lex = lex::lex(&contents);
    if verbose_level > 0 || lex.is_err() {
        println!("\nlexer output: \n----------------------");
        println!("{:?}", lex);
        println!("----------------------");
    }

    if let Ok(lex) = lex {
        let parsed = parse::parse(&lex);
        if verbose_level > 0 || parsed.is_err() {
            println!("\nparser output: \n----------------------");
            println!("{:?}", parsed);
            println!("----------------------");
        }
        if let Ok(parsed) = parsed {
            let compiled = compile::compile(&parsed, &conversion_table);
            if verbose_level > 0 {
                println!("\ncompiler output: \n----------------------");
            }
            print!("{}", compiled);
            if verbose_level > 0 {
                println!("----------------------");
            }
        }
    }
    Ok(())
}
