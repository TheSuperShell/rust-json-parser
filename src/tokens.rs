#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    RightBrace,
    LeftBrace,
    RightParen,
    LeftParen,
    Comma,
    Collon,
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Eof,
}
