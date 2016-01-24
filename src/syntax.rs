
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Error,
    Spaces(&'a str),
    Comment(&'a str),
    Integer(&'a str),
    Float(&'a str),
    Identifier(&'a str),
    Keyword(&'a str),
    Operator(&'a str),
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
