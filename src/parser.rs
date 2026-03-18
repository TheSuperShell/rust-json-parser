use std::collections::HashMap;

use crate::{error::Error, lexer::Lexer, tokens::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
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
            Value::String(s) => format!("\"{}\"", s),
            Value::Array(arr) => {
                let mut result = String::from("[\n");
                let mut arr = arr.iter().peekable();
                while let Some(v) = arr.next() {
                    result.push_str(&format!(
                        "{}{}",
                        " ".repeat(self.indent * (level + 1)),
                        self.visit(v, level + 1)
                    ));
                    match arr.peek() {
                        Some(_) => result.push_str(",\n"),
                        None => result.push('\n'),
                    }
                }
                result.push_str(&format!("{}]", indent_str));
                result
            }
            Value::Object(obj) => {
                let mut result = String::from("{\n");
                let mut obj = obj.iter().peekable();
                while let Some((k, v)) = obj.next() {
                    result.push_str(&format!(
                        "{}\"{}\": {}",
                        " ".repeat(self.indent * (level + 1)),
                        k,
                        self.visit(v, level + 1)
                    ));
                    match obj.peek() {
                        Some(_) => result.push_str(",\n"),
                        None => result.push('\n'),
                    }
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
                msg: format!("Expected {:?}", expected),
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
            Token::Eof => Ok(Value::Null),
            _ => {
                let obj = self.value()?;
                self.eat(Token::Eof)?;
                Ok(obj)
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
                let value = Value::String(s.clone());
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
                if self.current_token == Token::RightBrace {
                    return Err(Error::ParserError {
                        msg: "Trailing comma in object".to_string(),
                        token: self.current_token.clone(),
                    });
                }
                let (key, value) = self.property()?;
                map.insert(key, value);
            }
        }
        self.eat(Token::RightBrace)?;
        Ok(Value::Object(map))
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
                if self.current_token == Token::RightParen {
                    return Err(Error::ParserError {
                        msg: "Trailing comma in array".to_string(),
                        token: self.current_token.clone(),
                    });
                }
                let value = self.value()?;
                arr.push(value);
            }
        }
        self.eat(Token::RightParen)?;
        Ok(Value::Array(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_object() {
        let input = r#"{"key": "value", "number": 42, "bool": true, "null_value": null}"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parsed = parser.json().unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key".to_string(), Value::String("value".to_string()));
        expected_map.insert("number".to_string(), Value::Number(42.0));
        expected_map.insert("bool".to_string(), Value::Bool(true));
        expected_map.insert("null_value".to_string(), Value::Null);

        assert_eq!(parsed, Value::Object(expected_map));
    }

    #[test]
    fn test_parse_array() {
        let input = r#"["item1", 2, false, null]"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parsed = parser.json().unwrap();

        let expected_array = vec![
            Value::String("item1".to_string()),
            Value::Number(2.0),
            Value::Bool(false),
            Value::Null,
        ];

        assert_eq!(parsed, Value::Array(expected_array));
    }

    #[test]
    fn test_empty() {
        let input = r#""#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parsed = parser.json().unwrap();

        assert_eq!(parsed, Value::Null);
    }

    #[test]
    fn test_nested_structure() {
        let input = r#"{
            "person": {
                "name": "Alice",
                "age": 30,
                "is_student": false,
                "courses": ["Math", "Science"],
                "address": {
                    "street": "123 Main St",
                    "city": "Anytown"
                }
            }
        }"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parsed = parser.json().unwrap();

        let mut address_map = HashMap::new();
        address_map.insert(
            "street".to_string(),
            Value::String("123 Main St".to_string()),
        );
        address_map.insert("city".to_string(), Value::String("Anytown".to_string()));

        let mut person_map = HashMap::new();
        person_map.insert("name".to_string(), Value::String("Alice".to_string()));
        person_map.insert("age".to_string(), Value::Number(30.0));
        person_map.insert("is_student".to_string(), Value::Bool(false));
        person_map.insert(
            "courses".to_string(),
            Value::Array(vec![
                Value::String("Math".to_string()),
                Value::String("Science".to_string()),
            ]),
        );
        person_map.insert("address".to_string(), Value::Object(address_map));

        let mut expected_map = HashMap::new();
        expected_map.insert("person".to_string(), Value::Object(person_map));

        assert_eq!(parsed, Value::Object(expected_map));
    }

    #[test]
    fn test_trainling_comma_in_object() {
        let input = r#"{"key1": "value1", "key2": "value2",}"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let result = parser.json();

        assert!(result.is_err());
        if let Err(Error::ParserError { msg, token }) = result {
            assert_eq!(msg, "Trailing comma in object");
            assert_eq!(token, Token::RightBrace);
        } else {
            panic!("Expected ParserError");
        }
    }

    #[test]
    fn test_trailing_comma_in_array() {
        let input = r#"["item1", "item2",]"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let result = parser.json();

        assert!(result.is_err());
        if let Err(Error::ParserError { msg, token }) = result {
            assert_eq!(msg, "Trailing comma in array");
            assert_eq!(token, Token::RightParen);
        } else {
            panic!("Expected ParserError");
        }
    }

    #[test]
    fn test_unclosed_object() {
        let input = r#"{"key": "value""#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let result = parser.json();

        assert!(result.is_err());
        if let Err(Error::ParserError { msg, token }) = result {
            assert_eq!(msg, "Expected RightBrace");
            assert_eq!(token, Token::Eof);
        } else {
            panic!("Expected ParserError");
        }
    }

    #[test]
    fn test_unclosed_array() {
        let input = r#"["item1", "item2""#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let result = parser.json();

        assert!(result.is_err());
        if let Err(Error::ParserError { msg, token }) = result {
            assert_eq!(msg, "Expected RightParen");
            assert_eq!(token, Token::Eof);
        } else {
            panic!("Expected ParserError");
        }
    }
}
