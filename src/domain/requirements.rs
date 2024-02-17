use super::{token::Token, Result};
use logos::Lexer;

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Vec<&'a str>> {
    let mut requirements = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Requirement(requirement)) => {
                requirements.push(requirement)
            }
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    Ok(requirements)
}
