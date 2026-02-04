// pub fn query(mut self, procedure: &'static str) -> Self {
//     self
// }
//
// pub fn mutation(mut self, procedure: &'static str) -> Self {
//     self
// }
//
// pub fn subscription(mut self, procedure: &'static str) -> Self {
//     self
// }

use std::marker::PhantomData;

use axum::handler::Handler;

mod sealed {
    use axum::handler::Handler;

    pub trait Sealed {}

    impl Sealed for super::Unresolved {}
    impl<H, T, S> Sealed for super::Resolved<H, T, S>
    where
        H: Handler<T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
    }
}

pub trait ProcedureState: sealed::Sealed {}

#[derive(Clone)]
pub struct Unresolved;
impl ProcedureState for Unresolved {}

pub struct Resolved<H, T, S = ()>(pub (H, PhantomData<(T, S)>))
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static;

impl<H, T, S> ProcedureState for Resolved<H, T, S>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
}

#[derive(Clone)]
pub struct Procedure<State>
where
    State: ProcedureState,
{
    pub state: State,
    pub kind: ProcedureKind,
}

impl Default for Procedure<Unresolved> {
    fn default() -> Self {
        Self {
            state: Unresolved {},
            kind: ProcedureKind::Query,
        }
    }
}

#[derive(Clone)]
pub enum ProcedureKind {
    Query,
    Mutation,
    Subscription,
}

impl Procedure<Unresolved> {
    pub fn query<H, T, S /* , Req, Res */>(&self, handler: H) -> Procedure<Resolved<H, T, S>>
    where
        H: Handler<T, S>, // + RpcHandler<Req, Res, T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
        // Req: Type,
        // Res: Type,
    {
        Procedure::new(handler, ProcedureKind::Query)
    }

    pub fn mutation<H, T, S>(&self, handler: H) -> Procedure<Resolved<H, T, S>>
    where
        H: Handler<T, S>, //+ RpcHandler<Req, Res, T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
        Procedure::new(handler, ProcedureKind::Mutation)
    }

    pub fn subscription<H, T, S>(&self, handler: H) -> Procedure<Resolved<H, T, S>>
    where
        H: Handler<T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
        Procedure::new(handler, ProcedureKind::Subscription)
    }
}

impl<H, T, S> Procedure<Resolved<H, T, S>>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    fn new(handler: H, kind: ProcedureKind) -> Self {
        Self {
            state: Resolved((handler, PhantomData)),
            kind,
        }
    }
}
