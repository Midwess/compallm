use super::QueryCompileError;
use compallm_query::{
    filter::FilterIs,
    operation::{
        BooleanOperation, Comparison, Equality, ListOperation, OpaqueOperation, Operation,
        StringOperation,
    },
};
use sea_orm::{
    ColumnType,
    sea_query::{
        Alias, ArrayType, BinOper, Condition, Expr, ExprTrait, Func, LikeExpr, Value,
        extension::postgres::PgBinOper,
    },
};

#[derive(Clone, Copy)]
enum Kind {
    Int,
    Float,
    BigInt,
    BigFloat,
    Date,
    Time,
    Datetime,
    String,
    Boolean,
    Uuid,
    Enum,
    Opaque,
}

fn matches_type(kind: Kind, column: &ColumnType) -> bool {
    matches!(
        (kind, column),
        (
            Kind::Int,
            ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger,
        ) | (Kind::Float, ColumnType::Float | ColumnType::Double)
            | (Kind::BigInt, ColumnType::BigInteger)
            | (Kind::BigFloat, ColumnType::Decimal(_))
            | (Kind::Date, ColumnType::Date)
            | (Kind::Time, ColumnType::Time)
            | (
                Kind::Datetime,
                ColumnType::DateTime | ColumnType::Timestamp | ColumnType::TimestampWithTimeZone,
            )
            | (
                Kind::String,
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text
            )
            | (Kind::Boolean, ColumnType::Boolean)
            | (Kind::Uuid, ColumnType::Uuid)
            | (Kind::Enum, ColumnType::Enum { .. })
            | (
                Kind::Opaque,
                ColumnType::Custom(_)
                    | ColumnType::Cidr
                    | ColumnType::Inet
                    | ColumnType::MacAddr
                    | ColumnType::LTree,
            )
    )
}

fn require(kind: Kind, column: &ColumnType) -> Result<(), QueryCompileError> {
    if matches_type(kind, column) {
        Ok(())
    } else {
        Err(QueryCompileError::TypeMismatch)
    }
}

fn type_name(column: &ColumnType) -> Result<&'static str, QueryCompileError> {
    match column {
        ColumnType::TinyInteger | ColumnType::SmallInteger => Ok("smallint"),
        ColumnType::Integer => Ok("integer"),
        ColumnType::BigInteger => Ok("bigint"),
        ColumnType::Float => Ok("real"),
        ColumnType::Double => Ok("double precision"),
        ColumnType::Decimal(_) => Ok("numeric"),
        ColumnType::Date => Ok("date"),
        ColumnType::Time => Ok("time"),
        ColumnType::DateTime | ColumnType::Timestamp => Ok("timestamp"),
        ColumnType::TimestampWithTimeZone => Ok("timestamptz"),
        ColumnType::Char(_) => Ok("character"),
        ColumnType::String(_) => Ok("varchar"),
        ColumnType::Text => Ok("text"),
        ColumnType::Boolean => Ok("boolean"),
        ColumnType::Uuid => Ok("uuid"),
        ColumnType::Cidr => Ok("cidr"),
        ColumnType::Inet => Ok("inet"),
        ColumnType::MacAddr => Ok("macaddr"),
        ColumnType::LTree => Ok("ltree"),
        _ => Err(QueryCompileError::UnsupportedScalar),
    }
}

fn cast(value: String, column: &ColumnType) -> Result<Expr, QueryCompileError> {
    Ok(match column {
        ColumnType::Enum { name, .. } | ColumnType::Custom(name) => {
            Func::cast_as_quoted(Expr::val(value), name.clone()).into()
        }
        _ => Expr::val(value).cast_as(Alias::new(type_name(column)?)),
    })
}

fn null(column: Expr, value: FilterIs) -> Condition {
    Condition::all().add(match value {
        FilterIs::Null => column.is_null(),
        FilterIs::NotNull => column.is_not_null(),
    })
}

