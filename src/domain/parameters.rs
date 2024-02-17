use super::{token::Token, Parameter, Result};
use logos::Lexer;

pub fn parse<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Vec<Parameter<'a>>> {
    let mut parameters = Vec::new();

    let mut queue: Vec<&'a str> = Vec::new();
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Name(name)) => queue.push(name),
            Ok(Token::TypeSeparator) => match lexer.next() {
                Some(token) => match token {
                    Ok(Token::Name(name)) => {
                        for item in queue.iter() {
                            parameters.push(Parameter {
                                name: item,
                                r#type: Some(name),
                            });
                        }
                        queue.clear();
                    }
                    Ok(_) => return Err(("unexpected token", lexer.span())),
                    Err(_) => return Err(("invalid token", lexer.span())),
                },
                None => return Err(("erroneous type separator", lexer.span())),
            },
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    for item in queue.iter() {
        parameters.push(Parameter {
            name: item,
            r#type: None,
        });
    }

    Ok(parameters)
}
