use crate::token::{LexError, Token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::EOF);
        }

        let ch = self.advance();

        match ch {
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '"' => self.read_string(),
            c if c.is_ascii_digit() || c == '-' => self.read_number(c),
            c if c.is_ascii_alphabetic() => self.read_literal(c),
            _ => Err(LexError::InvalidCharacter(ch)),
        }
    }

    fn read_string(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::UnexpectedEOF); // missing closing quote
        }

        let s: String = self.input[start..self.position].iter().collect();
        self.advance(); // skip closing quote

        Ok(Token::String(s))
    }

    fn read_number(&mut self, first: char) -> Result<Token, LexError> {
        let mut num_str = String::new();

        num_str.push(first);

        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            num_str.push(self.advance());
        }

        let value = num_str.parse::<f64>().unwrap();
        Ok(Token::Number(value))
    }

    fn read_literal(&mut self, first: char) -> Result<Token, LexError> {
        let mut ident = String::new();
        ident.push(first);

        while !self.is_at_end() && self.peek().is_ascii_alphabetic() {
            ident.push(self.advance());
        }

        match ident.as_str() {
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "null" => Ok(Token::Null),
            _ => Err(LexError::InvalidCharacter(first)), // could return error token
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;
        ch
    }
}
