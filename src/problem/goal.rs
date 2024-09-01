use crate::token::Token;
use super::{Fact, Goal, Result};
use logos::Lexer;

//  NOTE: assumes opening bracket '(' is consumed
fn parse_expression<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Goal<'a>> {
    let token = lexer
        .next()
        .ok_or(("unexpected end of input", lexer.span()))?;
    match token {
        Ok(Token::Name(name)) => {
            let mut objects = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::Name(name)) => objects.push(name),
                    Ok(Token::RParen) => break,
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Ok(Goal::Fact(Fact {
                predicate: name,
                objects,
            }));
        }
        Ok(Token::Not) => {
            let n_token = lexer
                .next()
                .ok_or(("unexpected end of input", lexer.span()))?;
            let expression = match n_token {
                Ok(Token::LParen) => parse_expression(lexer),
                _ => return Err(("unexpected token", lexer.span())),
            }?;
            return Ok(Goal::Not(Box::new(expression)));
        }
        Ok(Token::And) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => return Ok(Goal::And(expressions)),
                    Ok(Token::LParen) => expressions.push(parse_expression(lexer)?),
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Err(("unexpected end of input", lexer.span()));
        }
        Ok(Token::Or) => {
            let mut expressions = Vec::new();

            while let Some(token) = lexer.next() {
                match token {
                    Ok(Token::RParen) => return Ok(Goal::Or(expressions)),
                    Ok(Token::LParen) => expressions.push(parse_expression(lexer)?),
                    _ => return Err(("unexpected token", lexer.span())),
                }
            }

            return Err(("unexpected end of input", lexer.span()));
        }
        _ => return Err(("unexpected token", lexer.span())),
    }
}

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Goal<'a>> {
    match lexer.next() {
        Some(token) => match token {
            Ok(Token::LParen) => {}
            _ => return Err(("unexpected token", lexer.span())),
        },
        None => return Err(("unexpected end of input", lexer.span())),
    };

    let goal = parse_expression(lexer)?;

    match lexer.next() {
        Some(token) => match token {
            Ok(Token::RParen) => {}
            _ => return Err(("unexpected token", lexer.span())),
        },
        None => return Err(("unexpected end of input", lexer.span())),
    };
    Ok(goal)
}
