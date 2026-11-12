use serde::{Deserialize, Serialize};

macro_rules! string_scalar {
    ($name:ident) => {
        #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        #[cfg_attr(feature = "graphql", derive(juniper::GraphQLScalar))]
        #[cfg_attr(feature = "graphql", graphql(transparent))]
        pub struct $name(pub String);

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.into())
            }
        }
    };
}

string_scalar!(Cursor);
string_scalar!(BigInt);
string_scalar!(BigFloat);
string_scalar!(Date);
string_scalar!(Time);
string_scalar!(Datetime);
string_scalar!(UUID);
string_scalar!(Opaque);
string_scalar!(JSON);
