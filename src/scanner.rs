
use syntax::Token;

#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![Token::Spaces(0,1)]);
    assert_eq!(scan(" \n\t"), vec![Token::Spaces(0,3)]);
    assert_eq!(scan("//x"), vec![Token::Comment(0,3)]);
    assert_eq!(scan("// one"), vec![Token::Comment(0,6)]);
}

#[test]
fn test_double(){
    assert_eq!(scan("//xy z\n  "), vec![
        Token::Comment(0,6),
        Token::Spaces(6,9),
    ]);
    assert_eq!(scan("\n  //xy z"), vec![
        Token::Spaces(0,3),
        Token::Comment(3,9),
    ]);
}

struct ScannerState {
    i : usize,
    j : usize,
    token : Token,
}

pub fn scan(source : & str) -> Vec<Token> {
    let mut state = ScannerState{ i:0, j:0, token : Token::Error };
    let mut res = vec![];
    loop {
        scan_spaces(source, &mut state);
        scan_comment_monoline(source, &mut state);
        if state.token == Token::Error {
            return res
        }
        res.push(state.token);
        state.i = state.j;
        state.token = Token::Error;
    }
}

fn scan_spaces(source : &str, state : &mut ScannerState){
    let mut x = state.i;
    loop {
        match source.chars().nth(x) {
            Some(' ') | Some('\t') | Some('\n') => x += 1,
            _ => break,
        }
    }
    if state.j < x {
        state.j = x;
        state.token = Token::Spaces(state.i, x)
    }
}

fn scan_comment_monoline(source : &str, state : &mut ScannerState){
    let mut x = state.i;
    if source.chars().nth(x) == Some('/') &&
       source.chars().nth(x+1) == Some('/') {
        x += 2;
        loop {
            match source.chars().nth(x) {
                None => break,
                Some('\n') => break,
                Some(_) => x += 1,
            }
        }
    }
    if state.j < x {
        state.j = x;
        state.token = Token::Comment(state.i, x)
    }
}
























