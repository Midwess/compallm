use compallm_query::{IntoQuery, Query, filter::StringFilter};

#[derive(IntoQuery)]
struct NameInput {
    name: Option<StringFilter>,
}

fn main() {
    let input: Box<dyn IntoQuery> = Box::new(NameInput {
        name: Some(StringFilter {
            eq: Some("Admin".into()),
            ..Default::default()
        }),
    });
    let _query: Query = input.into_query();
}
