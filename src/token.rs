#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    String(String),
    Number(f64),
    True,
    False,
    Null,
    EOF,
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedEOF,
    InvalidCharacter(char),
}
