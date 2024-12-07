use regex::{Match, Regex};

/// Type alias over a function from a [`regex::Match`] instance to a branch of the
/// token type enum.
type Handler<T> = fn(&Match) -> Option<T>;

/// Represents a rule based on which the lexer processes input.
#[derive(Debug)]
pub struct LexerRule<T> {
    /// Pattern that this rule matches
    pat: Regex,
    /// Handler function for the rule - see [`Handler`]
    handler: Handler<T>,
}

impl<T> LexerRule<T> {
    /// Calls this rule's handler function.
    pub fn handle(&self, match_item: &Match) -> Option<T> {
        (self.handler)(match_item)
    }
}

/// Represents a token returned by the lexer.
#[derive(Debug)]
pub struct Token<T> {
    /// A branch of an `enum` supplied by the user of the lexer module
    toktype: T,
    /// The start of the match
    start: usize,
    /// The end of the match
    end: usize,
}

impl<T> Token<T> {
    pub fn toktype(&self) -> &T {
        &self.toktype
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

/// Represents the lexer itself.
#[derive(Debug)]
pub struct Lexer<T> {
    /// The rules on which the lexer operates
    rules: Vec<LexerRule<T>>,
}

impl<T> Lexer<T> {
    /// Create a new lexer.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the lexer.
    pub fn rule(&mut self, pat: &str, handler: Handler<T>) {
        let pat = Regex::new(pat).expect("Error compiling pattern!");
        self.rules.push(LexerRule { pat, handler });
    }

    /// Lex a string.
    pub fn lex(&mut self, s: &str) -> Vec<Token<T>> {
        let mut toks: Vec<Token<T>> = Vec::new();
        let mut matched = vec![false; s.len()];

        for rule in &self.rules {
            for match_item in rule.pat.find_iter(s) {
                let mut is_matched = false;
                let start = match_item.start();
                let end = match_item.end();
                for i in start..end {
                    if matched[i] {
                        is_matched = true;
                    }
                    matched[i] = true;
                }
                if !is_matched {
                    if let Some(toktype) = rule.handle(&match_item) {
                        toks.push(Token {
                            toktype,
                            start,
                            end,
                        });
                    }
                }
            }
        }
        toks.sort_by(|a, b| a.start.cmp(&b.start));
        toks
    }
}

#[cfg(test)]
mod tests {

    use crate::Lexer;

    #[derive(Debug, PartialEq)]
    enum TokType {
        Add,
        Sub,
        Mul,
        Div,
        Number(i32),
    }

    #[test]
    fn basic_lex_test() {
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
}
