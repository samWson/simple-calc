/// Token represents a specific 'part of speech' of a programming language. A token will also have a value of what it specifically
/// represents.
#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(u32),
    Operator(Symbol),
}

#[derive(Debug, PartialEq)]
/// Symbol is a specific value for tokens, such as mathematical operators.
pub enum Symbol {
    Plus,
}

/// Tokenizer is used for turning the parts of a type into individual tokens.
pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Vec<Token> {
        let mut characters = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        loop {
            match characters.peek() {
                Some(&ch) => match ch {
                    '0'...'9' => {
                        tokens.push(Token::Integer(ch.to_digit(10).unwrap()));
                        characters.next();
                    },
                    '+' => {
                        tokens.push(Token::Operator(Symbol::Plus));
                        characters.next();
                    },
                    _ => panic!("Unrecognized character")
                },
                None => break // No more characters to tokenize.
            }
        }
        tokens
    }
}

#[test]
fn test_tokenize() {
    let expression = "3+5".to_string();
    let boundry_case = "0+9".to_string();
    let empty = "".to_string();

    assert_eq!(expression.tokenize(), vec![Token::Integer(3), Token::Operator(Symbol::Plus), Token::Integer(5)]);
    assert_eq!(boundry_case.tokenize(), vec![Token::Integer(0), Token::Operator(Symbol::Plus), Token::Integer(9)]);
    assert_eq!(empty.tokenize(), vec![]);
}

