use crate::scalar::{BigFloat, BigInt, Date, Datetime, Opaque, Time, UUID};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum FilterIs {
    Null,
    NotNull,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct IntFilter {
    pub eq: Option<i32>,
    pub gt: Option<i32>,
    pub gte: Option<i32>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<i32>>,
    pub lt: Option<i32>,
    pub lte: Option<i32>,
    pub neq: Option<i32>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct FloatFilter {
    pub eq: Option<f64>,
    pub gt: Option<f64>,
    pub gte: Option<f64>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<f64>>,
    pub lt: Option<f64>,
    pub lte: Option<f64>,
    pub neq: Option<f64>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BigIntFilter {
    pub eq: Option<BigInt>,
    pub gt: Option<BigInt>,
    pub gte: Option<BigInt>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<BigInt>>,
    pub lt: Option<BigInt>,
    pub lte: Option<BigInt>,
    pub neq: Option<BigInt>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BigFloatFilter {
    pub eq: Option<BigFloat>,
    pub gt: Option<BigFloat>,
    pub gte: Option<BigFloat>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<BigFloat>>,
    pub lt: Option<BigFloat>,
    pub lte: Option<BigFloat>,
    pub neq: Option<BigFloat>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct DateFilter {
    pub eq: Option<Date>,
    pub gt: Option<Date>,
    pub gte: Option<Date>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<Date>>,
    pub lt: Option<Date>,
    pub lte: Option<Date>,
    pub neq: Option<Date>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct TimeFilter {
    pub eq: Option<Time>,
    pub gt: Option<Time>,
    pub gte: Option<Time>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<Time>>,
    pub lt: Option<Time>,
    pub lte: Option<Time>,
    pub neq: Option<Time>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct DatetimeFilter {
    pub eq: Option<Datetime>,
    pub gt: Option<Datetime>,
    pub gte: Option<Datetime>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<Datetime>>,
    pub lt: Option<Datetime>,
    pub lte: Option<Datetime>,
    pub neq: Option<Datetime>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct StringFilter {
    pub eq: Option<String>,
    pub gt: Option<String>,
    pub gte: Option<String>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<String>>,
    pub lt: Option<String>,
    pub lte: Option<String>,
    pub neq: Option<String>,
    pub is: Option<FilterIs>,
    #[serde(rename = "startsWith")]
    #[cfg_attr(feature = "graphql", graphql(name = "startsWith"))]
    pub starts_with: Option<String>,
    pub like: Option<String>,
    pub ilike: Option<String>,
    pub regex: Option<String>,
    pub iregex: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BooleanFilter {
    pub eq: Option<bool>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct UUIDFilter {
    pub eq: Option<UUID>,
    pub neq: Option<UUID>,
    #[serde(rename = "in")]
    #[cfg_attr(feature = "graphql", graphql(name = "in"))]
    pub in_list: Option<Vec<UUID>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct OpaqueFilter {
    pub eq: Option<Opaque>,
    pub is: Option<FilterIs>,
}

#[cfg(feature = "graphql")]
type Id = juniper::ID;
#[cfg(not(feature = "graphql"))]
type Id = String;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct IDFilter {
    pub eq: Option<Id>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct StringListFilter {
    pub contains: Option<Vec<String>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<String>>,
    pub eq: Option<Vec<String>>,
    pub overlaps: Option<Vec<String>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct IntListFilter {
    pub contains: Option<Vec<i32>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<i32>>,
    pub eq: Option<Vec<i32>>,
    pub overlaps: Option<Vec<i32>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct FloatListFilter {
    pub contains: Option<Vec<f64>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<f64>>,
    pub eq: Option<Vec<f64>>,
    pub overlaps: Option<Vec<f64>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BooleanListFilter {
    pub contains: Option<Vec<bool>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<bool>>,
    pub eq: Option<Vec<bool>>,
    pub overlaps: Option<Vec<bool>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct UUIDListFilter {
    pub contains: Option<Vec<UUID>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<UUID>>,
    pub eq: Option<Vec<UUID>>,
    pub overlaps: Option<Vec<UUID>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BigIntListFilter {
    pub contains: Option<Vec<BigInt>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<BigInt>>,
    pub eq: Option<Vec<BigInt>>,
    pub overlaps: Option<Vec<BigInt>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct BigFloatListFilter {
    pub contains: Option<Vec<BigFloat>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<BigFloat>>,
    pub eq: Option<Vec<BigFloat>>,
    pub overlaps: Option<Vec<BigFloat>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct DateListFilter {
    pub contains: Option<Vec<Date>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<Date>>,
    pub eq: Option<Vec<Date>>,
    pub overlaps: Option<Vec<Date>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct TimeListFilter {
    pub contains: Option<Vec<Time>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<Time>>,
    pub eq: Option<Vec<Time>>,
    pub overlaps: Option<Vec<Time>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLInputObject))]
pub struct DatetimeListFilter {
    pub contains: Option<Vec<Datetime>>,
    #[serde(rename = "containedBy")]
    #[cfg_attr(feature = "graphql", graphql(name = "containedBy"))]
    pub contained_by: Option<Vec<Datetime>>,
    pub eq: Option<Vec<Datetime>>,
    pub overlaps: Option<Vec<Datetime>>,
    pub is: Option<FilterIs>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumFilter<T> {
    pub eq: Option<T>,
    pub neq: Option<T>,
    #[serde(rename = "in")]
    pub in_list: Option<Vec<T>>,
    pub is: Option<FilterIs>,
}

impl<T> Default for EnumFilter<T> {
    fn default() -> Self {
        Self {
            eq: None,
            neq: None,
            in_list: None,
            is: None,
        }
    }
}
