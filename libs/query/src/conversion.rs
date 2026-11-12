use crate::{filter::*, operation::*};

macro_rules! filter_operations {
    ($filter:ty, $family:ident, $operation:ident, { $($field:ident => $variant:ident),* $(,)? }) => {
        impl From<$filter> for Vec<Operation> {
            fn from(value: $filter) -> Self {
                let mut operations = Vec::new();
                $(if let Some(value) = value.$field {
                    operations.push(Operation::$family($operation::$variant(value)));
                })*
                operations
            }
        }
    };
}

macro_rules! operation_value {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Operation {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        }
        impl From<$ty> for Vec<Operation> {
            fn from(value: $ty) -> Self {
                vec![value.into()]
            }
        }
    };
}

impl From<Operation> for Vec<Operation> {
    fn from(value: Operation) -> Self {
        vec![value]
    }
}

filter_operations!(IntFilter, Int, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<i32>, Int);

filter_operations!(FloatFilter, Float, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<f64>, Float);

filter_operations!(BigIntFilter, BigInt, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<crate::scalar::BigInt>, BigInt);

filter_operations!(BigFloatFilter, BigFloat, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<crate::scalar::BigFloat>, BigFloat);

filter_operations!(DateFilter, Date, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<crate::scalar::Date>, Date);

filter_operations!(TimeFilter, Time, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<crate::scalar::Time>, Time);

filter_operations!(DatetimeFilter, Datetime, Comparison, { eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte, in_list => In, is => Is });
operation_value!(Comparison<crate::scalar::Datetime>, Datetime);

filter_operations!(StringFilter, String, StringOperation, {
    eq => Eq, neq => Neq, gt => Gt, gte => Gte, lt => Lt, lte => Lte,
    in_list => In, is => Is, starts_with => StartsWith, like => Like,
    ilike => ILike, regex => Regex, iregex => IRegex,
});
operation_value!(StringOperation, String);
filter_operations!(BooleanFilter, Boolean, BooleanOperation, { eq => Eq, is => Is });
operation_value!(BooleanOperation, Boolean);
filter_operations!(UUIDFilter, Uuid, Equality, { eq => Eq, neq => Neq, in_list => In, is => Is });
operation_value!(Equality<crate::scalar::UUID>, Uuid);
filter_operations!(OpaqueFilter, Opaque, OpaqueOperation, { eq => Eq, is => Is });
operation_value!(OpaqueOperation, Opaque);

impl From<IDFilter> for Vec<Operation> {
    fn from(value: IDFilter) -> Self {
        value
            .eq
            .into_iter()
            .map(|value| Operation::Id(value.to_string()))
            .collect()
    }
}

impl<T: Into<String>> From<EnumFilter<T>> for Vec<Operation> {
    fn from(value: EnumFilter<T>) -> Self {
        let mut operations = Vec::new();
        if let Some(value) = value.eq {
            operations.push(Operation::Enum(Equality::Eq(value.into())));
        }
        if let Some(value) = value.neq {
            operations.push(Operation::Enum(Equality::Neq(value.into())));
        }
        if let Some(values) = value.in_list {
            operations.push(Operation::Enum(Equality::In(
                values.into_iter().map(Into::into).collect(),
            )));
        }
        if let Some(value) = value.is {
            operations.push(Operation::Enum(Equality::Is(value)));
        }
        operations
    }
}
operation_value!(Equality<String>, Enum);

filter_operations!(IntListFilter, IntList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<i32>, IntList);

filter_operations!(FloatListFilter, FloatList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<f64>, FloatList);

filter_operations!(BigIntListFilter, BigIntList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::BigInt>, BigIntList);

filter_operations!(BigFloatListFilter, BigFloatList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::BigFloat>, BigFloatList);

filter_operations!(DateListFilter, DateList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::Date>, DateList);

filter_operations!(TimeListFilter, TimeList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::Time>, TimeList);

filter_operations!(DatetimeListFilter, DatetimeList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::Datetime>, DatetimeList);

filter_operations!(StringListFilter, StringList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<String>, StringList);

filter_operations!(BooleanListFilter, BooleanList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<bool>, BooleanList);

filter_operations!(UUIDListFilter, UuidList, ListOperation, { contains => Contains, contained_by => ContainedBy, eq => Eq, overlaps => Overlaps, is => Is });
operation_value!(ListOperation<crate::scalar::UUID>, UuidList);
