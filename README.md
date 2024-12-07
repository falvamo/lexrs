# lexrs
A Regex-based lexer inspired by Flex and implemented in Rust.

## Example usage
```rust
use lexrs::Lexer;

#[derive(Debug, PartialEq)]
enum TokType {
    Add,
    Sub,
    Mul,
    Div,
    Number(i32),
}

fn main() {
    let mut lexer = Lexer::new();

    lexer.rule(r"\+", |_| Some(TokType::Add));
    lexer.rule(r"-", |_| Some(TokType::Sub));
    lexer.rule(r"\*", |_| Some(TokType::Mul));
    lexer.rule(r"/", |_| Some(TokType::Div));
    lexer.rule(r"[0-9]+", |match_item| {
        Some(TokType::Number(match_item.as_str().parse().unwrap()))
    });

    let tokens = lexer.lex("5 + 10");
    assert_eq!(tokens[0].toktype(), &TokType::Number(5));
    assert_eq!(tokens[1].toktype(), &TokType::Add);
    assert_eq!(tokens[2].toktype(), &TokType::Number(10));
}
```