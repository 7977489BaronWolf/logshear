/// Minimal query DSL lexer for logshear.
///
/// Supports tokens like field selectors, comparison operators,
/// logical connectives, and string/number literals.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    StringLit(String),
    NumberLit(f64),
    Eq,
    NotEq,
    Contains,
    And,
    Or,
    Not,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug)]
pub struct LexError(pub String);

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.advance();
        }
    }

    fn read_string(&mut self) -> Result<Token, LexError> {
        self.advance(); // consume opening quote
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') => return Ok(Token::StringLit(s)),
                Some('\\') => match self.advance() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some(c) => s.push(c),
                    None => return Err(LexError("Unexpected end in string escape".into())),
                },
                Some(c) => s.push(c),
                None => return Err(LexError("Unterminated string literal".into())),
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while self.peek().map(|c| c.is_ascii_digit() || c == '.').unwrap_or(false) {
            s.push(self.advance().unwrap());
        }
        Token::NumberLit(s.parse().unwrap_or(0.0))
    }

    fn read_ident(&mut self) -> Token {
        let mut s = String::new();
        while self.peek().map(|c| c.is_alphanumeric() || c == '_' || c == '.').unwrap_or(false) {
            s.push(self.advance().unwrap());
        }
        match s.to_lowercase().as_str() {
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "contains" => Token::Contains,
            _ => Token::Ident(s),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None => { tokens.push(Token::Eof); break; }
                Some('"') => tokens.push(self.read_string()?),
                Some('(') => { self.advance(); tokens.push(Token::LParen); }
                Some(')') => { self.advance(); tokens.push(Token::RParen); }
                Some('=') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); }
                    tokens.push(Token::Eq);
                }
                Some('!') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(Token::NotEq); }
                    else { tokens.push(Token::Not); }
                }
                Some(c) if c.is_ascii_digit() => tokens.push(self.read_number()),
                Some(c) if c.is_alphabetic() || c == '_' => tokens.push(self.read_ident()),
                Some(c) => return Err(LexError(format!("Unexpected character: {:?}", c))),
            }
        }
        Ok(tokens)
    }
}
