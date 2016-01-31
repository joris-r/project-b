
use syntax::TokensOwn;

pub struct LexGen {
    list : Vec<TokensOwn>,
    size : usize,
}

pub fn new() -> LexGen {
    LexGen{
        list : vec![vec![]],
        size : 0,
    }
}

impl Iterator for LexGen {
    type Item = TokensOwn;
    
    fn next(&mut self) -> Option<TokensOwn> {
        Some(vec![])
    }
}

