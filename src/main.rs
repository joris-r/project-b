extern crate unicode_normalization;

use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod syntax;
mod scanner;

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
        .collect::<Vec<&syntax::Token>>();

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


fn main() {
    print_tokens("examples/gfarr.mch");
    print_tokens("examples/gfarr_i.imp");
}

