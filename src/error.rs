use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("unexpected end of file while parsing header")]
    NoHeader,

    #[error("file contains Xbox 360 File Container metadata")]
    XboxContainer,

    #[error("invalid file ID: expected 'TES4SAVEGAME', got: '{0:?}'")]
    BadFileHeader([u8; 12]),

    #[error("unexpected end of file")]
    UnexpectedEOF,
}

pub type ParseResult<T> = core::result::Result<T, ParseError>;
