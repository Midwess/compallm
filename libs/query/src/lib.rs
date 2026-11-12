pub mod connection;
pub mod cursor;
pub use cursor::CursorError;
mod conversion;
pub mod filter;
pub mod operation;
pub mod order;
pub mod pagination;
pub mod query;
pub mod scalar;

pub use connection::{Connection, Edge, PageInfo};
pub use order::{OrderBy, OrderByDirection};
pub use pagination::{Pagination, PaginationError};
pub use query::{IntoQuery, Predicate, Query};
pub use scalar::Cursor;

#[cfg(feature = "graphql")]
pub use juniper;

#[cfg(feature = "derive")]
pub use compallm_query_derive::IntoQuery;
