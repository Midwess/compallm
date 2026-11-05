use crate::{
    access::{Access, validate_parameters},
    processor::{IntoProcessor, Processor, ProcessorFuture, ProcessorOutput},
    resources::{ContextWriter, Owned, Resources, Shared},
};
use std::{any::TypeId, future::Future, io, marker::PhantomData};

pub trait AsyncParam: Send + 'static {
    fn access() -> Access;
    fn fetch(resources: &Resources) -> io::Result<Self>
    where
        Self: Sized;
}
impl<T: Clone + Send + Sync + 'static> AsyncParam for Owned<T> {
    fn access() -> Access {
        Access::default().read::<T>()
    }
    fn fetch(resources: &Resources) -> io::Result<Self> {
        Ok(Self(T::clone(&*resources.get::<T>()?)))
    }
}
impl<T: Send + Sync + 'static> AsyncParam for Shared<T> {
    fn access() -> Access {
        Access::default()
    }
    fn fetch(resources: &Resources) -> io::Result<Self> {
        resources.get_shared()
    }
}
impl AsyncParam for ContextWriter {
    fn access() -> Access {
        Access::exclusive()
    }
    fn fetch(resources: &Resources) -> io::Result<Self> {
        Ok(resources.writer())
    }
}

#[derive(Clone)]
pub struct AsyncProcessor<F>(F);
pub fn async_processor<F>(function: F) -> AsyncProcessor<F> {
    AsyncProcessor(function)
}
pub struct AsyncMarker<M>(PhantomData<M>);
pub struct AsyncFunction<F, M> {
    function: F,
    marker: PhantomData<fn() -> M>,
}

impl<F, Fut, O> IntoProcessor<AsyncMarker<fn() -> Fut>> for AsyncProcessor<F>
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = O> + Send + 'static,
    O: ProcessorOutput + 'static,
{
    type Processor = AsyncFunction<F, fn() -> Fut>;
    fn into_processor(self) -> Self::Processor {
        AsyncFunction {
            function: self.0,
            marker: PhantomData,
        }
    }
}
impl<F, Fut, O> Processor for AsyncFunction<F, fn() -> Fut>
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = O> + Send + 'static,
    O: ProcessorOutput + 'static,
{
    fn processor_type(&self) -> TypeId {
        TypeId::of::<F>()
    }
    fn access(&self) -> Access {
        Access::default()
    }
    fn is_async(&self) -> bool {
        true
    }
    fn run(&mut self, _: &Resources) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "async processor requires run_stage_async",
        ))
    }
    fn run_async<'a>(&'a mut self, _: &'a Resources) -> ProcessorFuture<'a> {
        let future = (self.function)();
        Box::pin(async move { future.await.into_result() })
    }
}
macro_rules! async_function {
    ($($param:ident),+) => {
        impl<F, Fut, O, $($param: AsyncParam),+> IntoProcessor<AsyncMarker<fn($($param),+) -> Fut>> for AsyncProcessor<F>
        where F: FnMut($($param),+) -> Fut + Send + 'static, Fut: Future<Output = O> + Send + 'static, O: ProcessorOutput + 'static,
        {
            type Processor = AsyncFunction<F, fn($($param),+) -> Fut>;
            fn into_processor(self) -> Self::Processor { AsyncFunction { function: self.0, marker: PhantomData } }
        }
        impl<F, Fut, O, $($param: AsyncParam),+> Processor for AsyncFunction<F, fn($($param),+) -> Fut>
        where F: FnMut($($param),+) -> Fut + Send + 'static, Fut: Future<Output = O> + Send + 'static, O: ProcessorOutput + 'static,
        {
            fn validate(&self) -> io::Result<()> { validate_parameters(&[$($param::access()),+]) }
            fn processor_type(&self) -> TypeId { TypeId::of::<F>() }
            fn access(&self) -> Access { let mut access = Access::default(); $(access.merge($param::access());)+ access }
            fn is_async(&self) -> bool { true }
            fn run(&mut self, _: &Resources) -> io::Result<()> { Err(io::Error::new(io::ErrorKind::Unsupported, "async processor requires run_stage_async")) }
            fn run_async<'a>(&'a mut self, resources: &'a Resources) -> ProcessorFuture<'a> {
                Box::pin(async move { (self.function)($($param::fetch(resources)?),+).await.into_result() })
            }
        }
    };
}
async_function!(P0);
async_function!(P0, P1);
async_function!(P0, P1, P2);
async_function!(P0, P1, P2, P3);
