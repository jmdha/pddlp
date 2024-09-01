use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+|;.*")]
pub(super) enum Token<'a> {
    // SHARED
    #[token("-")]
    TypeSeparator,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex("=")]
    Equal,
    #[regex("(?i)and")]
    And,
    #[regex("(?i)or")]
    Or,
    #[regex("(?i)not")]
    Not,
    #[regex("(?i)define")]
    Definition,
    #[regex("[a-zA-Z?][a-zA-Z0-9-_]*")]
    Name(&'a str),

    // DOMAIN
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

    // PROBLEM
    #[regex(":(?i)domain")]
    DomainDefinition,
    #[regex("(?i)problem")]
    ProblemName,
    #[regex(":(?i)objects")]
    Objects,
    #[regex(":(?i)init")]
    Init,
    #[regex(":(?i)goal")]
    Goal,
}
