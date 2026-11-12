use compallm_query::{
    IntoQuery, Predicate, Query,
    operation::{Operation, StringOperation},
};

fn name(value: &str) -> Predicate {
    Predicate::Field {
        field: "name".into(),
        operation: Operation::String(StringOperation::Eq(value.into())),
    }
}

#[test]
fn adding_a_filter_preserves_an_existing_or_group() {
    let alternatives = Predicate::Or(vec![name("Admin"), name("Owner")]);
    let query = Box::new(
        Query {
            predicate: alternatives.clone(),
        }
        .filter(
            "name",
            Operation::String(StringOperation::StartsWith("A".into())),
        ),
    )
    .into_query();
    let Predicate::And(predicates) = query.predicate else {
        panic!("expected conjunctive query")
    };
    assert_eq!(predicates[0], alternatives);
    assert_eq!(predicates.len(), 2);
}

#[test]
fn successive_filters_preserve_their_predicates_without_reordering() {
    let query = Query::new()
        .filter(
            "name",
            Operation::String(StringOperation::Eq("Admin".into())),
        )
        .filter(
            "name",
            Operation::String(StringOperation::Neq("Owner".into())),
        );
    assert_eq!(
        query.predicate,
        Predicate::And(vec![
            name("Admin"),
            Predicate::Field {
                field: "name".into(),
                operation: Operation::String(StringOperation::Neq("Owner".into()))
            },
        ])
    );
}
