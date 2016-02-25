
use syntax::{Token};


//
//         Scanner Properties
//
// Tokens are a partition of the source
// tokens are not empty
// No Consequent Spaces
// Spaces contains only white spaces
// Comments either:
//   - start with "//" contains no '\n' and next token start with '\n';
//   - or start with "/*" and ends with "*/", contains no inner "*/"
//     (accept multi-comments inside multi-comment?)
//


#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![]);
    assert_eq!(scan(" \n\t"), vec![]);
    assert_eq!(scan("//x"), vec![]);
    assert_eq!(scan("// one"), vec![]);
    assert_eq!(scan("/* \n*/"), vec![]);
    assert_eq!(scan("123"), vec![Token::Integer("123")]);
    assert_eq!(scan("123."), vec![Token::Float("123.")]);
    assert_eq!(scan("123.456"), vec![Token::Float("123.456")]);
    assert_eq!(scan("foo_bar2"), vec![Token::Identifier("foo_bar2")]);
    assert_eq!(scan("THEN"), vec![Token::KwTHEN]);
    assert_eq!(scan("THENxxx"), vec![Token::Identifier("THENxxx")]);
    assert_eq!(scan("-"), vec![Token::OpMinus]);
    assert_eq!(scan("-->"), vec![Token::OpTotalfun]);
    assert_eq!(scan("◦"), vec![Token::OpBullet]);
    assert_eq!(scan("×"), vec![Token::OpCross]);
    assert_eq!(scan("·"), vec![Token::OpMdot]);
}

#[test]
fn test_double(){
    assert_eq!(scan("//xy z\n  "), vec![
    ]);
    assert_eq!(scan("\n  //xy z"), vec![
    ]);
}

#[test]
fn test_unicode_1(){
    // one char and one grapheme
    
    let mut reconstruct = "".to_owned();
    reconstruct.push('é'); // one 
    assert_eq!("é", reconstruct);
    
    assert_eq!(scan("é"), vec![Token::Identifier("é")]);
}

#[test]
fn test_unicode_2(){
    // two chars but one graphemes
    
    let mut reconstruct = "".to_owned();
    reconstruct.push('e');
    reconstruct.push('́');
    assert_eq!("é", reconstruct);
    
    // the result should be 2 chars long
    // (if we consider composite char as legal string for identifier)
    //assert_eq!(scan("é"), vec![Token::Identifier("é")]);
    // TODO doesn't work for the moment
}

#[test]
fn test_unicode_normalisation(){
    use unicode_normalization::UnicodeNormalization;

    let mut composite = "".to_owned();
    composite.push('e');
    composite.push('́');
    
    assert!("é" != composite); // one grapheme
    assert!("é" == composite); // two graphemes
    
    let normalized : String = composite.nfc().collect();
    
    assert!("é" == normalized); // one grapheme
    assert!("é" != normalized); // two graphemes
    
}

#[test]
fn test_unicode_alphab(){
    // composable char are not alphabetic ...
    assert!( ! '́'.is_alphabetic())
}

#[test]
fn test_composed(){
    assert_eq!(scan("1..2"), vec![
        Token::Integer("1"),
        Token::OpInter,
        Token::Integer("2")]);
}

struct ScannerState <'a> {
    i : usize,
    j : usize,
    token : Option<Token<'a>>,
    source : &'a str,
    
    size_left : usize, // in bytes
    size_right : usize, // in bytes
    
    stop : bool,
}

pub fn scan <'a>(source : &'a str) -> Vec<Token> {
    let mut state = ScannerState{
        i:0,
        j:0,
        token : None,
        source : source,
        
        size_left : 0,
        size_right : 0,
        
        stop : true,
    };
    let mut res = vec![];
    loop {
    
        state.scan_spaces();
        state.scan_comment_monoline();
        state.scan_comment_multiline();
        state.scan_number();
        
        for &(string,tok) in &::syntax::KEYWORDS {
            state.scan_string(string,tok);
        }
        
        state.scan_identifier();
        
        for &(string,tok) in &::syntax::OPERATORS {
            state.scan_string(string,tok);
        }
        
        if state.stop {
            return res
        }
        match state.token {
            Some(tok) => {res.push(tok);},
            None => {},
        }
        state.i = state.j;
        state.size_left = state.size_right;
        state.token = None;
        state.stop = true;
    }
}

impl<'a> ScannerState<'a> {

