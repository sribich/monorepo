use async_trait::async_trait;

pub mod configuration;
pub mod dirs;
pub mod fs;
pub mod hooks;
pub mod http;

pub trait Procedure: Send + Sync {
    type Err = core::convert::Infallible;
    type Req;
    type Res;

    #[must_use]
    fn run(
        &self,
        data: Self::Req,
    ) -> impl std::future::Future<Output = core::result::Result<Self::Res, Self::Err>>;
}

pub trait UseCase: Send + Sync {
    type Err = core::convert::Infallible;
    type Req;
    type Res;

    #[must_use]
    fn run(
        &self,
        data: Self::Req,
    ) -> impl std::future::Future<Output = core::result::Result<Self::Res, Self::Err>>;
}

#[macro_export]
macro_rules! handler_aliases {
    ($ty:ident) => {
        type ProcedureFn = $ty;
        type ProcedureError = <ProcedureFn as Procedure>::Err;
        type ProcedureRequest = <ProcedureFn as Procedure>::Req;
        type ProcedureResponse = <ProcedureFn as Procedure>::Res;
    };
}

#[macro_export]
macro_rules! impl_repository {
    (
        $name:ident,
        $impl:ident,
        $($arg:ident: $path:path),*
        $(,)?
    ) => {
        #[allow(dead_code)]
        #[derive(railgun_di::Component)]
        #[component(implements(dyn ${concat($impl, Repository)}))]
        pub struct ${ concat($name, Repository) }
        {
            reader: std::sync::Arc<${concat($name, Reader)}>,
            writer: std::sync::Arc<${concat($name, Writer)}>,
        }

        #[allow(dead_code)]
        #[derive(railgun_di::Component)]
        #[component(implements(dyn ${concat($impl, Reader)}))]
        pub struct ${ concat($name, Reader) } {
            $($arg: $path,)*
        }

        #[allow(dead_code)]
        #[derive(railgun_di::Component)]
        #[component(implements(dyn ${concat($impl, Writer)}))]
        pub struct ${ concat($name, Writer) } {
            $($arg: $path,)*
        }

        impl ${concat($impl, Repository)} for ${ concat($name, Repository) } {
            fn reader(&self) -> std::sync::Arc<dyn ${concat($impl, Reader)}> {
                self.reader.clone()
            }

            fn writer(&self) -> std::sync::Arc<dyn ${concat($impl, Writer)}> {
                self.writer.clone()
            }
        }
    }
}

pub trait IntoVec<T> {
    fn into_vec(self) -> Vec<T>;
}

impl<T, F> IntoVec<T> for Vec<F>
where
    T: From<F>,
{
    fn into_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}
