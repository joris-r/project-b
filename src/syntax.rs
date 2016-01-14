
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Spaces(usize, usize),
    Comment(usize, usize),
    Integer(usize, usize),
    Float(usize, usize),
    //TODO: use slice instead of a index pair
}

/*

From the example:

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


"(", ")", "{", "}", "[", "]", ",", ":", "&", "..", "-->", ":=", "<--",
"=", ";", "\/", "/\", "%", "|", "-", "+", "*", "/", "<|",
*/
