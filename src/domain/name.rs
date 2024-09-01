use super::Result;
use crate::token::Token;
use logos::Lexer;

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<&'a str> {
    let name = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            _ => return Err(("unexpected token", lexer.span())),
        },
        None => return Err(("unexpected end of input", lexer.span())),
    };
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::RParen) => {}
            _ => return Err(("unexpected token", lexer.span())),
        },
        None => return Err(("unexpected end of input", lexer.span())),
    };
    Ok(name)
}
