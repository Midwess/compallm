pub use crate::context_writer::ContextWriter;
use crate::context_writer::{InsertResource, PendingInsertions};
use hecs::{Entity, World};
pub use hecs::{Ref as Res, RefMut as ResMut};
use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    io,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// One hecs World and resource entity per execution. Worker handles live only during a wave.
pub struct Resources {
    world: Arc<World>,
    entity: Entity,
    pending: PendingInsertions,
    shared: Arc<SharedResources>,
}
impl Default for Resources {
    fn default() -> Self {
        let mut world = World::new();
        let entity = world.spawn(());
        Self {
            world: Arc::new(world),
            entity,
            pending: PendingInsertions::default(),
            shared: Arc::default(),
        }
    }
}
impl Resources {
    pub(crate) fn with_shared(shared: Arc<SharedResources>) -> Self {
        Self {
            shared,
            ..Self::default()
        }
    }
    pub(crate) fn invocation(&self) -> Self {
        Self {
            world: Arc::clone(&self.world),
            entity: self.entity,
            pending: PendingInsertions::default(),
            shared: Arc::clone(&self.shared),
        }
    }
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        let world = Arc::get_mut(&mut self.world)
            .expect("cannot insert while resource workers are active; drop a cancelled execution");
        world
            .insert_one(self.entity, value)
            .expect("private resource entity");
    }
    #[inline]
    pub fn get<T: Send + Sync + 'static>(&self) -> io::Result<Res<'_, T>> {
        self.world
            .get::<&T>(self.entity)
            .map_err(|_| missing::<T>())
    }
    #[inline]
    pub fn get_mut<T: Send + Sync + 'static>(&self) -> io::Result<ResMut<'_, T>> {
        self.world
            .get::<&mut T>(self.entity)
            .map_err(|_| missing::<T>())
    }

    pub fn get_shared<T: Send + Sync + 'static>(&self) -> io::Result<Shared<T>> {
        let value = self
            .shared
            .values
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        Ok(Shared {
            value: Arc::clone(value).downcast().map_err(|_| missing::<T>())?,
        })
    }
    pub fn writer(&self) -> ContextWriter {
        self.pending.writer()
    }
    pub(crate) fn take_pending(&mut self) -> Vec<Box<dyn InsertResource>> {
        self.pending.take()
    }
    pub(crate) fn apply(&mut self, pending: Vec<Box<dyn InsertResource>>) {
        for insertion in pending {
            insertion.apply(self);
        }
    }
    pub(crate) fn apply_pending(&mut self) {
        let pending = self.take_pending();
        self.apply(pending);
    }
    pub(crate) fn discard_pending(&mut self) {
        self.take_pending();
    }
}
fn missing<T>() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        format!("missing resource {}", type_name::<T>()),
    )
}

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

/// An owned, cloned local resource value for async work without retained borrows.
pub struct Owned<T>(pub T);
impl<T> Owned<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Clone, Default)]
pub(crate) struct SharedResources {
    values: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}
impl SharedResources {
    pub(crate) fn insert<T: Send + Sync + 'static>(&mut self, value: Arc<T>) {
        self.values.insert(TypeId::of::<T>(), value);
    }
}
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
