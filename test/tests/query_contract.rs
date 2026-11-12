use compallm_query::{
    IntoQuery, Predicate, Query, QueryTarget,
    operation::{Operation, StringOperation},
};

struct Role;
#[derive(Clone, Debug, PartialEq, Eq)]
enum RoleField {
    Name,
}
impl QueryTarget for Role {
    type Field = RoleField;
}

fn name(value: &str) -> Predicate<RoleField> {
    Predicate::Field {
        field: RoleField::Name,
        operation: Operation::String(StringOperation::Eq(value.into())),
    }
}

#[test]
fn adding_a_filter_preserves_an_existing_or_group() {
    let alternatives = Predicate::Or(vec![name("Admin"), name("Owner")]);
    let query = Query::<Role> {
        predicate: alternatives.clone(),
        ..Default::default()
    }
    .filter(
        RoleField::Name,
        Operation::String(StringOperation::StartsWith("A".into())),
    )
    .into_query()
    .unwrap();
    let Predicate::And(predicates) = query.predicate else {
        panic!("expected conjunctive query")
    };
    assert_eq!(predicates[0], alternatives);
    assert_eq!(predicates.len(), 2);
}

#[test]
fn successive_filters_preserve_their_predicates_without_reordering() {
    let query = Query::<Role>::new()
        .filter(
            RoleField::Name,
            Operation::String(StringOperation::Eq("Admin".into())),
        )
        .filter(
            RoleField::Name,
            Operation::String(StringOperation::Neq("Owner".into())),
        );
    assert_eq!(
        query.predicate,
        Predicate::And(vec![
            name("Admin"),
            Predicate::Field {
                field: RoleField::Name,
                operation: Operation::String(StringOperation::Neq("Owner".into()))
            },
        ])
    );
}
