use crate::ast::*;
use crate::lexer::*;
use crate::token::*;
use std::iter::Iterator;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
struct Parser<Tokens: Iterator<Item = Token>> {
    tokens: Tokens
}


// TODO: implement token iterator for lexer
impl<Tokens: Iterator<Item = Token>> Parser<Tokens>  {
    fn from_tokens(tokens: Tokens) -> Self {
        Parser{tokens}
    }


    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap_or(Token::EOF)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_from_token_vector() {
        let tokens = vec![Token::LET, Token::IDENT{begin:0,end:6}, Token::INTEGER(8)];
        let mut parser = Parser::from_tokens(tokens.into_iter());
        println!("{:?}", parser);
        let t1 = parser.tokens.next().unwrap_or(Token::EOF);
        println!("{:?}", t1);
    }

    #[test]
    fn test_next_token() {
        let tokens = vec![Token::LET, Token::IDENT{begin:0,end:6}, Token::INTEGER(8)];
        let mut parser = Parser::from_tokens(tokens.into_iter());
        let t1 = parser.next_token();
        println!("{:?}", t1);
    }


}
