pub mod database;
pub mod dirs;
pub mod http;

pub trait Procedure: Send + Sync {
    type Err = core::convert::Infallible;
    type Req;
    type Res;

    #[must_use]
    fn run(
        &self,
        data: Self::Req,
    ) -> impl std::future::Future<Output = Result<Self::Res, Self::Err>>;
}
