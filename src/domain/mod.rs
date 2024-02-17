mod action;
mod name;
mod parameters;
mod predicates;
mod requirements;
mod token;
mod types;

use self::token::Token;
use super::Result;
use logos::Logos;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Domain<'a> {
    pub name: Option<&'a str>,
    pub requirements: Option<Vec<&'a str>>,
    pub types: Option<Vec<Type<'a>>>,
    pub predicates: Option<Vec<Predicate<'a>>>,
    pub actions: Vec<Action<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type<'a> {
    pub name: &'a str,
    /// The parent of a the type
    pub parent: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Predicate<'a> {
    pub name: &'a str,
    pub parameters: Vec<Parameter<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub r#type: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Action<'a> {
    pub name: &'a str,
    pub parameters: Option<Vec<Parameter<'a>>>,
    pub precondition: Option<Expression<'a>>,
    pub effect: Expression<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression<'a> {
    Fact {
        predicate: &'a str,
        parameters: Vec<&'a str>,
    },
    Equal(Vec<Expression<'a>>),
    And(Vec<Expression<'a>>),
    Or(Vec<Expression<'a>>),
    Not(Box<Expression<'a>>),
}

pub fn parse<'a>(input: &'a str) -> Result<Domain<'a>> {
    let mut name = None;
    let mut requirements = None;
    let mut types = None;
    let mut predicates = None;
    let mut actions = Vec::new();

    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::DomainName) => name = Some(name::parse(&mut lexer)?),
            Ok(Token::Requirements) => {
                requirements = Some(requirements::parse(&mut lexer)?)
            }
            Ok(Token::Types) => types = Some(types::parse(&mut lexer)?),
            Ok(Token::Predicates) => {
                predicates = Some(predicates::parse(&mut lexer)?)
            }
            Ok(Token::Action) => actions.push(action::parse(&mut lexer)?),
            Ok(_) => continue,
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    Ok(Domain {
        name,
        requirements,
        types,
        predicates,
        actions,
    })
}
