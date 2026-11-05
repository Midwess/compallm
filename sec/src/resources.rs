use hecs::{Entity, World};
use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
    io,
    ops::{Deref, DerefMut},
    sync::{
        Arc, Mutex,
        atomic::{AtomicIsize, Ordering},
    },
};

#[derive(Default)]
struct ResourceStore {
    world: World,
    entities: HashMap<TypeId, ResourceSlot>,
}
struct ResourceSlot {
    entity: Entity,
    borrow: Arc<AtomicIsize>,
}
struct Lease {
    borrow: Arc<AtomicIsize>,
    write: bool,
}
impl Drop for Lease {
    fn drop(&mut self) {
        if self.write {
            self.borrow.store(0, Ordering::Release);
        } else {
            self.borrow.fetch_sub(1, Ordering::Release);
        }
    }
}
impl ResourceSlot {
    fn lease<T: 'static>(&self, write: bool) -> io::Result<Lease> {
        let acquired = if write {
            self.borrow
                .compare_exchange(0, -1, Ordering::AcqRel, Ordering::Acquire)
        } else {
            self.borrow
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                    if value < 0 {
                        None
                    } else {
                        value.checked_add(1)
                    }
                })
        };
        acquired.map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("conflicting resource access {}", type_name::<T>()),
            )
        })?;
        Ok(Lease {
            borrow: Arc::clone(&self.borrow),
            write,
        })
    }
}

type Pending = Arc<Mutex<Vec<Box<dyn InsertResource>>>>;

/// Isolated hecs resource storage. Workers share storage only while a stage wave runs.
#[derive(Default)]
pub struct Resources {
    store: Arc<ResourceStore>,
    pending: Pending,
    shared: Arc<SharedResources>,
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
            store: Arc::clone(&self.store),
            pending: Arc::default(),
            shared: Arc::clone(&self.shared),
        }
    }
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        let store = Arc::get_mut(&mut self.store)
            .expect("cannot insert while resource workers are active; drop a cancelled execution");
        if let Some(slot) = store.entities.get(&TypeId::of::<T>()) {
            let entity = slot.entity;
            store
                .world
                .insert_one(entity, value)
                .expect("private resource entity");
        } else {
            let entity = store.world.spawn((value,));
            store.entities.insert(
                TypeId::of::<T>(),
                ResourceSlot {
                    entity,
                    borrow: Arc::new(AtomicIsize::new(0)),
                },
            );
        }
    }
    pub fn get<T: Send + Sync + 'static>(&self) -> io::Result<Res<'_, T>> {
        let slot = self
            .store
            .entities
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        let lease = slot.lease::<T>(false)?;
        let entity = slot.entity;
        let value = self
            .store
            .world
            .get::<&T>(entity)
            .map_err(|_| missing::<T>())?;
        Ok(Res {
            value,
            _lease: lease,
        })
    }
    pub fn get_mut<T: Send + Sync + 'static>(&self) -> io::Result<ResMut<'_, T>> {
        let slot = self
            .store
            .entities
            .get(&TypeId::of::<T>())
            .ok_or_else(missing::<T>)?;
        let lease = slot.lease::<T>(true)?;
        let entity = slot.entity;
        let value = self
            .store
            .world
            .get::<&mut T>(entity)
            .map_err(|_| missing::<T>())?;
        Ok(ResMut {
            value,
            _lease: lease,
        })
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
        ContextWriter(Arc::clone(&self.pending))
    }
    pub(crate) fn take_pending(&mut self) -> Vec<Box<dyn InsertResource>> {
        std::mem::take(
            self.pending
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .as_mut(),
        )
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

pub struct Res<'a, T: Send + Sync + 'static> {
    value: hecs::Ref<'a, T>,
    _lease: Lease,
}
impl<T: Send + Sync + 'static> Deref for Res<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
pub struct ResMut<'a, T: Send + Sync + 'static> {
    value: hecs::RefMut<'a, T>,
    _lease: Lease,
}
impl<T: Send + Sync + 'static> Deref for ResMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T: Send + Sync + 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
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

pub(crate) trait InsertResource: Send {
    fn apply(self: Box<Self>, resources: &mut Resources);
}
struct Insert<T>(T);
impl<T: Send + Sync + 'static> InsertResource for Insert<T> {
    fn apply(self: Box<Self>, resources: &mut Resources) {
        resources.insert(self.0);
    }
}
/// Processor-invocation-local deferred insertions; safe to own across an await.
#[derive(Clone)]
pub struct ContextWriter(Pending);
impl ContextWriter {
    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, value: T) {
        self.0
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(Box::new(Insert(value)));
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
