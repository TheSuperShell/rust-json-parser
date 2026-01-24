mod error;
mod lexer;
mod parser;
mod tokens;

use {
    lexer::Lexer,
    parser::{Dumper, Parser},
};

fn main() {
    let input = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
        },
        "graduation_year": null
    }
    "#;

    let lexer = Lexer::new(input.to_string());
    let parser_result = Parser::new(lexer);
    let mut parser = match parser_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error initializing parser: {}", e);
            return;
        }
    };
    let parsed_json_result = parser.json();
    let parsed_json = match &parsed_json_result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };
    println!("{:#?}", parsed_json);
    let dumper = Dumper { indent: 4 };
    println!("{}", dumper.dump(&parsed_json));
}