fn comparison<T>(
    column: Expr,
    operation: Comparison<T>,
    value: impl Fn(T) -> Result<Expr, QueryCompileError>,
) -> Result<Condition, QueryCompileError> {
    let expression = match operation {
        Comparison::Eq(v) => column.eq(value(v)?),
        Comparison::Neq(v) => column.ne(value(v)?),
        Comparison::Gt(v) => column.gt(value(v)?),
        Comparison::Gte(v) => column.gte(value(v)?),
        Comparison::Lt(v) => column.lt(value(v)?),
        Comparison::Lte(v) => column.lte(value(v)?),
        Comparison::In(values) => {
            if values.is_empty() {
                return Ok(Condition::all().add(Expr::Constant(false.into())));
            }
            column.is_in(
                values
                    .into_iter()
                    .map(value)
                    .collect::<Result<Vec<_>, _>>()?,
            )
        }
        Comparison::Is(v) => return Ok(null(column, v)),
    };
    Ok(Condition::all().add(expression))
}

fn equality<T>(
    column: Expr,
    operation: Equality<T>,
    value: impl Fn(T) -> Result<Expr, QueryCompileError>,
) -> Result<Condition, QueryCompileError> {
    comparison(
        column,
        match operation {
            Equality::Eq(v) => Comparison::Eq(v),
            Equality::Neq(v) => Comparison::Neq(v),
            Equality::In(v) => Comparison::In(v),
            Equality::Is(v) => Comparison::Is(v),
        },
        value,
    )
}

fn string(column: Expr, operation: StringOperation) -> Condition {
    let expression = match operation {
        StringOperation::Eq(v) => column.eq(v),
        StringOperation::Neq(v) => column.ne(v),
        StringOperation::Gt(v) => column.gt(v),
        StringOperation::Gte(v) => column.gte(v),
        StringOperation::Lt(v) => column.lt(v),
        StringOperation::Lte(v) => column.lte(v),
        StringOperation::In(v) => {
            if v.is_empty() {
                return Condition::all().add(Expr::Constant(false.into()));
            }
            column.is_in(v)
        }
        StringOperation::Is(v) => return null(column, v),
        StringOperation::StartsWith(v) => {
            let mut pattern = String::new();
            for ch in v.chars() {
                if matches!(ch, '!' | '%' | '_') {
                    pattern.push('!');
                }
                pattern.push(ch);
            }
            pattern.push('%');
            column.like(LikeExpr::new(pattern).escape('!'))
        }
        StringOperation::Like(v) => column.like(v),
        StringOperation::ILike(v) => column.binary(PgBinOper::ILike, Expr::val(v)),
        StringOperation::Regex(v) => column.binary(PgBinOper::Regex, Expr::val(v)),
        StringOperation::IRegex(v) => column.binary(PgBinOper::RegexCaseInsensitive, Expr::val(v)),
    };
    Condition::all().add(expression)
}

fn array<T>(
    column: Expr,
    column_type: &ColumnType,
    kind: Kind,
    operation: ListOperation<T>,
    array_type: ArrayType,
    value: impl Fn(T) -> Value,
) -> Result<Condition, QueryCompileError> {
    let ColumnType::Array(element) = column_type else {
        return Err(QueryCompileError::TypeMismatch);
    };
    require(kind, element)?;
    let (operator, values) = match operation {
        ListOperation::Contains(v) => (PgBinOper::Contains.into(), v),
        ListOperation::ContainedBy(v) => (PgBinOper::Contained.into(), v),
        ListOperation::Eq(v) => (BinOper::Equal, v),
        ListOperation::Overlaps(v) => (PgBinOper::Overlap.into(), v),
        ListOperation::Is(v) => return Ok(null(column, v)),
    };
    let values = Value::Array(
        array_type,
        Some(Box::new(values.into_iter().map(value).collect())),
    );
    let values = Expr::val(values).cast_as(Alias::new(format!("{}[]", type_name(element)?)));
    Ok(Condition::all().add(column.binary(operator, values)))
}

