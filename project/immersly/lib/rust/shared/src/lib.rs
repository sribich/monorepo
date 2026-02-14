#![feature(associated_type_defaults, macro_metavar_expr_concat)]

use async_trait::async_trait;
pub mod app;
pub mod domain;
pub mod infra;

#[doc(hidden)]
pub mod __internal__ {
    pub use typegen;
}

#[macro_export]
macro_rules! muid_newtype {
    ($ident:ident) => {
        /// TODO: This should not be Typegen/Serialize/Deserialize.
        #[derive(
            Clone,
            Debug,
            $crate::__internal__::typegen::Typegen,
            serde::Deserialize,
            serde::Serialize,
        )]
        pub struct $ident($crate::domain::value::muid::Muid);

        #[automatically_derived]
        impl $ident {
            pub fn new_now() -> Self {
                Self($crate::domain::value::muid::Muid::new_now())
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            pub fn to_vec(&self) -> Vec<u8> {
                self.0.as_bytes().to_vec()
            }

            pub fn try_from_slice(
                data: &[u8],
            ) -> Result<Self, $crate::domain::value::muid::MuidParseError> {
                Ok(Self($crate::domain::value::muid::Muid::try_from_slice(
                    data,
                )?))
            }

            pub fn try_from_str<S: AsRef<str>>(
                data: S,
            ) -> Result<Self, $crate::domain::value::muid::MuidParseError> {
                Ok(Self($crate::domain::value::muid::Muid::try_from_str(
                    data.as_ref(),
                )?))
            }

            pub fn from_slice_unchecked(slice: &[u8]) -> Self {
                Self($crate::domain::value::muid::Muid::from_slice_unchecked(
                    slice,
                ))
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        // TODO: I don't think we want this
        impl From<$crate::domain::value::muid::Muid> for $ident {
            fn from(value: $crate::domain::value::muid::Muid) -> Self {
                Self(value)
            }
        }

        // TODO: I don't think we want this
        impl From<$ident> for $crate::domain::value::muid::Muid {
            fn from(value: $ident) -> Self {
                value.0
            }
        }
    };
}

#[macro_export]
macro_rules! muid_new_newtype {
    ($ident:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $ident($crate::domain::value::muid::Muid);

        #[automatically_derived]
        impl $ident {
            pub fn new_now() -> Self {
                Self($crate::domain::value::muid::Muid::new_now())
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            pub fn to_vec(&self) -> Vec<u8> {
                self.0.as_bytes().to_vec()
            }

            pub fn try_from_slice(
                data: &[u8],
            ) -> Result<Self, $crate::domain::value::muid::MuidParseError> {
                Ok(Self($crate::domain::value::muid::Muid::try_from_slice(
                    data,
                )?))
            }

            pub fn try_from_str<S: AsRef<str>>(
                data: S,
            ) -> Result<Self, $crate::domain::value::muid::MuidParseError> {
                Ok(Self($crate::domain::value::muid::Muid::try_from_str(
                    data.as_ref(),
                )?))
            }

            pub fn from_slice_unchecked(slice: &[u8]) -> Self {
                Self($crate::domain::value::muid::Muid::from_slice_unchecked(
                    slice,
                ))
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        // TODO: I don't think we want this
        impl From<$crate::domain::value::muid::Muid> for $ident {
            fn from(value: $crate::domain::value::muid::Muid) -> Self {
                Self(value)
            }
        }

        // TODO: I don't think we want this
        impl From<$ident> for $crate::domain::value::muid::Muid {
            fn from(value: $ident) -> Self {
                value.0
            }
        }

        impl From<&$ident> for $crate::domain::value::muid::Muid {
            fn from(value: &$ident) -> Self {
                value.0.clone()
            }
        }
    };
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
macro_rules! entity_data_fns {
    ($name:ident) => {
        entity_data_fns!($name, ${concat($name, Data)});
    };
    ($name:ident, $data_name:ident) => {
        impl $name {
            pub fn from_data(data: $data_name) -> Self {
                Self(data)
            }

            pub fn as_inner(&self) -> &$data_name {
                &self.0
            }

            pub fn as_inner_mut(&mut self) -> &mut $data_name {
                &mut self.0
            }

            pub fn to_inner(self) -> $data_name {
                self.0
            }
        }

        impl From<$data_name> for $name {
            fn from(value: $data_name) -> $name {
                $name(value)
            }
        }
    };
}

//==============================================================================
// Hooks
//==============================================================================
#[async_trait]
pub trait OnStartup: Send + Sync {
    async fn run(&self) -> core::result::Result<(), Box<dyn core::error::Error>>;
}

#[async_trait]
pub trait OnShutdown: Send + Sync {
    async fn run(&self) -> core::result::Result<(), Box<dyn core::error::Error>>;
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