    fn scan_spaces(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        loop {
            match self.source.char_indices().nth(x) {
                Some((i,' ')) | Some((i,'\t')) | Some((i,'\n')) => {
                    x += 1;
                    new_right = i + ' '.len_utf8();
                },
                _ => break,
            }
        }
        if self.j < x {
            self.j = x;
            self.size_right = new_right;
            self.token = None;
            self.stop = false;
        }
    }

    fn scan_comment_monoline(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        if self.source.chars().nth(x) == Some('/') &&
           self.source.chars().nth(x+1) == Some('/') {
            x += 2;
            new_right +=  '/'.len_utf8()*2;
            loop {
                match self.source.char_indices().nth(x) {
                    None => break,
                    Some((_,'\n')) => break,
                    Some((i,c)) => {
                        x += 1;
                        new_right = i + c.len_utf8();
                    },
                }
            }
        }
        if self.j < x {
            self.j = x;
            self.size_right = new_right;
            self.token = None;
            self.stop = false;
        }
    }
    
    fn scan_comment_multiline(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let mut iter = self.source.char_indices();
        if iter.nth(x) == Some((new_right,'/')) &&
           iter.next() == Some((new_right+'/'.len_utf8(),'*')) {
            x += 2;
            new_right +=  '/'.len_utf8()  + '*'.len_utf8();
            'outer: loop {
                match iter.next() {
                    Some((i,'*')) => {
                        x += 1;
                        new_right = i + '*'.len_utf8();
                        'inner: loop {
                            match iter.next() {
                                Some((i,'/')) => {
                                    x += 1;
                                    new_right = i + '/'.len_utf8();
                                    break 'outer;
                                },
                                Some((i,'*')) => {
                                    x += 1;
                                    new_right = i + '*'.len_utf8();
                                    continue 'inner;
                                },
                                Some((i,c)) => {
                                    x += 1;
                                    new_right = i + c.len_utf8();
                                    continue 'outer;
                                },
                                None => {
                                    break 'outer;
                                },
                            }
                        }
                    },
                    Some((i,c)) => {
                        x += 1;
                        new_right = i + c.len_utf8();
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
            self.size_right = new_right;
            self.token = None;
            self.stop = false;
        }
    }
    
    fn scan_number(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let mut float = false;
        let mut iter = self.source.char_indices().skip(x);
        loop {
            match iter.next() {
                Some((i,'0' ... '9')) => {
                    x += 1;
                    new_right = i + '0'.len_utf8();
                },
                Some((i,'.')) => {
                    let mut iter_tmp = iter.clone();
                    // Looking for a '..' operator which can confuse with a float like "1."
                    match iter_tmp.next() {
                        Some((_, '.')) => break,
                        _ => {}
                    }
                    if float == true { break; }
                    float = true;
                    x += 1;
                    new_right = i + '.'.len_utf8();
                }
                _ => break,
            }
        }
        if self.j < x {
            self.j = x;
            self.size_right = new_right;
            let content = &self.source[self.size_left..self.size_right];
            self.token = if float {
                Some(Token::Float(content))
            } else {
                Some(Token::Integer(content))
            };
            self.stop = false;
        }
    }

    fn scan_identifier(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let mut iter = self.source.char_indices().skip(self.i);
        'outer: loop {
            match iter.next() {
                Some((i,c)) if c.is_alphabetic() => {
                    x += 1;
                    new_right = i + c.len_utf8();
                    'inner: loop {
                        match iter.next() {
                            Some((i,c)) if c.is_alphabetic() ||
                                       c.is_numeric() ||
                                       c == '_' => {
                                x += 1;
                                new_right = i + c.len_utf8();
                                continue 'inner;
                            },
                            _ => break 'outer,
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
            self.size_right = new_right;
            let content = &self.source[self.size_left..self.size_right];
            self.token = Some(Token::Identifier(content));
            self.stop = false;
        }
    }
    
    fn scan_string(&mut self, keyword : &str, tok : Token<'a>){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let iter = self.source.char_indices().skip(self.i);
        let ik = keyword.chars();
        for ((i,a),b) in iter.zip(ik) {
            if a == b {
                x += 1;
                new_right = i + a.len_utf8();
            } else {
                break;
            }
        }
        if self.j < x {
            self.j = x;
            self.size_right = new_right;
            self.token = Some(tok);
            self.stop = false;
        }
    }
    
}
