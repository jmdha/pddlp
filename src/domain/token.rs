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

    #[regex("(?i)domain")]
    DomainName,

    #[regex(":(?i)requirements")]
    Requirements,

    #[regex(":[a-zA-Z][a-zA-Z0-9-_]*")]
    Requirement(&'a str),

    #[regex(":(?i)types")]
    Types,

    #[regex(":(?i)predicates")]
    Predicates,

    #[regex(":(?i)action")]
    Action,

    #[regex(":(?i)parameters")]
    Parameters,

    #[regex(":(?i)precondition")]
    Precondition,

    #[regex(":(?i)effect")]
    Effect,

    #[regex(":(?i)init")]
    Init,

    #[regex("=")]
    Equal,

    #[regex("(?i)and")]
    And,

    #[regex("(?i)or")]
    Or,

    #[regex("(?i)not")]
    Not,

    #[token("-")]
    TypeSeparator,

    #[regex("[a-zA-Z?][a-zA-Z0-9-_]*")]
    Name(&'a str),
}
