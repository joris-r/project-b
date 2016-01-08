
mod genlex;
mod syntax;
mod scanner;

use syntax::Token;

fn main() {
    
    let mut i = 0;
    for (src,expected) in genlex::new().take(121) {
        i += 1;
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

