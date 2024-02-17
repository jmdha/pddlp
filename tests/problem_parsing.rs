mod blocksworld {
    use pddlp::problem::{self, Fact, Goal, Object};

    pub const BLOCKSWORLD: &'static str = "
        ;; base case
        ;;
        (define (problem blocksworld-01)
         (:domain blocksworld)
         (:objects  b1 b2 - object)
         (:init 
            (arm-empty)
            (clear b2)
            (on-table b2)
            (clear b1)
            (on-table b1)
        )
         (:goal (and 
            (clear b1)
            (on b1 b2)
            (on-table b2)
        )))";

    #[test]
    fn parse() {
        let problem = problem::parse(BLOCKSWORLD).unwrap();
        assert_eq!("blocksworld-01", problem.name.unwrap());
        assert_eq!("blocksworld", problem.domain.unwrap());
        assert_eq!(
            vec![
                Object {
                    name: "b1",
                    type_name: Some("object")
                },
                Object {
                    name: "b2",
                    type_name: Some("object")
                }
            ],
            problem.objects.unwrap()
        );
        assert_eq!(
            vec![
                Fact {
                    predicate: "arm-empty",
                    objects: vec![]
                },
                Fact {
                    predicate: "clear",
                    objects: vec!["b2"]
                },
                Fact {
                    predicate: "on-table",
                    objects: vec!["b2"]
                },
                Fact {
                    predicate: "clear",
                    objects: vec!["b1"]
                },
                Fact {
                    predicate: "on-table",
                    objects: vec!["b1"]
                },
            ],
            problem.init.unwrap()
        );
        assert_eq!(
            Goal::And(vec![
                Goal::Fact(Fact {
                    predicate: "clear",
                    objects: vec!["b1"]
                }),
                Goal::Fact(Fact {
                    predicate: "on",
                    objects: vec!["b1", "b2"]
                }),
                Goal::Fact(Fact {
                    predicate: "on-table",
                    objects: vec!["b2"]
                }),
            ]),
            problem.goal.unwrap()
        );
    }
}
mod ferry {
    use pddlp::problem::{self, Fact, Goal, Object};

    pub const BLOCKSWORLD: &'static str = "
        ;; base case
        ;;
        (define (problem ferry-01)
         (:domain ferry)
         (:objects 
            car1 - car
            loc1 loc2 - location
         )
         (:init 
            (empty-ferry)
            (at-ferry loc1)
            (at car1 loc1)
        )
         (:goal  (and (at car1 loc2))))";

    #[test]
    fn parse() {
        let problem = problem::parse(BLOCKSWORLD);
        assert!(problem.is_ok());
        let problem = problem.unwrap();
        assert_eq!("ferry-01", problem.name.unwrap());
        assert_eq!("ferry", problem.domain.unwrap());
        assert_eq!(
            vec![
                Object {
                    name: "car1",
                    type_name: Some("car")
                },
                Object {
                    name: "loc1",
                    type_name: Some("location")
                },
                Object {
                    name: "loc2",
                    type_name: Some("location")
                }
            ],
            problem.objects.unwrap()
        );
        assert_eq!(
            vec![
                Fact {
                    predicate: "empty-ferry",
                    objects: vec![]
                },
                Fact {
                    predicate: "at-ferry",
                    objects: vec!["loc1"]
                },
                Fact {
                    predicate: "at",
                    objects: vec!["car1", "loc1"]
                },
            ],
            problem.init.unwrap()
        );
        assert_eq!(
            Goal::And(vec![Goal::Fact(Fact {
                predicate: "at",
                objects: vec!["car1", "loc2"]
            }),]),
            problem.goal.unwrap()
        );
    }
}
