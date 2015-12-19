
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token {
    Error,
    Spaces(usize, usize),
    Comment(usize, usize),
}

#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![Token::Spaces(0,1)]);
    assert_eq!(scan(" \n\t"), vec![Token::Spaces(0,3)]);
    // assert_eq!(scan("//x"), vec![Token::Comment(0,3)]);
    // assert_eq!(scan("// one"), vec![Token::Comment(0,6)]);
}

struct ScannerState {
    i : usize,
    j : usize,
    token : Token,
}

fn scan(source : & str) -> Vec<Token> {
    let mut state = ScannerState{ i:0, j:0, token : Token::Error };
    scan_spaces(source, &mut state);
    // scan_comment(source, &mut state);
    vec![state.token]
}

fn scan_spaces(source : &str, state : &mut ScannerState){
    let mut l = state.i;
    loop {
        match source.chars().nth(l) {
            Some(' ') | Some('\t') | Some('\n') => l += 1,
            _ => break,
        }
    }
    if state.j < l {
        state.j = l;
        state.token = Token::Spaces(state.i, l)
    }
}

// fn scan_comment(source : &str, state : &mut ScannerState){
    // let mut l = state.i;

// }

fn main() {
    println!("{:?}", scan(" \n\t"));
}





























