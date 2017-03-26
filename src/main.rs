
mod parser_lalrpop;
mod mbast;



fn main() {
    let src = "IMPLEMENTATION m0_i REFINES m0 IMPORTS toto(2+2)
    INITIALISATION BEGIN xx := 10 END END";
    let res = parser_lalrpop::parse_Component(src);
    println!("{:?}", res);
}

