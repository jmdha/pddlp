use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+|;.*")]
pub(super) enum Token<'a> {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("[a-zA-Z][a-zA-Z0-9-_]*")]
    Name(&'a str),
}
