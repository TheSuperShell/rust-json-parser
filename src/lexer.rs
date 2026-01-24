use crate::tokens::Token;

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

    pub fn next_token(&mut self) -> Token {
        self.trim_whitespace();
        let char = self.advance();
        match char {
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('[') => Token::LeftParen,
            Some(']') => Token::RightParen,
            Some(',') => Token::Comma,
            Some(':') => Token::Collon,
            Some('"') => self.string(),
            Some(ch) if ch.is_digit(10) => {
                self.posistion -= 1; // step back to include the digit
                self.number()
            }
            Some(ch) if ch == '-' && self.peek().map_or(false, |c| c.is_digit(10)) => {
                self.posistion -= 1; // step back to include the '-'
                self.number()
            }
            Some(ch) if ch.is_alphabetic() => {
                self.posistion -= 1; // step back to include the character
                self.id()
            }
            None => Token::EOF,
            _ => panic!("Unexpected character: {:?}", char),
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

    fn id(&mut self) -> Token {
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
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "null" => Token::Null,
            _ => panic!("Unknown identifier: {}", ident),
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
