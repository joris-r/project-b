
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Error,
    Spaces(usize, usize, &'a str),
    Comment(usize, usize, &'a str),
    Integer(usize, usize, &'a str),
    Float(usize, usize, &'a str),
    Identifier(usize, usize, &'a str),
    Keyword(usize, usize, &'a str),
    Operator(usize, usize, &'a str),
}

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

pub const OPERATORS : [&'static str; 24] = [
    "(", ")", "{", "}", "[", "]", ",", ":", "&", "..", "-->", ":=", "<--",
    "=", ";", r"\/", r"/\", "%", "|", "-", "+", "*", "/", "<|",
];
