use super::{parameters, token::Token, Action, Expression, Result};
use logos::Lexer;

fn parse_name<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<&'a str> {
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => Ok(name),
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        },
        None => return Err(("erroneous action", lexer.span())),
    }
}

fn parse_expression<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Expression<'a>> {
    let token = lexer
        .next()
        .ok_or(("unexpected end of input", lexer.span()))?;
    println!("{:?}", token);
    match token {
        Ok(Token::Name(name)) => {
            let mut parameters = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::Name(name)) => parameters.push(name),
                    Ok(Token::RParen) => break,
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Ok(Expression::Fact {
                predicate: name,
                parameters,
            });
        }
        Ok(Token::Not) => {
            let n_token = lexer
                .next()
                .ok_or(("unexpected end of input", lexer.span()))?;
            let expression = match n_token {
                Ok(Token::LParen) => parse_expression(lexer),
                _ => return Err(("unexpected token", lexer.span())),
            }?;
            match lexer.next() {
                Some(token) => match token {
                    Ok(Token::RParen) => {}
                    Ok(_) => return Err(("unexpected token", lexer.span())),
                    Err(_) => return Err(("invalid token", lexer.span())),
                },
                None => return Err(("unexpected end of input", lexer.span())),
            }
            return Ok(Expression::Not(Box::new(expression)));
        }
        Ok(Token::And) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => {
                        return Ok(Expression::And(expressions))
                    }
                    Ok(Token::LParen) => {
                        expressions.push(parse_expression(lexer)?)
                    }
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Err(("unexpected end of input", lexer.span()));
        }
        Ok(Token::Or) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => {
                        return Ok(Expression::Or(expressions))
                    }
                    Ok(Token::LParen) => {
                        expressions.push(parse_expression(lexer)?)
                    }
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Err(("unexpected end of input", lexer.span()));
        }
        _ => return Err(("unexpected token", lexer.span())),
    }
}

fn parse_expression_root<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
) -> Result<Expression<'a>> {
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::LParen) => parse_expression(lexer),
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        },
        None => return Err(("unexpected end of input", lexer.span())),
    }
}

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Action<'a>> {
    let name = parse_name(lexer)?;
    let mut parameters = None;
    let mut precondition = None;
    let mut effect = None;

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Parameters) => match lexer.next() {
                Some(token) => match token {
                    Ok(Token::LParen) => {
                        parameters = Some(parameters::parse(lexer)?)
                    }
                    Ok(_) => return Err(("unexpected token", lexer.span())),
                    Err(_) => return Err(("invalid token", lexer.span())),
                },
                None => return Err(("unexpected end of input", lexer.span())),
            },
            Ok(Token::Precondition) => {
                precondition = Some(parse_expression_root(lexer)?)
            }
            Ok(Token::Effect) => effect = Some(parse_expression_root(lexer)?),
            Ok(Token::LParen) => continue,
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    let effect = match effect {
        Some(effect) => effect,
        None => return Err(("action missing effect", lexer.span())),
    };

    Ok(Action {
        name,
        parameters,
        precondition,
        effect,
    })
}
