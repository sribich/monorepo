pub mod axum;
pub mod export;
pub mod procedure;
pub mod router;
pub mod rpc;

pub use rpc::RpcContext;
use serde::{Deserialize, Serialize};
pub use typegen;
use typegen::Typegen;

pub trait RpcHandler<Req, Res, Rest, S> {}

pub struct HandlerIO {}

#[derive(Typegen, Serialize, Deserialize)]
pub struct Empty {}

const _: () = {
    use std::future::Future;

    use typegen::Type;

    macro_rules! impl_handler_type {
        (
            [$($extractor:ident),*], $ty:ident
        ) => {
            impl<F, Fut, S, M, Res, $($extractor,)* $ty> RpcHandler<$ty, Res, (M, $($extractor,)* $ty,), S> for F
            where
                F: FnOnce($($extractor,)* $ty,) -> Fut,
                Fut: Future<Output = Res>,
                Res: Type /*+ FromRequest<S, M>*/,
                S: Send + Sync + 'static,
                $ty: Type,
            {}
        }
    }

    // impl<F, Fut, S, Res> RpcHandler<Empty, Res, ((),), S> for F
    // where
    //     F: FnOnce() -> Fut,
    //     Fut: Future<Output = Res>,
    //     Res: Type, /* + FromRequest<S> */
    //     S: Send + Sync + 'static,
    // {
    // }

    impl_handler_type!([], T1);
    impl_handler_type!([T1], T2);
    impl_handler_type!([T1, T2], T3);
    impl_handler_type!([T1, T2, T3], T4);
    impl_handler_type!([T1, T2, T3, T4], T5);
    impl_handler_type!([T1, T2, T3, T4, T5], T6);
    impl_handler_type!([T1, T2, T3, T4, T5, T6], T7);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7], T8);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
    impl_handler_type!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
    impl_handler_type!(
        [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
        T14
    );
    impl_handler_type!(
        [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
        T15
    );
    impl_handler_type!(
        [
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
        ],
        T16
    );
};
