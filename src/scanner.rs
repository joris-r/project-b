
use syntax::Token;

#[test]
fn test_trivial(){
    assert_eq!(scan(" "), vec![Token::Spaces(" ")]);
    assert_eq!(scan(" \n\t"), vec![Token::Spaces(" \n\t")]);
    assert_eq!(scan("//x"), vec![Token::Comment("//x")]);
    assert_eq!(scan("// one"), vec![Token::Comment("// one")]);
    assert_eq!(scan("/* \n*/"), vec![Token::Comment("/* \n*/")]);
    assert_eq!(scan("123"), vec![Token::Integer("123")]);
    assert_eq!(scan("123.456"), vec![Token::Float("123.456")]);
    assert_eq!(scan("foo_bar2"), vec![Token::Identifier("foo_bar2")]);
    assert_eq!(scan("THEN"), vec![Token::Keyword("THEN")]);
    assert_eq!(scan("THENxxx"), vec![Token::Identifier("THENxxx")]);
    assert_eq!(scan("-"), vec![Token::Operator("-")]);
    assert_eq!(scan("-->"), vec![Token::Operator("-->")]);
}

#[test]
fn test_double(){
    assert_eq!(scan("//xy z\n  "), vec![
        Token::Comment("//xy z"),
        Token::Spaces("\n  "),
    ]);
    assert_eq!(scan("\n  //xy z"), vec![
        Token::Spaces("\n  "),
        Token::Comment("//xy z"),
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

struct ScannerState<'a> {
    i : usize,
    j : usize,
    token : Token<'a>,
    source : &'a str,
    
    size_left : usize, // in bytes
    size_right : usize, // in bytes
}

pub fn scan(source : &str) -> Vec<Token> {
    let mut state = ScannerState{
        i:0,
        j:0,
        token : Token::Error,
        source : source,
        
        size_left : 0,
        size_right : 0,
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
        state.size_left = state.size_right;
        state.token = Token::Error;
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
            let content = &self.source[self.size_left..self.size_right];
            self.token = Token::Spaces(content);
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
            let content = &self.source[self.size_left..self.size_right];
            self.token = Token::Comment(content);
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
            let content = &self.source[self.size_left..self.size_right];
            self.token = Token::Comment(content)
        }
    }
    
    fn scan_number(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let mut float = false;
        loop {
            match self.source.char_indices().nth(x) {
                Some((i,'0' ... '9')) => {
                    x += 1;
                    new_right = i + '0'.len_utf8();
                },
                Some((i,'.')) => {
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
            if float {
                self.token = Token::Float(content)
            } else {
                self.token = Token::Integer(content)
            }
        }
    }

    fn scan_identifier(&mut self){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let mut iter = self.source.char_indices().skip(self.i);
        'outer: loop {
            match iter.next() {
                Some((i,c)) if c.is_alphabetic() => {
                    println!("({},{})",i,c);
                    x += 1;
                    new_right = i + c.len_utf8();
                    'inner: loop {
                        match iter.next() {
                            Some((i,c)) if c.is_alphabetic() ||
                                       c.is_numeric() ||
                                       c == '_' => {
                                println!("({},{})",i,c);
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
            self.token = Token::Identifier(content)
        }
    }
    
    fn scan_keyword(&mut self, keyword : &str){
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
            let content = &self.source[self.size_left..self.size_right];
            self.token = Token::Keyword(content)
        }
    }
    
    // TODO factorize with scan_keyword
    fn scan_operator(&mut self, operator : &str){
        let mut x = self.i;
        let mut new_right = self.size_left;
        let iter = self.source.char_indices().skip(self.i);
        let ik = operator.chars();
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
            let content = &self.source[self.size_left..self.size_right];
            self.token = Token::Operator(content)
        }
    }
    
}



// TODO scan_operator


