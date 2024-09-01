mod fact;
mod goal;
mod init;
mod name;
mod objects;

use crate::token::Token;

use super::Result;
use logos::Logos;

/// Represents a PDDL problem
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Problem<'a> {
    /// Name of the problem
    pub name: Option<&'a str>,
    /// Name of associated domain
    pub domain: Option<&'a str>,
    /// The objects consisting of name and type
    pub objects: Option<Vec<Object<'a>>>,
    /// List of initial state facts
    pub init: Option<Vec<Fact<'a>>>,
    /// List of goal state facts
    pub goal: Option<Goal<'a>>,
}

/// Denotes an object
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Object<'a> {
    pub name: &'a str,
    pub type_name: Option<&'a str>,
}

/// Denotes a goal condition
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Goal<'a> {
    Fact(Fact<'a>),
    And(Vec<Goal<'a>>),
    Or(Vec<Goal<'a>>),
    Not(Box<Goal<'a>>),
}

/// Denotes a fact of either init and goal
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fact<'a> {
    pub predicate: &'a str,
    pub objects: Vec<&'a str>,
}

pub fn parse<'a>(input: &'a str) -> Result<Problem<'a>> {
    let mut name = None;
    let mut domain = None;
    let mut objects = None;
    let mut init = None;
    let mut goal = None;

    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::ProblemName) => name = Some(name::parse(&mut lexer)?),
            Ok(Token::DomainDefinition) => domain = Some(name::parse(&mut lexer)?),
            Ok(Token::Objects) => objects = Some(objects::parse(&mut lexer)?),
            Ok(Token::Init) => init = Some(init::parse(&mut lexer)?),
            Ok(Token::Goal) => goal = Some(goal::parse(&mut lexer)?),
            Ok(_) => continue,
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    Ok(Problem {
        name,
        domain,
        objects,
        init,
        goal,
    })
}
