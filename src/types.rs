#[derive(Debug)]
pub enum Token {
    Newline(String),
    Number(String),
    // Operators
    Plus(String),
    Minus(String),
    Asterisk(String),
    EQ(String),
    InvalidToken(String),
}