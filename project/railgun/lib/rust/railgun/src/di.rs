use std::sync::Arc;

use async_trait::async_trait;
pub use railgun_di::*;
use rpc::procedure::Procedure;
use rpc::procedure::Unresolved;
use rpc::router::Router;

#[macro_export]
macro_rules! module {
    ($name:ident, $state:ident) => {
        pub struct $name {}

        impl $name {
            pub fn new_module() -> Box<dyn $crate::di::Module<State = $state>> {
                Box::new(Self {})
            }
        }

        impl $crate::di::Module for $name {
            type State = $state;

            fn as_routes(&self) -> Option<&dyn $crate::di::Routes<Self::State>> {
                ::std::any::try_as_dyn::<Self, dyn $crate::di::Routes<Self::State>>(self)
            }

            fn as_container(&self) -> Option<&dyn $crate::di::Container> {
                ::std::any::try_as_dyn::<Self, dyn $crate::di::Container>(self)
            }
        }
    };
}

pub trait Module {
    type State;
    // <T: 'static + Clone + Send + Sync>
    fn as_routes(&self) -> Option<&dyn Routes<Self::State>>;
    fn as_container(&self) -> Option<&dyn Container>;
}

pub trait Routes<T: Clone + Send + Sync> {
    fn routes(
        &self,
        router: Router<T>,
        procedure: Procedure<Unresolved>,
        state: Arc<T>,
    ) -> Router<T>;
}

pub trait Container {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError>;
}
