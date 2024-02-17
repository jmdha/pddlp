use super::{token::Token, Step};
use crate::Result;
use logos::Lexer;

pub(super) fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Step<'a>> {
    let action = match lexer.next() {
        Some(token) => match token {
            Ok(Token::Name(name)) => name,
            Ok(_) => return Err(("Unexpected Token", lexer.span())),
            Err(_) => return Err(("Invalid Token", lexer.span())),
        },
        None => return Err(("Erroneous LParen", lexer.span())),
    };
    let objects = {
        let mut objects = Vec::new();
        let mut r_paren = false;

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Name(name)) => objects.push(name),
                Ok(Token::RParen) => {
                    r_paren = true;
                    break;
                }
                Ok(_) => return Err(("Unexpected Token", lexer.span())),
                Err(_) => return Err(("Invalid Token", lexer.span())),
            }
        }

        if !r_paren {
            return Err(("Missing RParen", lexer.span()));
        }

        objects
    };
    Ok(Step { action, objects })
}

#[cfg(test)]
mod test {
    use super::{Result, Step, Token};
    use logos::Logos;
    use rstest::rstest;

    #[rstest]
    #[case(Ok(crate::plan::Step { action: "a", objects: vec![] }), "a)")]
    #[case(Ok(crate::plan::Step { action: "a", objects: vec!["b"] }), "a b)")]
    #[case(Ok(crate::plan::Step { action: "a", objects: vec!["b", "c"] }), "a b c)")]
    #[case(Err(("Missing RParen", 1..1)), "a")]
    #[case(Err(("Unexpected Token", 0..1)), ")")]
    fn parse(#[case] expected: Result<Step>, #[case] input: &str) {
        let mut lexer = Token::lexer(input);
        assert_eq!(expected, super::parse(&mut lexer));
    }
}
