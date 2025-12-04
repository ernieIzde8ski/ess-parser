use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("Unexpected end of file while parsing header")]
    NoHeader,

    #[error("File contains Xbox 360 File Container metadata")]
    XboxContainer,

    #[error("Invalid file ID: expected 'TES4SAVEGAME', got: '{0:?}'")]
    BadFileID([u8; 12]),

    #[error("Unexpected end of file")]
    UnexpectedEOF,
}

pub type ParseResult<T> = core::result::Result<T, ParseError>;
