use core::{
    any::{TypeId, type_name},
    marker::PhantomData,
};
use std::sync::Arc;

use crate::{
    InjectionError, Injector, error::CalledGetOnCollectionContext, typecast_builder::BuilderList,
};

pub trait Spec {
    type Out;

    fn get(injector: &Injector) -> Result<Self::Out, InjectionError>;

    fn validate(injector: &Injector) -> Result<(), InjectionError>;
}

pub struct OneOf<T>
where
    T: 'static + ?Sized + Send + Sync,
{
    _marker: PhantomData<T>,
}

impl<T> Spec for OneOf<T>
where
    T: 'static + ?Sized + Send + Sync,
{
    type Out = Arc<T>;

    #[track_caller]
    fn get(injector: &Injector) -> Result<Self::Out, InjectionError> {
        let builders = injector.0.builders_for::<T>()?;

        let builder = match builders {
            BuilderList::Single(typecast_builder) => typecast_builder,
            BuilderList::Collection(_) => {
                return CalledGetOnCollectionContext {
                    type_id: TypeId::of::<T>(),
                    type_name: type_name::<T>(),
                }
                .fail();
            },
        };

        builder.get(injector)
    }

    fn validate(_injector: &Injector) -> Result<(), InjectionError> {
        todo!()
    }
}

pub struct AllOf<T>
where
    T: 'static + ?Sized + Send + Sync,
{
    _marker: PhantomData<T>,
}

impl<T> Spec for AllOf<T>
where
    T: 'static + ?Sized + Send + Sync,
{
    type Out = Vec<Arc<T>>;

    fn get(injector: &Injector) -> Result<Self::Out, InjectionError> {
        let builders = injector.0.builders_for::<T>()?;

        match builders {
            BuilderList::Single(typecast_builder) => Ok(vec![typecast_builder.get(injector)?]),
            BuilderList::Collection(items) => items
                .map(|it| it.get(injector))
                .collect::<Result<Vec<_>, InjectionError>>(),
        }
    }

    fn validate(_injector: &Injector) -> Result<(), InjectionError> {
        todo!()
    }
}
