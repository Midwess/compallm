use std::{
    any::{Any, TypeId, type_name},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    io,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Execution-local typed storage. Runtime borrow checks keep parameter injection safe.
#[derive(Default)]
pub struct Resources {
    values: HashMap<TypeId, RefCell<Box<dyn Any + Send + Sync>>>,
    pending: RefCell<Vec<Box<dyn InsertResource>>>,
    shared: Arc<SharedResources>,
}

impl Resources {
    pub(crate) fn with_shared(shared: Arc<SharedResources>) -> Self {
        Self {
            shared,
            ..Self::default()
        }
    }

    pub fn get_shared<T: Send + Sync + 'static>(&self) -> io::Result<Shared<T>> {
        let value = self
            .shared
            .values
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        let value = Arc::clone(value)
            .downcast::<T>()
            .map_err(|_| missing::<T>())?;
        Ok(Shared { value })
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.values
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(value)));
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> io::Result<Res<'_, T>> {
        let cell = self
            .values
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        let value = cell.try_borrow().map_err(|_| conflicting::<T>())?;
        let value = Ref::filter_map(value, |value| value.downcast_ref::<T>())
            .map_err(|_| missing::<T>())?;
        Ok(Res { value })
    }

    pub fn get_mut<T: Send + Sync + 'static>(&self) -> io::Result<ResMut<'_, T>> {
        let cell = self
            .values
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        let value = cell.try_borrow_mut().map_err(|_| conflicting::<T>())?;
        let value = RefMut::filter_map(value, |value| value.downcast_mut::<T>())
            .map_err(|_| missing::<T>())?;
        Ok(ResMut { value })
    }
}

fn missing<T>() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        format!("missing resource {}", type_name::<T>()),
    )
}

fn conflicting<T>() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("conflicting resource access {}", type_name::<T>()),
    )
}

pub struct Res<'a, T> {
    value: Ref<'a, T>,
}
impl<T> Deref for Res<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

pub struct ResMut<'a, T> {
    value: RefMut<'a, T>,
}
impl<T> Deref for ResMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// State belonging to this processor instance, retained for repeated runs within its owning execution.
pub struct Local<'a, T>(pub(crate) &'a mut T);
impl<T> Deref for Local<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0
    }
}
impl<T> DerefMut for Local<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

trait InsertResource: Send {
    fn apply(self: Box<Self>, resources: &mut Resources);
}
struct Insert<T>(T);
impl<T: Send + Sync + 'static> InsertResource for Insert<T> {
    fn apply(self: Box<Self>, resources: &mut Resources) {
        resources.insert(self.0);
    }
}

/// Queue resource insertions without invalidating active typed parameter borrows.
pub struct ContextWriter<'a>(pub(crate) &'a Resources);
impl ContextWriter<'_> {
    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, value: T) {
        self.0.pending.borrow_mut().push(Box::new(Insert(value)));
    }
}
impl Resources {
    pub(crate) fn apply_pending(&mut self) {
        let pending = self.pending.get_mut().drain(..).collect::<Vec<_>>();
        for insertion in pending {
            insertion.apply(self);
        }
    }
    pub(crate) fn discard_pending(&mut self) {
        self.pending.get_mut().clear();
    }
}

/// Immutable shared handles, snapshotted when an Execution is created.
#[derive(Clone, Default)]
pub(crate) struct SharedResources {
    values: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}
impl SharedResources {
    pub(crate) fn insert<T: Send + Sync + 'static>(&mut self, value: Arc<T>) {
        self.values.insert(TypeId::of::<T>(), value);
    }
}

/// An owned Arc handle to a shared service or immutable configuration.
pub struct Shared<T> {
    value: Arc<T>,
}
impl<T> Shared<T> {
    pub fn into_arc(self) -> Arc<T> {
        self.value
    }
}
impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
        }
    }
}
impl<T> Deref for Shared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
