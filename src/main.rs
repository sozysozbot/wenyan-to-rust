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
mod tests {
    use super::*;
    use serde_json;
    use std::fs::File;
    fn test(s: &str) {
        let pinyin_json = include_str!("hanzi2roman-map-pinyin.json");
        let conversion_table: HashMap<String, String> = serde_json::from_str(pinyin_json).unwrap();

        let mut file = File::open(format!("{}.wy", s)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let lex = lex::lex(&contents).unwrap();
        let parsed = parse::parse(&lex).unwrap();
        let compiled = compile::compile(&parsed, &conversion_table);

        let mut file2 = File::open(format!("{}.rs", s)).unwrap();
        let mut contents2 = String::new();
        file2.read_to_string(&mut contents2).unwrap();

        assert_eq!(
            str::replace(&compiled, "\r", ""),
            str::replace(&contents2, "\r", "")
        )
    }

    #[test]
    fn test000() {
        test("test000")
    }

    #[test]
    fn test001() {
        test("test001")
    }
    #[test]
    fn test002() {
        test("test002")
    }
    #[test]
    fn test003() {
        test("test003")
    }

    #[test]
    fn test004() {
        test("test004")
    }

    #[test]
    fn test005() {
        test("test005")
    }

    #[test]
    fn test006() {
        test("test006")
    }
    #[test]
    fn test007() {
        test("test007")
    }
    #[test]
    fn test008() {
        test("test008")
    }
    #[test]
    fn test009() {
        test("test009")
    }
    #[test]
    fn test010() {
        test("test010")
    }

    #[test]
    fn test011() {
        test("test011")
    }

    #[test]
    fn test012() {
        test("test012")
    }

    #[test]
    fn test013() {
        test("test013")
    }

    #[test]
    fn test014() {
        test("test014")
    }

    #[test]
    fn test015() {
        test("test015")
    }

    #[test]
    fn test016() {
        test("test016")
    }

    #[test]
    fn test017() {
        test("test017")
    }

    #[test]
    fn test018() {
        test("test018")
    }
    #[test]
    fn test019() {
        test("test019")
    }
    #[test]
    fn test020() {
        test("test020")
    }
    #[test]
    fn test021() {
        test("test021")
    }
    #[test]
    fn test022() {
        test("test022")
    }
    #[test]
    fn test023() {
        test("test023")
    }
    #[test]
    fn test024() {
        test("test024")
    }
    #[test]
    fn test025() {
        test("test025")
    }
    #[test]
    fn test026() {
        test("test026")
    }
    #[test]
    fn test027() {
        test("test027")
    }
    #[test]
    fn test028() {
        test("test028")
    }
    #[test]
    fn test029() {
        test("test029")
    }
    #[test]
    fn test030() {
        test("test030")
    }
    #[test]
    fn test031() {
        test("test031")
    }
    #[test]
    fn test032() {
        test("test032")
    }
    #[test]
    fn test033() {
        test("test033")
    }
    #[test]
    fn test034() {
        test("test034")
    }
    #[test]
    fn test035() {
        test("test035")
    }
    #[test]
    fn test036() {
        test("test036")
    }
    #[test]
    fn test037() {
        test("test037")
    }
    #[test]
    fn test038() {
        test("test038")
    }
    #[test]
    fn test039() {
        test("test039")
    }
}

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
        println!("{:?}", lex.clone());
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
