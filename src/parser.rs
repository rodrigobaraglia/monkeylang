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
        let t1 = parser.next_token();
        println!("{:?}", t1);
        assert_eq!(t1, Token::LET);
        let t2 = parser.next_token();
        println!("{:?}", t2);
        assert_eq!(t2, Token::IDENT{begin:0,end:6});
        let t3 = parser.next_token();
        println!("{:?}", t3);
        assert_eq!(t3, Token::INTEGER(8))
    }

}
