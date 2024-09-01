use crate::token::Token;
use super::{Result, Type};
use logos::Lexer;

pub fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Vec<Type<'a>>> {
    let mut types = Vec::new();

    let mut queue: Vec<&'a str> = Vec::new();
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Name(name)) => queue.push(name),
            Ok(Token::TypeSeparator) => match lexer.next() {
                Some(token) => match token {
                    Ok(Token::Name(name)) => {
                        for item in queue.iter() {
                            types.push(Type {
                                name: item,
                                parent: Some(name),
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
        types.push(Type {
            name: item,
            parent: None,
        });
    }

    Ok(types)
}

#[cfg(test)]
mod test {
    use super::{Result, Token, Type};
    use logos::Logos;
    use rstest::rstest;

    #[rstest]
    #[case(Ok(vec![Type { name: "a", parent: None }]), "a)")]
    #[case(Ok(vec![Type { name: "a", parent: Some("b") }]), "a - b)")]
    #[case(Ok(vec![
        Type { name: "a", parent: Some("c") },
        Type { name: "b", parent: Some("c") }
    ]), "a b - c)")]
    #[case(Ok(vec![
        Type { name: "b", parent: Some("c") },
        Type { name: "a", parent: Some("b") }
    ]), "b - c a - b)")]
    #[case(Ok(vec![
        Type { name: "b", parent: Some("c") },
        Type { name: "a", parent: None }
    ]), "b - c a)")]
    fn parse(#[case] expected: Result<Vec<Type>>, #[case] input: &str) {
        let mut lexer = Token::lexer(input);
        assert_eq!(expected, super::parse(&mut lexer));
    }
}
