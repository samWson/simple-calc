use std::fmt;

/// Token Types
/// Start can be thought of as the start of a token sequence and has no other value.
/// EOF token is used to indicate that there is no more input left for lexical analysis.
/// A Plus is represented as a String for ease of representation.
#[derive(PartialEq, Debug)]
enum Token {
    Start,
    Integer(u32),
    Plus(char),
    EOF,
}

struct Interpreter {
    text: String,
    position: usize,
    current_token: Token,
}

impl Interpreter {

    /// get_next_token is a Lexical analyser (also known as a scanner or tokenizer).
    /// This method is responsible for breaking a sentence apart into tokens. One
    /// token at a time.
    fn get_next_token(&mut self) -> Result<Token, ParseError> {
        let mut chars = self.text.chars();

        // Is self.position index past the end of self.text.chars?
        // If so there is no more input to tokenize.
        if self.position > self.text.chars().count() - 1 {
            return Ok(Token::EOF);
        }

        let current_char = chars.nth(self.position).unwrap();

        if current_char.is_numeric() {
            self.position += 1;
            let integer = current_char.to_digit(10).unwrap();
            return Ok(Token::Integer(integer));
        }

        if current_char == '+' {
            self.position += 1;
            return Ok(Token::Plus(current_char));
        }

        Err(ParseError{message: "Error parsing token".to_string()})
    }
}

#[test]
fn test_get_next_token_integer() {
    let mut condition_integer = Interpreter {
        text: "3+5".to_string(),
        position: 0,
        current_token: Token::Start,
    };

    assert_eq!(condition_integer.get_next_token().expect("Error while testing"), Token::Integer(3));
    assert_eq!(condition_integer.position, 1);
}

#[test]
fn test_get_next_token_plus() {
    let mut condition_plus = Interpreter {
        text: "3+5".to_string(),
        position: 1,
        current_token: Token::Integer(3),
    };

    assert_eq!(condition_plus.get_next_token().expect("Error while testing"), Token::Plus('+'));
    assert_eq!(condition_plus.position, 2);
}

#[test]
fn test_get_next_token_eof() {
    let mut condition_end_of_file = Interpreter {
        text: "3+5".to_string(),
        position: 4,
        current_token: Token::Integer(5),
    };

    assert_eq!(condition_end_of_file.get_next_token().expect("Error while testing"), Token::EOF);
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
