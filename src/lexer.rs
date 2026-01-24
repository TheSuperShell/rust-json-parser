use crate::{error::Error, tokens::Token};

#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub posistion: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            posistion: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.trim_whitespace();
        let char = self.advance();
        match char {
            Some('{') => Ok(Token::LeftBrace),
            Some('}') => Ok(Token::RightBrace),
            Some('[') => Ok(Token::LeftParen),
            Some(']') => Ok(Token::RightParen),
            Some(',') => Ok(Token::Comma),
            Some(':') => Ok(Token::Collon),
            Some('"') => Ok(self.string()),
            Some(ch) if ch.is_digit(10) => {
                self.posistion -= 1; // step back to include the digit
                Ok(self.number())
            }
            Some(ch) if ch == '-' && self.peek().map_or(false, |c| c.is_digit(10)) => {
                self.posistion -= 1; // step back to include the '-'
                Ok(self.number())
            }
            Some(ch) if ch.is_alphabetic() => {
                self.posistion -= 1; // step back to include the character
                self.id()
            }
            None => Ok(Token::EOF),
            _ => Err(Error::LexerError {
                msg: format!("Unexpected character {:?}", char),
                position: self.posistion,
            }),
        }
    }

    fn trim_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn id(&mut self) -> Result<Token, Error> {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Err(Error::LexerError {
                msg: format!("Unknown identifier: {}", ident),
                position: self.posistion,
            }),
        }
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let number: f64 = num_str.parse().unwrap();
        Token::Number(number)
    }

    fn string(&mut self) -> Token {
        let mut result = String::new();
        let mut char = self.advance();
        while let Some(ch) = char {
            if ch == '"' {
                break;
            } else {
                result.push(ch);
                char = self.advance();
            }
        }
        Token::String(result)
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.posistion)
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.posistion += 1;
        }
        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokens() {
        let input = r#"{
            "name": "Alice",
            "age": 25,
            "is_student": true,
            "courses": ["Math", "Science"],
            "address": null
        }"#;

        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = vec![
            Token::LeftBrace,
            Token::String("name".to_string()),
            Token::Collon,
            Token::String("Alice".to_string()),
            Token::Comma,
            Token::String("age".to_string()),
            Token::Collon,
            Token::Number(25.0),
            Token::Comma,
            Token::String("is_student".to_string()),
            Token::Collon,
            Token::Boolean(true),
            Token::Comma,
            Token::String("courses".to_string()),
            Token::Collon,
            Token::LeftParen,
            Token::String("Math".to_string()),
            Token::Comma,
            Token::String("Science".to_string()),
            Token::RightParen,
            Token::Comma,
            Token::String("address".to_string()),
            Token::Collon,
            Token::Null,
            Token::RightBrace,
            Token::EOF,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_lexer_unexpected_character() {
        let input = r#"{"key": @value}"#;
        let mut lexer = Lexer::new(input.to_string());

        // Consume tokens until the unexpected character
        lexer.next_token().unwrap(); // {
        lexer.next_token().unwrap(); // "key"
        lexer.next_token().unwrap(); // :

        let result = lexer.next_token();
        assert!(result.is_err());
        if let Err(Error::LexerError { msg, position }) = result {
            assert_eq!(msg, "Unexpected character Some('@')");
            assert_eq!(position, 9); // position of '@'
        }
    }
}
