use std::marker::PhantomData;

use crate::{
    procedure::{Procedure, Unresolved},
    router::Router,
};

pub struct RpcContext<TContext = ()>
where
    TContext: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<TContext>,
}

impl Default for RpcContext {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<TContext> RpcContext<TContext>
where
    TContext: Clone + Send + Sync + 'static,
{
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn router(&self) -> Router<TContext> {
        Router::new()
    }

    pub fn procedure(&self) -> Procedure<Unresolved> {
        Procedure::<Unresolved>::default()
    }
}
