
use syntax::Token;

#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![Token::Spaces(0,1)]);
    assert_eq!(scan(" \n\t"), vec![Token::Spaces(0,3)]);
    assert_eq!(scan("//x"), vec![Token::Comment(0,3)]);
    assert_eq!(scan("// one"), vec![Token::Comment(0,6)]);
    assert_eq!(scan("/* \n*/"), vec![Token::Comment(0,6)]);
    assert_eq!(scan("123"), vec![Token::Integer(0,3)]);
    assert_eq!(scan("123.456"), vec![Token::Float(0,7)]);
    assert_eq!(scan("foo_bar2"), vec![Token::Identifier(0,8)]);
    assert_eq!(scan("THEN"), vec![Token::Keyword(0,4)]);
    assert_eq!(scan("THENxxx"), vec![Token::Identifier(0,7)]);
    assert_eq!(scan("-"), vec![Token::Operator(0,1)]);
    assert_eq!(scan("-->"), vec![Token::Operator(0,3)]);
}

#[test]
fn test_unicode_1(){
    // one char and one grapheme
    
    let mut reconstruct = "".to_owned();
    reconstruct.push('é'); // one 
    assert_eq!("é", reconstruct);
    
    assert_eq!(scan("é"), vec![Token::Identifier(0,1)]);
}

#[test]
fn test_unicode_2(){
    // two chars but one graphemes
    
    let mut reconstruct = "".to_owned();
    reconstruct.push('e');
    reconstruct.push('́');
    assert_eq!("é", reconstruct);
    
    // the result should be 2 chars long
    assert_eq!(scan("é"), vec![Token::Identifier(0,2)]);
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
        state.scan_number();
        for keyword in ::syntax::KEYWORDS.iter() {
            state.scan_keyword(keyword);
        }
        state.scan_identifier();
        for operator in ::syntax::OPERATORS.iter() {
            state.scan_operator(operator);
        }
        
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
    
    fn scan_number(&mut self){
        let mut x = self.i;
        let mut float = false;
        loop {
            match self.source.chars().nth(x) {
                Some('0' ... '9') => x += 1,
                Some('.') => {
                    float = true;
                    x += 1;
                }
                Some(_) => break,
                _ => break,
            }
        }
        if self.j < x {
            self.j = x;
            if float {
                self.token = Token::Float(self.i, x)
            } else {
                self.token = Token::Integer(self.i, x)
            }
        }
    }

    fn scan_identifier(&mut self){
        let mut x = self.i;
        let mut iter = self.source.chars().skip(self.i);
        'outer: loop {
            match iter.next() {
                Some(c) if c.is_alphabetic() => {
                    x += 1;
                    'inner: loop {
                        match iter.next() {
                            Some(c) if c.is_alphabetic() ||
                                       c.is_numeric() ||
                                       c == '_' => {
                                x += 1;
                                continue 'inner;
                            },
                            _ => {
                                break 'outer;
                            },
                        }
                    }
                },
                _ => {
                    break 'outer
                },
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Identifier(self.i, x)
        }
    }
    
    fn scan_keyword(&mut self, keyword : &str){
        let mut x = self.i;
        let iter = self.source.chars().skip(self.i);
        let ik = keyword.chars();
        for (a,b) in iter.zip(ik) {
            if a == b {
                x += 1;
            } else {
                break;
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Keyword(self.i, x)
        }
    }
    
    // TODO factorize with scan_keyword
    fn scan_operator(&mut self, operator : &str){
        let mut x = self.i;
        let iter = self.source.chars().skip(self.i);
        let ik = operator.chars();
        for (a,b) in iter.zip(ik) {
            if a == b {
                x += 1;
            } else {
                break;
            }
        }
        if self.j < x {
            self.j = x;
            self.token = Token::Operator(self.i, x)
        }
    }
    
}



// TODO scan_operator


