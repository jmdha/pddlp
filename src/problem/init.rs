use super::{fact, token::Token, Fact, Result};
use logos::Lexer;

pub(super) fn parse<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Vec<Fact<'a>>> {
    let mut facts = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => facts.push(fact::parse(lexer)?),
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    Ok(facts)
}
