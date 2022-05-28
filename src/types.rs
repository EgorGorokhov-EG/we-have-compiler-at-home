#[derive(Debug)]
pub enum Token {
    Newline(String),
    Number(String),
    Var(String),
    // Keywords
    Let(String),
    // Operators
    Plus(String),
    Minus(String),
    Asterisk(String),
    EQ(String),
    InvalidToken(String),
}