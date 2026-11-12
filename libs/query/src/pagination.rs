use crate::Cursor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub first: Option<i32>,
    pub last: Option<i32>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub offset: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PaginationError {
    #[error("page sizes and offset must be nonnegative")]
    Negative,
    #[error("only one of first and last may be provided")]
    ConflictingSizes,
    #[error("only one of before and after may be provided")]
    ConflictingCursors,
    #[error("first cannot be used with before")]
    ForwardBefore,
    #[error("last cannot be used with after")]
    BackwardAfter,
    #[error("offset cannot be used with backward pagination")]
    BackwardOffset,
}

impl Pagination {
    pub const DEFAULT_MAX_ROWS: u32 = 30;

    pub fn validate(&self) -> Result<(), PaginationError> {
        if [self.first, self.last, self.offset]
            .into_iter()
            .flatten()
            .any(|value| value < 0)
        {
            return Err(PaginationError::Negative);
        }
        if self.first.is_some() && self.last.is_some() {
            return Err(PaginationError::ConflictingSizes);
        }
        if self.before.is_some() && self.after.is_some() {
            return Err(PaginationError::ConflictingCursors);
        }
        if self.first.is_some() && self.before.is_some() {
            return Err(PaginationError::ForwardBefore);
        }
        if self.last.is_some() && self.after.is_some() {
            return Err(PaginationError::BackwardAfter);
        }
        if self.offset.is_some() && (self.last.is_some() || self.before.is_some()) {
            return Err(PaginationError::BackwardOffset);
        }
        Ok(())
    }

    pub fn page_size(&self, max_rows: u32) -> Result<u32, PaginationError> {
        self.validate()?;
        Ok(self
            .first
            .or(self.last)
            .map_or(max_rows, |size| size as u32)
            .min(max_rows))
    }
}
