
extern crate unicode_normalization;

mod syntax;
mod scanner;
mod genlex;

fn main() {
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


