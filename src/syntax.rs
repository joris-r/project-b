
#[derive(Debug, PartialEq, Eq)]
pub enum Token<T> where T:PartialEq+ Eq{
    Error,
    Spaces(T),
    Comment(T),
    Integer(T),
    Float(T),
    Identifier(T),
    Keyword(T),
    Operator(T),
}

pub type TokensOwn = Vec<Token<String>>;

pub type TokensRef<'a> = Vec<Token<&'a str>>;



pub fn tokens_to_source( toks : &TokensOwn ) -> String {
    "".to_owned() //TODO implement me
}

/* TODO does not compile
pub fn tokensOwn_to_tokensRef( toks : &TokensOwn ) -> &TokensRef {
    let mut res = vec![];
    for tok in &toks {
        match tok {
        Error @t => res.push(t),
        _ =>  (),
        }
    }
    res
}
*/

pub const KEYWORDS : [&'static str; 16] = [
    "MACHINE",
    "IMPLEMENTATION",
    "CONSTRAINTS",
    "CONCRETE_VARIABLES",
    "INVARIANT",
    "INITIALISATION",
    "OPERATIONS",
    "END",
    "PRE",
    "THEN",
    "VAR",
    "IN",
    "WHILE",
    "DO",
    "INVARIANT",
    "VARIANT",
];

pub const OPERATORS : [&'static str; 26] = [
    "(", ")", "{", "}", "[", "]", ",", ":", "&", "..", "-->", ":=", "<--",
    "=", ";", "◦", "×", r"\/", r"/\", "%", "|", "-", "+", "·", "/", "<|",
];
