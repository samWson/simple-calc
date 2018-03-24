use std::fmt;

/// Token Types
/// EOF token is used to indicate that there is no more input left for lexical analysis.
/// A Plus is represented as a String for ease of representation.
enum TokenType {
    Integer(usize),
    Plus(String),
    EOF,
}

struct Interpreter {
    text: String,
    position: u8,
    current_token: TokenType,
}

impl Interpreter {
    
}

#[derive(Debug)]
struct ParseError {
    message: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}
fn main() {
    println!("Hello, world!");
}
