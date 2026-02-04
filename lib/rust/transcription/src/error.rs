use railgun::error::Error;
use railgun::error::Location;
// use whisper_rs::WhisperError;

pub type Result<T> = core::result::Result<T, TranscriptionError>;

#[derive(Error)]
pub enum TranscriptionError {
    #[error(display("extension '{extension}' is not supported"))]
    InvalidAudioExtension {
        extension: String,
        location: Location,
    },
    // #[error(display("an error has occurred in whisper"))]
    // WhisperError {
    //     error: WhisperError,
    //     location: Location,
    // },
}