pub(super) fn translate(
    column: Expr,
    column_type: &ColumnType,
    operation: Operation,
) -> Result<Condition, QueryCompileError> {
    match operation {
        Operation::Id(v) => Ok(Condition::all().add(column.eq(cast(v, column_type)?))),
        Operation::Int(v) => {
            require(Kind::Int, column_type)?;
            comparison(column, v, |v| Ok(Expr::val(v)))
        }
        Operation::Float(v) => {
            require(Kind::Float, column_type)?;
            comparison(column, v, |v| Ok(Expr::val(v)))
        }
        Operation::String(v) => {
            require(Kind::String, column_type)?;
            Ok(string(column, v))
        }
        Operation::Boolean(v) => {
            require(Kind::Boolean, column_type)?;
            Ok(match v {
                BooleanOperation::Eq(v) => Condition::all().add(column.eq(v)),
                BooleanOperation::Is(v) => null(column, v),
            })
        }
        Operation::Enum(v) => {
            require(Kind::Enum, column_type)?;
            equality(column, v, |v| cast(v, column_type))
        }
        Operation::Uuid(v) => {
            require(Kind::Uuid, column_type)?;
            equality(column, v, |v| cast(v.0, column_type))
        }
        Operation::Opaque(v) => {
            require(Kind::Opaque, column_type)?;
            Ok(match v {
                OpaqueOperation::Eq(v) => Condition::all().add(column.eq(cast(v.0, column_type)?)),
                OpaqueOperation::Is(v) => null(column, v),
            })
        }
        Operation::BigInt(v) => {
            require(Kind::BigInt, column_type)?;
            comparison(column, v, |v| cast(v.0, column_type))
        }
        Operation::BigFloat(v) => {
            require(Kind::BigFloat, column_type)?;
            comparison(column, v, |v| cast(v.0, column_type))
        }
        Operation::Date(v) => {
            require(Kind::Date, column_type)?;
            comparison(column, v, |v| cast(v.0, column_type))
        }
        Operation::Time(v) => {
            require(Kind::Time, column_type)?;
            comparison(column, v, |v| cast(v.0, column_type))
        }
        Operation::Datetime(v) => {
            require(Kind::Datetime, column_type)?;
            comparison(column, v, |v| cast(v.0, column_type))
        }
        Operation::IntList(v) => array(column, column_type, Kind::Int, v, ArrayType::Int, |v| {
            v.into()
        }),
        Operation::FloatList(v) => array(
            column,
            column_type,
            Kind::Float,
            v,
            ArrayType::Double,
            |v| v.into(),
        ),
        Operation::StringList(v) => array(
            column,
            column_type,
            Kind::String,
            v,
            ArrayType::String,
            |v| v.into(),
        ),
        Operation::BooleanList(v) => array(
            column,
            column_type,
            Kind::Boolean,
            v,
            ArrayType::Bool,
            |v| v.into(),
        ),
        Operation::UuidList(v) => {
            array(column, column_type, Kind::Uuid, v, ArrayType::String, |v| {
                Value::String(Some(v.0))
            })
        }
        Operation::BigIntList(v) => array(
            column,
            column_type,
            Kind::BigInt,
            v,
            ArrayType::String,
            |v| Value::String(Some(v.0)),
        ),
        Operation::BigFloatList(v) => array(
            column,
            column_type,
            Kind::BigFloat,
            v,
            ArrayType::String,
            |v| Value::String(Some(v.0)),
        ),
        Operation::DateList(v) => {
            array(column, column_type, Kind::Date, v, ArrayType::String, |v| {
                Value::String(Some(v.0))
            })
        }
        Operation::TimeList(v) => {
            array(column, column_type, Kind::Time, v, ArrayType::String, |v| {
                Value::String(Some(v.0))
            })
        }
        Operation::DatetimeList(v) => array(
            column,
            column_type,
            Kind::Datetime,
            v,
            ArrayType::String,
            |v| Value::String(Some(v.0)),
        ),
    }
}
