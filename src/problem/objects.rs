use crate::token::Token;
use super::{Object, Result};
use logos::Lexer;

pub(super) fn parse<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<Vec<Object<'a>>> {
    let mut objects = Vec::new();

    let mut object_names: Vec<&'a str> = Vec::new();
    let mut awaiting_type = false;
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Name(name)) => match awaiting_type {
                true => {
                    for object in object_names.iter() {
                        objects.push(Object {
                            name: object,
                            type_name: Some(name),
                        });
                    }
                    object_names.clear();
                    awaiting_type = false;
                }
                false => object_names.push(name),
            },
            Ok(Token::TypeSeparator) => awaiting_type = true,
            Ok(Token::RParen) => break,
            Ok(_) => return Err(("unexpected token", lexer.span())),
            Err(_) => return Err(("invalid token", lexer.span())),
        }
    }

    match awaiting_type {
        true => return Err(("erroneous type separator", lexer.span())),
        false => {
            for object in object_names.into_iter() {
                objects.push(Object {
                    name: object,
                    type_name: None,
                });
            }
        }
    }

    Ok(objects)
}

#[cfg(test)]
mod test {
    use super::{Object, Result, Token};
    use logos::Logos;
    use rstest::rstest;

    #[rstest]
    #[case(Ok(vec![Object { name: "a", type_name: None }]), "a)")]
    #[case(Ok(vec![Object { name: "a", type_name: Some("b") }]), "a - b)")]
    #[case(Ok(vec![
        Object { name: "a", type_name: Some("c") }, 
        Object { name: "b", type_name: Some("c") }
    ]), "a b - c)")]
    fn parse(#[case] expected: Result<Vec<Object>>, #[case] input: &str) {
        let mut lexer = Token::lexer(input);
        assert_eq!(expected, super::parse(&mut lexer));
    }
}
