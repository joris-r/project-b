
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Spaces(usize, usize),
    Comment(usize, usize),
}
