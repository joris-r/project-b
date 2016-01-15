
mod genlex;
mod syntax;
mod scanner;

fn main() {
    test();
}

fn test() {
    let mut i = 0;
    for (src,expected) in genlex::new().take(10_000) {
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