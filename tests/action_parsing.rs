use pddlp::domain::{self, Action};

#[test]
fn parse() {
    assert_eq!(
        Ok(Action {
            name: "a1",
            parameters: None,
            precondition: None,
            effect: domain::Expression::And(vec![domain::Expression::Fact {
                predicate: "p1",
                parameters: vec![]
            }])
        }),
        pddlp::domain::action::parse(
            "(:action a1
    :effect(and (p1))
)"
        )
    );
}
