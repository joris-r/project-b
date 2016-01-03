
mod genlex;
mod syntax;
mod scanner;

use syntax::Token;

fn main() {
    let mut size = 0;
    let mut list = vec!["".to_string()];
    loop {
        match give_one(&mut size, &mut list) {
            Some(s) => println!("{:?}",s),
            None => break
        }
    }
}

fn give_one(size : &mut usize, list : &mut Vec<String>) -> Option<String> {
    loop {
        match list.pop() {
            None => {
                *size += 1;
                list.push("".to_string());
            },
            Some(s) => {
                if s.len() == *size {
                    return Some(s);
                } else if s.len() > *size {
                    continue;
                } else {
                    list.push(s.clone() + "a");
                    list.push(s.clone() + "bbb");
                    continue;
                }
            }
        }
    }
}

fn enum_rec(){
    for n in 0..7 {
        enum_rec_f(n, "".to_string());
    }
}

fn enum_rec_f(n : usize, s : String){
    if n == s.len() {
        println!("{}",s);
        return;
    }
    if n < s.len() {
        return;
    }
    enum_rec_f(n, s.clone() + "a");
    enum_rec_f(n, s.clone() + "bb");
}

fn test() {
    
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

























