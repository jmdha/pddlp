mod step;
mod token;

use self::token::Token;
use crate::Result;
use logos::Logos;

/// Denotes a single step in a plan
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Step<'a> {
    pub action: &'a str,
    pub objects: Vec<&'a str>,
}

/// A plan denotes a sequence of steps
/// Which is the solution to planning task
pub type Plan<'a> = Vec<Step<'a>>;

/// Tries parsing input as a pddl plan
/// In case of error, returns Err with error msg
pub fn parse(input: &str) -> Result<Plan> {
    let mut lexer = Token::lexer(input);

    let mut steps = vec![];
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LParen) => steps.push(step::parse(&mut lexer)?),
            Ok(_) => return Err(("Unexpected Token", lexer.span())),
            Err(_) => return Err(("Invalid Token", lexer.span())),
        }
    }

    Ok(steps)
}

#[cfg(test)]
mod test {
    use super::{Plan, Result, Step};
    use rstest::rstest;

    #[rstest]
    #[case("(a)", Ok(vec![crate::plan::Step { action: "a", objects: vec![] }]))]
    #[case("(a)(b)", Ok(vec![
        Step { action: "a", objects: vec![] },
        Step { action: "b", objects: vec![] }]
    ))]
    fn parse(#[case] input: &str, #[case] expected: Result<Plan>) {
        assert_eq!(expected, super::parse(input));
    }
}
