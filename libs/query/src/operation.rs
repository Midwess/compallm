use crate::{
    filter::FilterIs,
    scalar::{BigFloat, BigInt, Date, Datetime, Opaque, Time, UUID},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Comparison<T> {
    Eq(T),
    Neq(T),
    Gt(T),
    Gte(T),
    Lt(T),
    Lte(T),
    In(Vec<T>),
    Is(FilterIs),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Equality<T> {
    Eq(T),
    Neq(T),
    In(Vec<T>),
    Is(FilterIs),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StringOperation {
    Eq(String),
    Neq(String),
    Gt(String),
    Gte(String),
    Lt(String),
    Lte(String),
    In(Vec<String>),
    Is(FilterIs),
    StartsWith(String),
    Like(String),
    ILike(String),
    Regex(String),
    IRegex(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BooleanOperation {
    Eq(bool),
    Is(FilterIs),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ListOperation<T> {
    Contains(Vec<T>),
    ContainedBy(Vec<T>),
    Eq(Vec<T>),
    Overlaps(Vec<T>),
    Is(FilterIs),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OpaqueOperation {
    Eq(Opaque),
    Is(FilterIs),
}

/// Database-independent operands; values never contain SQL expressions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Id(String),
    Int(Comparison<i32>),
    Float(Comparison<f64>),
    BigInt(Comparison<BigInt>),
    BigFloat(Comparison<BigFloat>),
    Date(Comparison<Date>),
    Time(Comparison<Time>),
    Datetime(Comparison<Datetime>),
    String(StringOperation),
    Boolean(BooleanOperation),
    Uuid(Equality<UUID>),
    Opaque(OpaqueOperation),
    Enum(Equality<String>),
    IntList(ListOperation<i32>),
    FloatList(ListOperation<f64>),
    BigIntList(ListOperation<BigInt>),
    BigFloatList(ListOperation<BigFloat>),
    DateList(ListOperation<Date>),
    TimeList(ListOperation<Time>),
    DatetimeList(ListOperation<Datetime>),
    StringList(ListOperation<String>),
    BooleanList(ListOperation<bool>),
    UuidList(ListOperation<UUID>),
}
