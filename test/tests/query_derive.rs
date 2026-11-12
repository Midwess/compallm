use compallm_query as query_runtime;
use compallm_query::{
    IntoQuery, Predicate,
    filter::{FilterIs, IntFilter, StringFilter, StringListFilter},
    operation::{Comparison, ListOperation, Operation, StringOperation},
};

#[derive(Default, IntoQuery)]
struct RoleFilter {
    id: Option<IntFilter>,
    name: Option<StringFilter>,
    tags: Option<StringListFilter>,
    and: Option<Vec<RoleFilter>>,
    or: Option<Vec<RoleFilter>>,
    not: Option<Box<RoleFilter>>,
}

fn name(value: &str) -> RoleFilter {
    RoleFilter {
        name: Some(StringFilter {
            eq: Some(value.into()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn derived_filter_preserves_each_operation_and_nested_boolean_group() {
    let query = Box::new(RoleFilter {
        id: Some(IntFilter {
            gte: Some(10),
            lt: Some(100),
            ..Default::default()
        }),
        or: Some(vec![name("Admin"), name("Owner")]),
        not: Some(Box::new(RoleFilter {
            tags: Some(StringListFilter {
                overlaps: Some(vec!["hidden".into()]),
                ..Default::default()
            }),
            ..Default::default()
        })),
        ..Default::default()
    })
    .into_query();
    let Predicate::And(predicates) = query.predicate else {
        panic!("expected and")
    };
    assert_eq!(
        &predicates[..2],
        &[
            Predicate::Field {
                field: "id".into(),
                operation: Operation::Int(Comparison::Gte(10))
            },
            Predicate::Field {
                field: "id".into(),
                operation: Operation::Int(Comparison::Lt(100))
            },
        ]
    );
    assert!(matches!(&predicates[2], Predicate::Or(branches) if branches.len() == 2));
    assert!(
        matches!(&predicates[3], Predicate::Not(child) if matches!(child.as_ref(), Predicate::And(fields) if matches!(&fields[0], Predicate::Field { operation: Operation::StringList(ListOperation::Overlaps(values)), .. } if values == &vec!["hidden".to_string()])))
    );
}

#[derive(IntoQuery)]
#[query(crate = query_runtime)]
struct RenamedFilter<'a, T> {
    #[query(field = "name")]
    search: T,
    #[query(skip)]
    _metadata: &'a str,
}

#[test]
fn explicit_mapping_accepts_operations_and_keeps_metadata_outside_the_query() {
    let query = Box::new(RenamedFilter {
        search: StringOperation::Is(FilterIs::NotNull),
        _metadata: "metadata stays outside the query",
    })
    .into_query();
    assert_eq!(
        query.predicate,
        Predicate::And(vec![Predicate::Field {
            field: "name".into(),
            operation: Operation::String(StringOperation::Is(FilterIs::NotNull))
        }])
    );
}
