use std::collections::HashMap;

use crate::{error::Error, lexer::Lexer, tokens::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    Sting(String),
    Array(Box<Vec<Value>>),
    Object(Box<HashMap<String, Value>>),
}

#[derive(Debug)]
pub struct Dumper {
    pub indent: usize,
}

impl Dumper {
    pub fn dump(&self, value: &Value) -> String {
        self.visit(value, 0)
    }

    fn visit(&self, value: &Value, level: usize) -> String {
        let indent_str = " ".repeat(self.indent * level);
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Sting(s) => format!("\"{}\"", s),
            Value::Array(arr) => {
                let mut result = String::from("[\n");
                for v in arr.iter() {
                    result.push_str(&format!(
                        "{}{},\n",
                        " ".repeat(self.indent * (level + 1)),
                        self.visit(v, level + 1)
                    ));
                }
                result.push_str(&format!("{}]", indent_str));
                result
            }
            Value::Object(obj) => {
                let mut result = String::from("{\n");
                for (k, v) in obj.iter() {
                    result.push_str(&format!(
                        "{}\"{}\": {},\n",
                        " ".repeat(self.indent * (level + 1)),
                        k,
                        self.visit(v, level + 1)
                    ));
                }
                result.push_str(&format!("{}{}", indent_str, "}"));
                result
            }
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, Error> {
        let current_token = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    fn eat(&mut self, expected: Token) -> Result<(), Error> {
        if self.current_token != expected {
            return Err(Error::ParserError {
                msg: format!(
                    "Expected token {:?}, got {:?}",
                    expected, self.current_token
                ),
                token: self.current_token.clone(),
            });
        }
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    /// json:
    ///  value? EOF
    pub fn json(&mut self) -> Result<Value, Error> {
        match self.current_token {
            Token::EOF => Ok(Value::Null),
            _ => {
                let obj = self.value();
                self.eat(Token::EOF)?;
                obj
            }
        }
    }

    /// value:
    ///  object |
    /// array |
    /// string |
    /// number |
    /// boolean |
    /// null
    fn value(&mut self) -> Result<Value, Error> {
        match &self.current_token {
            Token::LeftBrace => self.object(),
            Token::LeftParen => self.array(),
            Token::String(s) => {
                let value = Value::Sting(s.clone());
                self.eat(Token::String(s.clone()))?;
                Ok(value)
            }
            Token::Number(n) => {
                let value = Value::Number(*n);
                self.eat(Token::Number(*n))?;
                Ok(value)
            }
            Token::Boolean(b) => {
                let value = Value::Bool(*b);
                self.eat(Token::Boolean(*b))?;
                Ok(value)
            }
            Token::Null => {
                self.eat(Token::Null)?;
                Ok(Value::Null)
            }
            _ => Err(Error::ParserError {
                msg: format!("Unexpected token in value: {:?}", self.current_token),
                token: self.current_token.clone(),
            }),
        }
    }

    /// object:
    ///  LeftBrace (property (Comma property)*)? RightBrace
    fn object(&mut self) -> Result<Value, Error> {
        self.eat(Token::LeftBrace)?;
        let mut map = HashMap::new();
        if self.current_token != Token::RightBrace {
            let (key, value) = self.property()?;
            map.insert(key, value);
            while self.current_token == Token::Comma {
                self.eat(Token::Comma)?;
                let (key, value) = self.property()?;
                map.insert(key, value);
            }
        }
        self.eat(Token::RightBrace)?;
        Ok(Value::Object(Box::new(map)))
    }

    /// property:
    /// String Collon value
    fn property(&mut self) -> Result<(String, Value), Error> {
        let key = match &self.current_token {
            Token::String(s) => s.clone(),
            _ => {
                return Err(Error::ParserError {
                    msg: format!(
                        "Expected string as object key, got {:?}",
                        self.current_token
                    ),
                    token: self.current_token.clone(),
                });
            }
        };
        self.eat(Token::String(key.clone()))?;
        self.eat(Token::Collon)?;
        let value = self.value()?;
        Ok((key, value))
    }

    /// array:
    /// LeftParen (value (Comma value)*)? RightParen
    fn array(&mut self) -> Result<Value, Error> {
        self.eat(Token::LeftParen)?;
        let mut arr = Vec::new();
        if self.current_token != Token::RightParen {
            let value = self.value()?;
            arr.push(value);
            while self.current_token == Token::Comma {
                self.eat(Token::Comma)?;
                let value = self.value()?;
                arr.push(value);
            }
        }
        self.eat(Token::RightParen)?;
        Ok(Value::Array(Box::new(arr)))
    }
}
