use super::{token::Token, Fact, Result};
use logos::Lexer;

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Fact<'a>> {
    let predicate = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        },
        None => return Err(("erroneous open paren", lexer.span())),
    };
    let objects = {
        let mut objects = Vec::new();

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Name(name)) => objects.push(name),
                Ok(Token::RParen) => break,
                Ok(_) => return Err(("unexpected token", lexer.span())),
                Err(_) => return Err(("invalid token", lexer.span())),
            }
        }

        objects
    };
    Ok(Fact { predicate, objects })
}
