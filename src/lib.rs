mod error;
mod lexer;
mod parser;
mod tokens;

pub type Value = parser::Value;
pub type Error = error::Error;

use {lexer::Lexer, parser::Parser};

pub fn parse_json(input: &str) -> Result<Value, Error> {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer)?;
    parser.json()
}

pub fn dump_json(value: &Value, indent: usize) -> String {
    let dumper = parser::Dumper { indent };
    dumper.dump(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_and_dump() {
        let input = r#"{
            "name": "Alice",
            "age": 25,
            "is_student": true,
            "courses": ["Math", "Science"],
            "address": null
        }"#;

        let parsed = parse_json(input).unwrap();
        let dumped = dump_json(&parsed, 4);
        let reparsed = parse_json(&dumped).unwrap();

        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_invalid_json() {
        let input = r#"{
            "name": "Alice",
            "age": 25,
            "is_student": true,
            "courses": ["Math", "Science",
            "address": null
        }"#;

        let result = parse_json(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_json() {
        let input = r#""#;

        let result = parse_json(input).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn simple_json() {
        let input = "{\"key\": \"value\"}";
        let parsed = parse_json(input).unwrap();
        let mut expected = std::collections::HashMap::new();
        expected.insert("key".to_string(), Value::String("value".to_string()));
        assert_eq!(parsed, Value::Object(expected));
    }
}
