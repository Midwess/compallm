// Adapted from modql 0.7.0, Copyright (c) 2024 Jeremy Chone.
// Licensed under MIT; see libs/core/LICENSE-modql.

mod order_by {
    // Adapted from modql 0.7.0, Copyright (c) 2024 Jeremy Chone.
    // Licensed under MIT; see libs/core/LICENSE-modql.

    // region:    --- OrderBy
    #[derive(Debug, Clone)]
    pub enum OrderBy {
        Asc(String),
        Desc(String),
    }

    impl core::fmt::Display for OrderBy {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                OrderBy::Asc(val) => {
                    let content = quote_first_part(val);
                    fmt.write_str(&content)?;
                    fmt.write_str(" ")?;
                    fmt.write_str("ASC")?;
                }
                OrderBy::Desc(val) => {
                    let content = quote_first_part(val);
                    fmt.write_str(&content)?;
                    fmt.write_str(" ")?;
                    fmt.write_str("DESC")?;
                }
            };

            Ok(())
        }
    }

    // region:    --- Formatter
    // NOTE: Probably need to be generalized for other construct of this crate
    fn quote_first_part(s: &str) -> String {
        if let Some((first, rest)) = s.split_once(' ') {
            format!("{} {}", quote_piece(first), rest)
        } else {
            quote_piece(s)
        }
    }

    fn quote_piece(piece: &str) -> String {
        if piece.contains('.') {
            piece
                .split('.')
                .map(|part| format!("\"{part}\"",))
                .collect::<Vec<_>>()
                .join(".")
        } else {
            format!("\"{piece}\"",)
        }
    }
    // endregion: --- Formatter

    impl<T: AsRef<str>> From<T> for OrderBy {
        fn from(val: T) -> Self {
            let raw: &str = val.as_ref();

            if let Some(stripped) = raw.strip_prefix('!') {
                OrderBy::Desc(stripped.to_string())
            } else {
                OrderBy::Asc(raw.to_string())
            }
        }
    }

    // endregion: --- OrderBy

    // region:    --- OrderBys
    #[derive(Debug, Clone)]
    pub struct OrderBys(Vec<OrderBy>);

    impl OrderBys {
        pub fn new(v: Vec<OrderBy>) -> Self {
            OrderBys(v)
        }
        pub fn order_bys(self) -> Vec<OrderBy> {
            self.0
        }

        pub fn join_for_sql(&self) -> String {
            self.0
                .iter()
                .map(|o| format!("{o}"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// Builders with_
    impl OrderBys {
        pub fn push(&mut self, order_by: impl Into<OrderBy>) {
            self.0.push(order_by.into());
        }

        pub fn append(mut self, order_by: impl Into<OrderBy>) -> Self {
            self.0.push(order_by.into());
            self
        }
    }

    // This will allow us to iterate over &OrderBys
    impl<'a> IntoIterator for &'a OrderBys {
        type Item = &'a OrderBy;
        type IntoIter = std::slice::Iter<'a, OrderBy>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    // This will allow us to iterate over OrderBys directly (consuming it)
    impl IntoIterator for OrderBys {
        type Item = OrderBy;
        type IntoIter = std::vec::IntoIter<OrderBy>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    // NOTE: If we want the Vec<T> and T, we have to make the individual from
    //       specific to the type. Otherwise, conflict.

    impl From<&str> for OrderBys {
        fn from(val: &str) -> Self {
            OrderBys(vec![val.into()])
        }
    }
    impl From<&String> for OrderBys {
        fn from(val: &String) -> Self {
            OrderBys(vec![val.into()])
        }
    }
    impl From<String> for OrderBys {
        fn from(val: String) -> Self {
            OrderBys(vec![val.into()])
        }
    }

    impl From<OrderBy> for OrderBys {
        fn from(val: OrderBy) -> Self {
            OrderBys(vec![val])
        }
    }

    impl<T: AsRef<str>> From<Vec<T>> for OrderBys {
        fn from(val: Vec<T>) -> Self {
            let d = val
                .into_iter()
                .map(|o| OrderBy::from(o))
                .collect::<Vec<_>>();
            OrderBys(d)
        }
    }

    // endregion: --- OrderBys

    // region:    --- with-sea-query
    #[cfg(feature = "with-sea-query")]
    mod with_sea_query {
        use super::*;
        use sea_query::Alias;
        use sea_query::IntoColumnRef;

        impl OrderBys {
            pub fn into_sea_col_order_iter(
                self,
            ) -> impl Iterator<Item = (sea_query::ColumnRef, sea_query::Order)> {
                self.0.into_iter().map(OrderBy::into_sea_col_order)
            }
        }

        impl OrderBy {
            pub fn into_sea_col_order(self) -> (sea_query::ColumnRef, sea_query::Order) {
                let (col, order) = match self {
                    OrderBy::Asc(col) => (Alias::new(col), sea_query::Order::Asc),
                    OrderBy::Desc(col) => (Alias::new(col), sea_query::Order::Desc),
                };

                (col.into_column_ref(), order)
            }
        }
    }
    // endregion: --- with-sea-query
}
mod order_bys_de {
    // Adapted from modql 0.7.0, Copyright (c) 2024 Jeremy Chone.
    // Licensed under MIT; see libs/core/LICENSE-modql.

    use super::{OrderBy, OrderBys};
    use serde::{Deserialize, Deserializer, de};
    use std::fmt;

    impl<'de> Deserialize<'de> for OrderBys {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(OrderBysVisitor)
        }
    }

    struct OrderBysVisitor;

    impl<'de> de::Visitor<'de> for OrderBysVisitor {
        type Value = OrderBys; // for deserialize

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "OrderBysVisitor visitor not implemented for this type."
            )
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(OrderBy::from(v).into())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(OrderBy::from(v.to_string()).into())
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut order_bys: Vec<OrderBy> = Vec::new();

            while let Some(string) = seq.next_element::<String>()? {
                order_bys.push(OrderBy::from(string));
            }

            Ok(OrderBys::new(order_bys))
        }
        // FIXME: Needs to add support for visit_seq
    }
}

pub use order_by::*;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_bys: Option<OrderBys>,
}

/// Constructors
impl ListOptions {
    pub fn from_limit(limit: i64) -> Self {
        Self {
            limit: Some(limit),
            ..Default::default()
        }
    }

    pub fn from_offset_limit(offset: i64, limit: i64) -> Self {
        Self {
            limit: Some(limit),
            offset: Some(offset),
            ..Default::default()
        }
    }

    pub fn from_order_bys(order_bys: impl Into<OrderBys>) -> Self {
        Self {
            order_bys: Some(order_bys.into()),
            ..Default::default()
        }
    }
}

/// Builders with_
impl ListOptions {
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_order_bys(mut self, order_bys: impl Into<OrderBys>) -> Self {
        self.order_bys = Some(order_bys.into());
        self
    }

    pub fn append_order_by(mut self, order_by: impl Into<OrderBy>) -> Self {
        let order_by = order_by.into();
        if let Some(order_bys) = &mut self.order_bys {
            order_bys.push(order_by);
        } else {
            self.order_bys = Some(OrderBys::from(order_by));
        }
        self
    }
}

// region:    --- Froms

impl From<OrderBys> for ListOptions {
    fn from(val: OrderBys) -> Self {
        Self {
            order_bys: Some(val),
            ..Default::default()
        }
    }
}

impl From<OrderBys> for Option<ListOptions> {
    fn from(val: OrderBys) -> Self {
        Some(ListOptions {
            order_bys: Some(val),
            ..Default::default()
        })
    }
}

impl From<OrderBy> for ListOptions {
    fn from(val: OrderBy) -> Self {
        Self {
            order_bys: Some(OrderBys::from(val)),
            ..Default::default()
        }
    }
}

impl From<OrderBy> for Option<ListOptions> {
    fn from(val: OrderBy) -> Self {
        Some(ListOptions {
            order_bys: Some(OrderBys::from(val)),
            ..Default::default()
        })
    }
}

// endregion: --- Froms

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
    use super::*;
    use sea_query::SelectStatement;

    impl ListOptions {
        pub fn apply_to_sea_query(self, select_query: &mut SelectStatement) {
            fn as_positive_u64(num: i64) -> u64 {
                if num < 0 { 0 } else { num as u64 }
            }
            if let Some(limit) = self.limit {
                select_query.limit(as_positive_u64(limit)); // Note: Negative == 0
            }

            if let Some(offset) = self.offset {
                select_query.offset(as_positive_u64(offset)); // Note: Negative == 0
            }

            if let Some(order_bys) = self.order_bys {
                for (col, order) in order_bys.into_sea_col_order_iter() {
                    select_query.order_by(col, order);
                }
            }
        }
    }
}
// endregion: --- with-sea-query
