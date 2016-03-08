extern crate unicode_normalization;

mod syntax;
mod scanner;
mod parser_lalrpop;
mod mbast;



fn main() {
    let src = "IMPLEMENTATION m0_i REFINES m0 CONSTRAINTS 2+2=4 END";
    let tok = scanner::scan(src);
    let res = parser_lalrpop::parse_Component(src, tok);
    println!("{:?}", res);
}

