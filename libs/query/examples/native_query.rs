use compallm_query::{
    IntoQuery, Query, QueryTarget,
    operation::{Operation, StringOperation},
};
use std::convert::Infallible;

struct Role;
enum RoleField {
    Name,
}
impl QueryTarget for Role {
    type Field = RoleField;
}

struct NameInput {
    name: String,
}

impl IntoQuery for NameInput {
    type Target = Role;
    type Error = Infallible;

    fn into_query(self) -> Result<Query<Role>, Self::Error> {
        Ok(Query::new().filter(
            RoleField::Name,
            Operation::String(StringOperation::Eq(self.name)),
        ))
    }
}

fn main() {
    let _query: Query<Role> = NameInput {
        name: "Admin".into(),
    }
    .into_query()
    .unwrap();
}
