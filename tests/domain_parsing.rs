mod blocksworld {
    use pddlp::domain::{self, Parameter, Predicate};

    pub const BLOCKSWORLD: &'static str = "
;; source: https://github.com/AI-Planning/pddl-generators/blob/main/blocksworld/domain.pddl
;;
(define (domain blocksworld)

(:requirements :strips)

(:predicates (clear ?x)
             (on-table ?x)
             (arm-empty)
             (holding ?x)
             (on ?x ?y))

(:action pickup
  :parameters (?ob)
  :precondition (and (clear ?ob) (on-table ?ob) (arm-empty))
  :effect (and (holding ?ob) (not (clear ?ob)) (not (on-table ?ob)) 
               (not (arm-empty))))

(:action putdown
  :parameters  (?ob)
  :precondition (holding ?ob)
  :effect (and (clear ?ob) (arm-empty) (on-table ?ob) 
               (not (holding ?ob))))

(:action stack
  :parameters  (?ob ?underob)
  :precondition (and (clear ?underob) (holding ?ob))
  :effect (and (arm-empty) (clear ?ob) (on ?ob ?underob)
               (not (clear ?underob)) (not (holding ?ob))))

(:action unstack
  :parameters  (?ob ?underob)
  :precondition (and (on ?ob ?underob) (clear ?ob) (arm-empty))
  :effect (and (holding ?ob) (clear ?underob)
               (not (on ?ob ?underob)) (not (clear ?ob)) (not (arm-empty)))))";

    #[test]
    fn parse() {
        let domain = domain::parse(BLOCKSWORLD).unwrap();
        assert_eq!("blocksworld", domain.name.unwrap());
        assert_eq!(vec![":strips"], domain.requirements.unwrap());
        assert_eq!(
            vec![
                Predicate {
                    name: "clear",
                    parameters: vec![Parameter {
                        name: "?x",
                        r#type: None
                    }]
                },
                Predicate {
                    name: "on-table",
                    parameters: vec![Parameter {
                        name: "?x",
                        r#type: None
                    }]
                },
                Predicate {
                    name: "arm-empty",
                    parameters: vec![]
                },
                Predicate {
                    name: "holding",
                    parameters: vec![Parameter {
                        name: "?x",
                        r#type: None
                    }]
                },
                Predicate {
                    name: "on",
                    parameters: vec![
                        Parameter {
                            name: "?x",
                            r#type: None
                        },
                        Parameter {
                            name: "?y",
                            r#type: None
                        }
                    ]
                },
            ],
            domain.predicates.unwrap()
        );
    }
}
