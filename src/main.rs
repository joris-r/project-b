extern crate unicode_normalization;

use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod syntax;
mod scanner;
mod genlex;
mod parser_lalrpop;




fn main() {
    print_tokens("examples/gfarr.mch");
    print_tokens("examples/gfarr_i.imp");
    
    println!("\n\n-----------------------------------------------");
    let src = "IMPLEMENTATION imp VARIABLES var1,v2, toto END";
    let res = parser_lalrpop::parse_Component(src);
    println!("{:?}",res);
    
}

fn print_tokens(path: &str) {
    let path = Path::new(path);
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    
    let tokens = scanner::scan(&source);
    let tokens = tokens
        .iter()
        .filter(|t| match **t {
            syntax::Token::Spaces(_) => false,
            _ => true, })
        .collect::<Vec<&syntax::Token<&str>>>();

    let sep = std::iter::repeat("=")
        .take(80)
        .collect::<String>();
    println!("{}", sep);
    println!("{}:", path.display());
    println!("{}", sep);
    println!("{:?}", tokens);
    println!("{}", sep);

    for t in tokens {
        println!("{:?}", t);
    }
}


/* TODO does not compile
fn test() {
    let mut i = 0;
    for expected in genlex::new().take(10_000) {
        i += 1;
        let src = syntax::tokens_to_source(&expected);
        print!("testing source {}: {:?}",i,src);
        let result = scanner::scan(&src);
        if result == expected {
            println!(" ok");
        } else {
            println!(" error");
            println!("expected : {:?}", expected);
            println!("result : {:?}", result);
            break;
        }
    }
}
*/


