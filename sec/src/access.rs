use std::{any::TypeId, collections::HashMap};

/// Declared execution-local resource access. Shared<T> services synchronize themselves.
#[derive(Clone, Default)]
pub struct Access {
    exclusive: bool,
    resources: HashMap<TypeId, bool>,
}
impl Access {
    pub fn exclusive() -> Self {
        Self {
            exclusive: true,
            ..Self::default()
        }
    }
    pub fn read<T: 'static>(mut self) -> Self {
        self.resources.entry(TypeId::of::<T>()).or_insert(false);
        self
    }
    pub fn write<T: 'static>(mut self) -> Self {
        self.resources.insert(TypeId::of::<T>(), true);
        self
    }
    pub fn merge(&mut self, other: Self) {
        self.exclusive |= other.exclusive;
        for (id, write) in other.resources {
            *self.resources.entry(id).or_default() |= write;
        }
    }
    pub fn compatible(&self, other: &Self) -> bool {
        !self.exclusive
            && !other.exclusive
            && self.resources.iter().all(|(id, write)| {
                other
                    .resources
                    .get(id)
                    .is_none_or(|other_write| !write && !other_write)
            })
    }
}
