use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Token {
    Integer(usize),
    Plus,
}

#[derive(Debug)]
struct TokenError {
    message: String,
}

struct Interpreter<'a> {
    characters: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {

    /// get_next_token is a lexical analyser, also known as a scanner or tokenizer.
    /// It returns the appropriate token for each of the characters in the interpreter.
    /// Tokenizing errors return a TokenError.
    fn get_next_token(&mut self) -> Result<Token, TokenError> {
        let current_char = self.characters.next();

        match current_char {
            Some(value) => match value {
                value if value.is_numeric() => return Ok(Token::Integer(usize::from_str(&value.to_string()).unwrap())),
                '+' => return Ok(Token::Plus),
                _ => Err(TokenError {message: "Error tokenizing character".to_string()}),
            },
            None => return Err(TokenError {message: "No value to tokenize".to_string()}),
        }
    }
}

#[test]
    fn test_get_next_token() {
        let mut interpreter = Interpreter {
            characters: "4+5".chars(),
        };

        assert_eq!(interpreter.get_next_token().unwrap(), Token::Integer(4));
        assert_eq!(interpreter.get_next_token().unwrap(), Token::Plus);
        assert_eq!(interpreter.get_next_token().unwrap(), Token::Integer(5));
    }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = &args[1];

    let interpreter = Interpreter {
        characters: text.chars(),
    };

    println!("The provided expression is: {}", text);
}
