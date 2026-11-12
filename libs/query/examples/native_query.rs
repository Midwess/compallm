use compallm_query::{IntoQuery, Query, QueryTarget, filter::StringFilter};

struct Role;
enum RoleField {
    Name,
}
impl QueryTarget for Role {
    type Field = RoleField;
}

#[derive(IntoQuery)]
#[query(target = Role, fields = RoleField)]
struct NameInput {
    name: Option<StringFilter>,
}

fn main() {
    let _query: Query<Role> = NameInput {
        name: Some(StringFilter {
            eq: Some("Admin".into()),
            ..Default::default()
        }),
    }
    .into_query()
    .unwrap();
}
