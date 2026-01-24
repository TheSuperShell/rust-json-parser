use std::fmt;

use crate::tokens::Token;
#[derive(Debug, Clone)]
pub enum Error {
    LexerError { msg: String, position: usize },
    ParserError { msg: String, token: Token },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexerError { msg, position } => {
                write!(f, "Lexer Error at position {}: {}", position, msg)
            }
            Error::ParserError { msg, token } => {
                write!(f, "Parser Error at token {:?}: {}", token, msg)
            }
        }
    }
}
