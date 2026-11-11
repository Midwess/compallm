//! Constant-size JSON number, string, and UTF-8 state shared by request profiles.
mod number;
mod string;
mod utf8;

pub(crate) use number::{JsonFloat, Number};
pub(crate) use string::{JsonString, Piece};
pub(crate) use utf8::Utf8;
