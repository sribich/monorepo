use core::error::Error;

use crate::IntoError;

/// A temporary error type used when converting an [`Option`] into a
/// [`Result`].
///
/// [`Option`]: std::option::Option
/// [`Result`]: std::result::Result
pub struct NoneError;

/// Additions to [`Option`].
pub trait OptionExt<T>: Sized {
    /// Convert an [`Option`][] into a [`Result`][] with additional
    /// context-sensitive information.
    ///
    /// [Option]: std::option::Option
    /// [Result]: std::option::Result
    ///
    /// ```rust
    /// use snafu::prelude::*;
    ///
    /// #[derive(Debug, Snafu)]
    /// enum Error {
    ///     UserLookup { user_id: i32 },
    /// }
    ///
    /// fn example(user_id: i32) -> Result<(), Error> {
    ///     let name = username(user_id).context(UserLookupSnafu { user_id })?;
    ///     println!("Username was {name}");
    ///     Ok(())
    /// }
    ///
    /// fn username(user_id: i32) -> Option<String> {
    ///     /* ... */
    /// # None
    /// }
    /// ```
    ///
    /// Note that the context selector will call [`Into::into`][] on each field,
    /// so the types are not required to exactly match.
    fn context<C, E>(self, context: C) -> Result<T, E>
    where
        C: IntoError<E, Source = NoneError>,
        E: Error;

    /// Convert an [`Option`] into a [`Result`] with
    /// lazily-generated context-sensitive information.
    ///
    /// [`Option`]: std::option::Option
    /// [`Result`]: std::result::Result
    ///
    /// ```
    /// #[derive(Debug, Error)]
    /// enum Error {
    ///     UserLookup {
    ///         user_id: i32,
    ///         previous_ids: Vec<i32>,
    ///     },
    /// }
    ///
    /// fn example(user_id: i32) -> Result<(), Error> {
    ///     let name = username(user_id).with_context(|| UserLookupSnafu {
    ///         user_id,
    ///         previous_ids: Vec::new(),
    ///     })?;
    ///     println!("Username was {name}");
    ///     Ok(())
    /// }
    ///
    /// fn username(user_id: i32) -> Option<String> {
    ///     /* ... */
    /// # None
    /// }
    /// ```
    ///
    /// Note that this *may not* be needed in many cases because the context
    /// selector will call [`Into::into`][] on each field.
    fn with_context<F, C, E>(self, context: F) -> Result<T, E>
    where
        F: FnOnce() -> C,
        C: IntoError<E, Source = NoneError>,
        E: Error;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn context<C, E>(self, context: C) -> Result<T, E>
    where
        C: IntoError<E, Source = NoneError>,
        E: Error,
    {
        #[expect(
            clippy::option_if_let_else,
            reason = "https://github.com/rust-lang/rust/issues/87417"
        )]
        match self {
            Some(v) => Ok(v),
            None => Err(context.into_error(NoneError)),
        }
    }

    #[track_caller]
    fn with_context<F, C, E>(self, context: F) -> Result<T, E>
    where
        F: FnOnce() -> C,
        C: IntoError<E, Source = NoneError>,
        E: Error,
    {
        #[expect(
            clippy::option_if_let_else,
            reason = "https://github.com/rust-lang/rust/issues/87417"
        )]
        match self {
            Some(v) => Ok(v),
            None => Err(context().into_error(NoneError)),
        }
    }
}
