# lexrs
A Regex-based lexer inspired by flex and implemented in Rust.

## Example usage
```rust
use lexrs::Lexer;

enum TokType {
    Add,
    Sub,
    Mul,
    Div
    Number(i32),
}

fn main() {
    let lexer = Lexer::new();

    lexer.rule(r"\+", |_| TokType::Add);
    lexer.rule(r"-", |_| TokType::Sub);
    lexer.rule(r"\*", |_| TokType::Mul);
    lexer.rule(r"/", |_| TokType::Div);
    lexer.rule(r"[0-9]+", |match_item| TokType::Number(match_item.as_str().parse().unwrap()) )

    let tokens = lexer.lex("5 + 10");
}
```