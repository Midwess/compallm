use crate::llm::text::component::Collection;
use serde::de::DeserializeSeed;
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use std::marker::PhantomData;

pub(super) struct CollectionWire<A>(PhantomData<A>);

impl<T, S: AsRef<str>, A: SerializeAs<T>> SerializeAs<Collection<T, S>> for CollectionWire<A> {
    fn serialize_as<R: Serializer>(
        value: &Collection<T, S>,
        serializer: R,
    ) -> Result<R::Ok, R::Error> {
        match value {
            Collection::Owned(values) => <Vec<A>>::serialize_as(values, serializer),
            Collection::View(view) => serde::Serialize::serialize(view, serializer),
        }
    }
}

impl<'de, T, A: DeserializeAs<'de, T>> DeserializeAs<'de, Collection<T, &'de str>>
    for CollectionWire<A>
{
    fn deserialize_as<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Collection<T, &'de str>, D::Error> {
        struct Items<T, A>(PhantomData<(T, A)>);
        impl<'de, T, A: DeserializeAs<'de, T>> DeserializeSeed<'de> for Items<T, A> {
            type Value = ();
            fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
                compallm_serde_rules::deserialize::validate_sequence::<
                    serde_with::de::DeserializeAsWrap<T, A>,
                    D,
                >(deserializer)
            }
        }
        crate::json::JsonView::deserialize_collection(deserializer, Items::<T, A>(PhantomData))
            .map(Collection::View)
    }
}
