mod location;
mod option;
mod result;

use core::error::Error;

pub use location::*;
pub use option::*;
pub use railgun_error_derive::Error;
pub use result::*;

#[doc(hidden)]
pub mod prelude {
    //! The prelude is for use in doctests to prevent ballooning comments.
    //!
    //! This is not encouraged for use in real projects, hence the
    //! `doc(hidden)` attribute.
    pub use crate::*;
}

pub trait StackedError: core::error::Error {
    fn display(&self, layer: usize, buf: &mut Vec<String>);

    fn next(&self) -> Option<&dyn StackedError>;

    fn last(&self) -> &dyn StackedError
    where
        Self: Sized,
    {
        let Some(mut result) = self.next() else {
            return self;
        };

        while let Some(err) = result.next() {
            result = err;
        }

        result
    }
}

/// Combines an underlying error with additional information
/// about the error.
///
/// It is expected that most users of SNAFU will not directly interact
/// with this trait.
pub trait IntoError<E>
where
    E: Error,
{
    /// The underlying error.
    type Source;

    /// Combine the information to produce the error.
    fn into_error(self, source: Self::Source) -> E;
}

/// Takes a string message and builds the corresponding error.
///
/// It is expected that most users of SNAFU will not directly interact
/// with this trait.
pub trait FromString {
    /// Create a brand new error from the given string
    fn without_source(message: String) -> Self;
}

/// Ensure that a condition is true. If it is not, return with an error.
///
/// ## Examples
///
/// ```rust
/// use railgun_error::prelude::*;
///
/// #[derive(Error)]
/// enum Error {
///     InvalidUser { user_id: i32 },
/// }
///
/// fn example(user_id: i32) -> Result<(), Error> {
///     ensure!(user_id > 0, InvalidUserContext { user_id });
///     let user_id = user_id as u32;
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($predicate:expr, $context_selector:expr $(,)?) => {
        if !$predicate {
            return $context_selector
                .fail()
                .map_err(::core::convert::Into::into);
        }
    };
}

#[macro_export]
macro_rules! ensure_whatever {
    ($predicate:expr, $fmt:literal$(, $($arg:expr),* $(,)?)?) => {
        if !$predicate {
            $crate::whatever!($fmt$(, $($arg),*)*)
        }
    }
}

#[macro_export]
macro_rules! whatever {
    ($fmt:literal$(, $($arg:expr),* $(,)?)?) => {
        return core::result::Result::Err(
            { format!($fmt$(, $($arg),*)*) }.into()
        )
    };
}
