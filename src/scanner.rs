
use syntax::Token;

#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![Token::Spaces(0,1)]);
    assert_eq!(scan(" \n\t"), vec![Token::Spaces(0,3)]);
    assert_eq!(scan("//x"), vec![Token::Comment(0,3)]);
    assert_eq!(scan("// one"), vec![Token::Comment(0,6)]);
    assert_eq!(scan("/* \n*/"), vec![Token::Comment(0,6)]);
    assert_eq!(scan("123"), vec![Token::Integer(0,3)]);
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

struct ScannerState<'a> {
    i : usize,
    j : usize,
    token : Token,
    source : &'a str,
}

pub fn scan(source : &str) -> Vec<Token> {
    let mut state = ScannerState{
        i:0,
        j:0,
        token : Token::Error,
        source : source,
    };
    let mut res = vec![];
    loop {
    
        state.scan_spaces();
        state.scan_comment_monoline();
        state.scan_comment_multiline();
        state.scan_integer();
        
        if state.token == Token::Error {
            return res
        }
        res.push(state.token);
        state.i = state.j;
        state.token = Token::Error;
    }
}

impl<'a> ScannerState<'a> {

    fn scan_spaces(&mut self){
        let mut x = self.i;
        loop {
            match self.source.chars().nth(x) {
                Some(' ') | Some('\t') | Some('\n') => x += 1,
                _ => break,
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Spaces(self.i, x)
        }
    }

    fn scan_comment_monoline(&mut self){
        let mut x = self.i;
        if self.source.chars().nth(x) == Some('/') &&
           self.source.chars().nth(x+1) == Some('/') {
            x += 2;
            loop {
                match self.source.chars().nth(x) {
                    None => break,
                    Some('\n') => break,
                    Some(_) => x += 1,
                }
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Comment(self.i, x)
        }
    }
    
    fn scan_comment_multiline(&mut self){
        let mut x = self.i;
        let mut iter = self.source.chars();
        if iter.nth(x) == Some('/') &&
           iter.next() == Some('*') {
            x += 2;
            'outer: loop {
                match iter.next() {
                    Some('*') => {
                        x += 1;
                        'inner: loop {
                            match iter.next() {
                                Some('/') => {
                                    x += 1;
                                    break 'outer;
                                },
                                Some('*') => {
                                    x += 1;
                                    continue 'inner;
                                },
                                Some(_) => {
                                    x += 1;
                                    continue 'outer;
                                },
                                None => {
                                    break 'outer;
                                },
                            }
                        }
                    },
                    Some(_) => {
                        x += 1;
                        continue 'outer;
                    },
                    None => {
                        break 'outer;
                    },
                }
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Comment(self.i, x)
        }
    }
    
    fn scan_integer(&mut self){
        let mut x = self.i;
        loop {
            match self.source.chars().nth(x) {
                Some('0' ... '9') => x += 1,
                Some(_) => break,
                _ => break,
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Integer(self.i, x)
        }
    }

}


// TODO scan_number
// TODO scan_identifier
// TODO scan_keyword
// TODO scan_operator


