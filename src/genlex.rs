
use syntax::Token;

const SPACES_CYCLE : [char; 3] = ['\n', '\t', ' '];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenToken {
    Spaces(String),
}

pub struct LexGen {
    list : Vec<Vec<GenToken>>,
    size : usize,
}

pub fn new() -> LexGen {
    LexGen{
        list : vec![vec![]],
        size : 0,
    }
}

fn length_list(src : &Vec<GenToken>) -> usize{
    src.iter().fold(0, |sum, ref val| {sum + length_token(val)})
}

fn length_token(token : &GenToken) -> usize{
    match token {
        &GenToken::Spaces(ref s) => s.len()
    }
}

impl LexGen {

    fn enumerate(&mut self) -> Vec<GenToken> {
        loop {
            match self.list.pop() {
                None => {
                    self.size += 1;
                    self.list.push(vec![]);
                },
                Some(elem) => {
                    if length_list(&elem) == self.size {
                        return elem;
                    } else if length_list(&elem) > self.size {
                        continue;
                    } else {
                        for c in SPACES_CYCLE.iter() {
                            let mut new = elem.clone();
                            new.push(GenToken::Spaces(c.to_string()));
                            self.list.push(new);
                        }
                        continue;
                    }
                }
            }
        }
    }
    
}
    
fn generate(res : &Vec<GenToken>) -> (String, Vec<Token>) {

    let mut res_str = String::new();
    let mut res_tok = vec![];
    let mut before = 0;
    
    for t in res.clone() {
        match t {
            GenToken::Spaces(s) => {
                res_str.push_str(&s);
                match res_tok.last() {
                    Some(& Token::Spaces(a,b)) =>  {
                        res_tok.pop();
                        res_tok.push(Token::Spaces(a,b+s.len()));
                        // TODO possible to modify in place with a mutable pattern ?
                    }
                    _ => {
                        res_tok.push( Token::Spaces(before, res_str.len()) );
                    }
                }
                before = res_str.len();
            }
        }
    }
    
    (res_str, res_tok)
}

impl Iterator for LexGen {
    type Item = (String,Vec<Token>);
    
    fn next(&mut self) -> Option<(String,Vec<Token>)> {
        let res = self.enumerate();
        let res = Some(generate(&res));
        res
    }
}

#[test]
// Really a bad way to test, maybe I can just check is some
// outputs are present
fn test_gen() {
    let mut gen = new();
    assert_eq!(gen.next(), Some((
        "".to_string(), vec![
        ])));
    assert_eq!(gen.next(), Some((
        " ".to_string(), vec![
            Token::Spaces(0,1),
        ])));
    assert_eq!(gen.next(), Some((
        "\t".to_string(), vec![
            Token::Spaces(0,1),
        ])));
    assert_eq!(gen.next(), Some((
        "\n".to_string(), vec![
            Token::Spaces(0,1),
        ])));
        
    assert_eq!(gen.next(), Some((
        "  ".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        " \t".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        " \n".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        "\t ".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        "\t\t".to_string(), vec![
            Token::Spaces(0,2),
        ])));
}

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




