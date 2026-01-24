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
            "city": "Anytown"
        },
        "graduation_year": null
    }
    "#;

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let parsed_json = parser.json();
    println!("{:#?}", parsed_json);
    let dumper = Dumper { indent: 4 };
    println!("{}", dumper.dump(&parsed_json));
}
