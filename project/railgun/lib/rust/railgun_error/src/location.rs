/// The source code location where the error was reported.
///
/// This will be implcitly added to all error structs/variants unless
/// `#[error(explicit)]` is defined on the struct or variant. If `explicit`
/// is defined, location information must be added manually:
///
/// ```rust
/// use railgun_error::prelude::*;
///
/// #[derive(Error)]
/// enum MathError {
///     #[error(explicit)]
///     DivByZero { numerator: f32, location: Location },
/// }
///
/// fn test(numerator: f32, denominator: f32) -> Result<f32, MathError> {
///     ensure!(denominator != 0_f32, DivByZeroContext { numerator })
/// }
/// ```
///
/// The name of `location` can be changed by setting the `location` argument:
///
/// ```rust
/// use railgun_error::prelude::*;
///
/// #[derive(Error)]
/// enum DeliveryError {
///     #[error(location = "source_loc")]
///     MailboxDoesNotExist { location: &'static str },
/// }
///
/// fn deliver() -> Result<(), DeliveryError> {
///     ensure!(
///         location_exists(),
///         MailboxDoesNotExistContext { location: "..." }
///     );
///     Ok(())
/// }
///
/// fn location_exists() -> bool {
///     false
/// }
/// ```
pub struct Location {
    /// The file where the error was created.
    pub file: &'static str,
    /// The line where the error was reported.
    pub line: u32,
    /// The column where the error was reported.
    pub column: u32,
}

impl Default for Location {
    #[track_caller]
    fn default() -> Self {
        let location = core::panic::Location::caller();

        Self {
            file: location.file(),
            line: location.line(),
            column: location.column(),
        }
    }
}

impl core::fmt::Debug for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Location")
            .field("file", &self.file)
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.column,
        )
    }
}
