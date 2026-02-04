use core::error::Error;

use crate::IntoError;

/// Additions to [`Result`][].
pub trait ResultExt<T, E>: Sized {
    /// Extend a [`Result`]'s error with additional context-sensitive
    /// information.
    ///
    /// [`Result`]: std::result::Result
    ///
    /// ```rust
    /// use railgun_error::prelude::*;
    ///
    /// #[derive(Error)]
    /// enum AuthError {
    ///     Authenticating {
    ///         user_name: String,
    ///         user_id: i32,
    ///         source: Box<dyn core::error::Error>,
    ///     },
    /// }
    ///
    /// fn example() -> Result<(), Error> {
    ///     another_function().context(AuthenticatingContext {
    ///         user_name: "admin",
    ///         user_id: 42,
    ///     })?;
    ///     Ok(())
    /// }
    ///
    /// fn another_function() -> Result<i32, Box<dyn core::error::Error>> {
    ///     Ok(42)
    /// }
    /// ```
    ///
    /// Note that the context selector will call [`Into::into`][] on each field,
    /// so the types are not required to exactly match.
    fn context<C, E2>(self, context: C) -> Result<T, E2>
    where
        C: IntoError<E2, Source = E>,
        E2: Error;

    /// Extend a [`Result`][]'s error with lazily-generated context-sensitive
    /// information.
    ///
    /// [`Result`]: std::result::Result
    ///
    /// ```rust
    /// use snafu::prelude::*;
    ///
    /// #[derive(Debug, Snafu)]
    /// enum Error {
    ///     Authenticating {
    ///         user_name: String,
    ///         user_id: i32,
    ///         source: ApiError,
    ///     },
    /// }
    ///
    /// fn example() -> Result<(), Error> {
    ///     another_function().with_context(|_| AuthenticatingSnafu {
    ///         user_name: "admin".to_string(),
    ///         user_id: 42,
    ///     })?;
    ///     Ok(())
    /// }
    ///
    /// # type ApiError = std::io::Error;
    /// fn another_function() -> Result<i32, ApiError> {
    ///     /* ... */
    /// # Ok(42)
    /// }
    /// ```
    ///
    /// Note that this *may not* be needed in many cases because the context
    /// selector will call [`Into::into`][] on each field.
    fn with_context<F, C, E2>(self, context: F) -> Result<T, E2>
    where
        F: FnOnce(&mut E) -> C,
        C: IntoError<E2, Source = E>,
        E2: Error;

    /// Convert a [`Result`]'s error into a boxed trait object
    /// compatible with multiple threads.
    ///
    /// This is useful when you have errors of multiple types that you
    /// wish to treat as one type. This may occur when dealing with
    /// errors in a generic context, such as when the error is a
    /// trait's associated type.
    ///
    /// In cases like this, you cannot name the original error type
    /// without making the outer error type generic as well. Using an
    /// error trait object offers an alternate solution.
    ///
    /// ```rust
    /// use railgun_error::*;
    /// use railgun_error_derive::*;
    ///
    /// fn convert_value_into_u8<V>(v: V) -> Result<u8, ConversionFailedError>
    /// where
    ///     V: TryInto<u8>,
    ///     V::Error: snafu::Error + Send + Sync + 'static,
    /// {
    ///     v.try_into().boxed().context(ConversionFailedSnafu)
    /// }
    ///
    /// #[derive(Debug, Error)]
    /// struct ConversionFailedError {
    ///     source: Box<dyn snafu::Error + Send + Sync + 'static>,
    /// }
    /// ```
    ///
    /// ## Avoiding misapplication
    ///
    /// We recommended **against** using this to create fewer error
    /// variants which in turn would group unrelated errors. While
    /// convenient for the programmer, doing so usually makes lower
    /// quality error messages for the user.
    ///
    /// ```rust
    /// use std::fs;
    ///
    /// use railgun_error::*;
    /// use railgun_error_derive::*;
    ///
    /// fn do_not_do_this() -> Result<i32, UselessError> {
    ///     let content = fs::read_to_string("/path/to/config/file")
    ///         .boxed()
    ///         .context(UselessSnafu)?;
    ///
    ///     content.parse().boxed().context(UselessSnafu)
    /// }
    ///
    /// #[derive(Debug, Error)]
    /// struct UselessError {
    ///     source: Box<dyn snafu::Error + Send + Sync + 'static>,
    /// }
    /// ```
    fn boxed<'a>(self) -> Result<T, Box<dyn Error + Send + Sync + 'a>>
    where
        E: Error + Send + Sync + 'a;

    /// Convert a [`Result`]'s error into a boxed trait object.
    ///
    /// This is useful when you have errors of multiple types that you
    /// wish to treat as one type. This may occur when dealing with
    /// errors in a generic context, such as when the error is a
    /// trait's associated type.
    ///
    /// In cases like this, you cannot name the original error type
    /// without making the outer error type generic as well. Using an
    /// error trait object offers an alternate solution.
    ///
    /// ```rust
    /// # use std::convert::TryInto;
    ///
    /// use railgun_error::*;
    /// use railgun_error_derive::*;
    ///
    /// fn convert_value_into_u8<V>(v: V) -> Result<u8, ConversionFailedError>
    /// where
    ///     V: TryInto<u8>,
    ///     V::Error: snafu::Error + 'static,
    /// {
    ///     v.try_into().boxed_local().context(ConversionFailedSnafu)
    /// }
    ///
    /// #[derive(Debug, Debug)]
    /// struct ConversionFailedError {
    ///     source: Box<dyn snafu::Error + 'static>,
    /// }
    /// ```
    ///
    /// ## Avoiding misapplication
    ///
    /// We recommended **against** using this to create fewer error
    /// variants which in turn would group unrelated errors. While
    /// convenient for the programmer, doing so usually makes lower
    /// quality error messages for the user.
    ///
    /// ```rust
    /// use std::fs;
    ///
    /// use railgun_error::*;
    /// use railgun_error_derive::*;
    ///
    /// fn do_not_do_this() -> Result<i32, UselessError> {
    ///     let content = fs::read_to_string("/path/to/config/file")
    ///         .boxed_local()
    ///         .context(UselessSnafu)?;
    ///
    ///     content.parse().boxed_local().context(UselessSnafu)
    /// }
    ///
    /// #[derive(Debug, Error)]
    /// struct UselessError {
    ///     source: Box<dyn snafu::Error + 'static>,
    /// }
    /// ```
    fn boxed_local<'a>(self) -> Result<T, Box<dyn Error + 'a>>
    where
        E: Error + 'a;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn context<C, E2>(self, context: C) -> Result<T, E2>
    where
        C: IntoError<E2, Source = E>,
        E2: Error,
    {
        match self {
            Ok(v) => Ok(v),
            Err(error) => Err(context.into_error(error)),
        }
    }

    #[track_caller]
    fn with_context<F, C, E2>(self, context: F) -> Result<T, E2>
    where
        F: FnOnce(&mut E) -> C,
        C: IntoError<E2, Source = E>,
        E2: Error,
    {
        match self {
            Ok(v) => Ok(v),
            Err(mut error) => {
                let context = context(&mut error);
                Err(context.into_error(error))
            },
        }
    }

    fn boxed<'a>(self) -> Result<T, Box<dyn Error + Send + Sync + 'a>>
    where
        E: Error + Send + Sync + 'a,
    {
        self.map_err(core::convert::Into::into)
    }

    fn boxed_local<'a>(self) -> Result<T, Box<dyn Error + 'a>>
    where
        E: Error + 'a,
    {
        self.map_err(core::convert::Into::into)
    }
}
