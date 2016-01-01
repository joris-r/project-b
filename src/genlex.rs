
use syntax::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenToken {
    Spaces(char),
}

pub struct LexGen {
    next : Vec<GenToken>,
}

const SPACES_CYCLE : [char; 3] = [' ', '\t', '\n'];

impl LexGen {

    fn increment(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.next.len() {
                self.next.push(GenToken::Spaces(SPACES_CYCLE[0]));
                break;
            } else {
                let x = match self.next[i] {
                    GenToken::Spaces(c) =>
                        SPACES_CYCLE.iter().position(|x| *x==c).unwrap()
                    };
                if x + 1 < SPACES_CYCLE.len() {
                    self.next[i] = GenToken::Spaces(SPACES_CYCLE[x+1]);
                    break;
                } else {
                    self.next[i] = GenToken::Spaces(SPACES_CYCLE[0]);
                    i += 1;
                }
            }
        }
    }
    
    fn generate(&self) -> (String, Vec<Token>) {

        let mut res_str = String::new();
        let mut res_tok = vec![];
        let mut before = 0;
        
        for t in self.next.clone() {
            match t {
                GenToken::Spaces(s) => {
                    res_str.push(s);
                    match res_tok.last() {
                        Some(& Token::Spaces(a,b)) =>  {
                            res_tok.pop();
                            res_tok.push(Token::Spaces(a,b+1));
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
    
}

pub fn new() -> LexGen {
    LexGen{
        next : vec![],
    }
}

impl Iterator for LexGen {
    type Item = (String,Vec<Token>);
    
    fn next(&mut self) -> Option<(String,Vec<Token>)> {
        let res = Some(self.generate());
        self.increment();
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
        "\t ".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        "\n ".to_string(), vec![
            Token::Spaces(0,2),
        ])));
    assert_eq!(gen.next(), Some((
        " \t".to_string(), vec![
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




