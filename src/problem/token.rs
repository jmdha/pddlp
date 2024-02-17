use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+|;.*")]
pub(super) enum Token<'a> {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex("(?i)define")]
    Definition,

    #[regex("(?i)problem")]
    ProblemName,

    #[regex(":(?i)domain")]
    DomainName,

    #[regex(":(?i)objects")]
    Objects,

    #[regex(":(?i)init")]
    Init,

    #[regex(":(?i)goal")]
    Goal,

    #[regex("(?i)and")]
    And,

    #[regex("(?i)or")]
    Or,

    #[regex("(?i)not")]
    Not,

    #[token("-")]
    TypeSeparator,

    #[regex("[a-zA-Z][a-zA-Z0-9-_]*")]
    Name(&'a str),
}
