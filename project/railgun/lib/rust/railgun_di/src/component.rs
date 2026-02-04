use core::any::{Any, TypeId, type_name};
use std::sync::Arc;

use crate::{Injector, InjectorBuilder, error::InjectionError};

pub trait Builder: Send + Sync {
    /// Returns the [`TypeId`] of the underlying concrete type.
    fn type_id(&self) -> TypeId;

    /// Returns the name of the underlying concrete type.
    fn type_name(&self) -> &'static str;

    /// Returns an instance of the supplied type.
    fn as_any(&self, injector: &Injector) -> Result<Arc<dyn Any + Send + Sync>, InjectionError>;
}

pub trait TypedBuilder<T>: Builder
where
    T: Send + Sync + ?Sized,
{
    /// Called to get an instance of the component, respecting the lifetime
    /// defined by the scope.
    fn get(&self, injector: &Injector) -> Result<Arc<T>, InjectionError>;

    // Called during registration to automatically bind this builder to all
    // interfaces this component implements
    fn bind_interfaces(&self, injector: &mut InjectorBuilder);
}

pub trait Component {
    type Impl: Send + Sync;
    type Builder: TypedBuilder<Self::Impl>;

    fn builder() -> Self::Builder;
}

impl<TConcrete> Builder for Arc<TConcrete>
where
    TConcrete: 'static + Send + Sync,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<TConcrete>()
    }

    fn type_name(&self) -> &'static str {
        type_name::<TConcrete>()
    }

    fn as_any(&self, _injector: &Injector) -> Result<Arc<dyn Any + Send + Sync>, InjectionError> {
        let this = Arc::clone(self);

        Ok(this)
    }
}

impl<TConcrete> TypedBuilder<TConcrete> for Arc<TConcrete>
where
    TConcrete: 'static + Send + Sync,
{
    fn get(&self, _injector: &Injector) -> Result<Arc<TConcrete>, InjectionError> {
        let this = Arc::clone(self);

        Ok(this)
    }

    // There are no interfaces to bind since we can trivially clone the type.
    fn bind_interfaces(&self, _injector: &mut InjectorBuilder) {}
}
