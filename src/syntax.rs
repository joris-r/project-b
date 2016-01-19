
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Spaces(usize, usize),
    Comment(usize, usize),
    Integer(usize, usize),
    Float(usize, usize),
    Identifier(usize, usize),
    Keyword(usize, usize),
    //TODO: use slice instead of a index pair
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
