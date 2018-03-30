use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Token {
    Start,
    Integer(usize),
    Plus,
    EOF,
}

#[derive(Debug)]
struct TokenError {
    message: String,
}

struct Interpreter<'a> {
    characters: std::str::Chars<'a>,
    current_token: Token,
}

impl<'a> Interpreter<'a> {

    /// get_next_token is a lexical analyser, also known as a scanner or tokenizer.
    /// It returns a Result containing either a Token or a TokenError.
    fn get_next_token(&mut self) -> Result<Token, TokenError> {
        let current_char = self.characters.next();

        match current_char {
            Some(value) => match value {
                value if value.is_numeric() => return Ok(Token::Integer(usize::from_str(&value.to_string()).unwrap())),
                '+' => return Ok(Token::Plus),
                _ => Err(TokenError {message: "Error: attempted to tokenize unkown character".to_string()}),
            },
            None => return Ok(Token::EOF),
        }
    }

    /// eat_integer expects that the interpreters current token is an integer. If the current token
    /// is correct the next token will be scanned and its Result returned. An incorrect token will result in a panic.
    fn eat_integer(&mut self) -> Result<Token, TokenError> {
        match self.current_token {
            Token::Integer(_value) => {
                return self.get_next_token();
            },
            _ => panic!("Syntax error: Expected Token::Integer. Found {:?}", self.current_token),
        }
    }
}

#[test]
fn test_get_next_token() {
    let mut interpreter = Interpreter {
        characters: "4+5".chars(),
        current_token: Token::Start,
    };

    assert_eq!(interpreter.get_next_token().unwrap(), Token::Integer(4));
    assert_eq!(interpreter.get_next_token().unwrap(), Token::Plus);
    assert_eq!(interpreter.get_next_token().unwrap(), Token::Integer(5));
    assert_eq!(interpreter.get_next_token().unwrap(), Token::EOF);
}

#[test]
fn test_eat_integer() {
    let mut interpreter = Interpreter {
        characters: "+5".chars(), // We have already consumed the first integer: 3, from this iterator.
        current_token: Token::Integer(3),
    };

    let token = interpreter.eat_integer().unwrap();

    assert_eq!(token, Token::Plus);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = &args[1];

    let mut interpreter = Interpreter {
        characters: text.chars(),
        current_token: Token::Start,
    };

    println!("The provided expression is: {}", text);
    println!("The tokens are:");
    println!("{:?}", interpreter.get_next_token().unwrap());
    println!("{:?}", interpreter.get_next_token().unwrap());
    println!("{:?}", interpreter.get_next_token().unwrap());
    println!("{:?}", interpreter.get_next_token().unwrap());
}
