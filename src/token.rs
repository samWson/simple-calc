/// token defines the tokens that are produced after lexical analysis
/// of an input string into a lexer. The tokens can then be the input
/// for a parser to produce an abstract syntax tree.
mod token {

    /// Token represents a specific 'part of speech' of a programming language.
    /// A token will also have a literal string value of how it appears in an
    /// input string for a lexer.
    #[derive(Debug, PartialEq)]
    pub enum Token {
        Integer(String),
        Operator(String),
    }
}
