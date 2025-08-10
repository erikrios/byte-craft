mod lexer;
mod token;

use lexer::Lexer;
use token::{LexError, Token};

fn main() {
    let json = r#"{ "name": "Erik R", "age": 3, "isMale": true }"#;
    let lexer = Lexer::new(json);
    print_lexer(lexer);

    let json = r#"{ "name": "Erik R, "age": 3, "isMale": true }"#;
    let lexer = Lexer::new(json);
    print_lexer(lexer);

    let json = r#"{ "name": "Erik R", "age": 3, "isMale: true }"#;
    let lexer = Lexer::new(json);
    print_lexer(lexer);
}

fn print_lexer(mut lexer: Lexer) {
    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => {
                println!("End of file reached.");
                break;
            }
            Ok(token) => println!("Token: {token:?}"),
            Err(e) => match e {
                LexError::UnexpectedEOF => {
                    eprintln!("Lexer error: UnexpectedEOF {e:?}");
                    break;
                }
                LexError::InvalidCharacter(ch) => {
                    eprintln!("Lexer error: InvalidCharacter {ch} {e:?}");
                    break;
                }
            },
        }
    }
}
