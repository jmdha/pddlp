use super::{parameters, token::Token, Predicate, Result};
use logos::Lexer;

fn parse_predicate<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Predicate<'a>> {
    let name = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        },
        None => return Err(("erroneous open paren", lexer.span())),
    };
    let parameters = parameters::parse(lexer)?;
    Ok(Predicate { name, parameters })
}

pub fn parse<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Vec<Predicate<'a>>> {
    let mut predicates = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => predicates.push(parse_predicate(lexer)?),
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    Ok(predicates)
}
