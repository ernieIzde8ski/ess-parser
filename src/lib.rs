mod bytes;
mod error;
mod ess;
pub(crate) mod option_helper;
pub(crate) mod result_helper;

pub use error::*;
pub use ess::*;

use bytes::BytesLE;
use error::ParseError as Error;
use error::ParseResult as Result;
use itermore::IterNextChunk as _;
use option_helper::OptionHelper as _;
use result_helper::ResultHelper as _;

macro_rules! err {
    ($name:ident $( ( $($arg:expr),* ) )? ) => {
        ::std::result::Result::Err(
            crate::error::ParseError::$name $( ( $($arg),* ) )?
        )
    };
}

/// Reads a Windows SystemTime struct from a byte iterator.
/// See: https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime
fn read_systime(bytes: &mut impl BytesLE) -> Result<SysTime> {
    let year = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let month = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let weekday = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let day = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let hour = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let minute = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let seconds = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    let milliseconds = bytes.next_u16().or_err(Error::UnexpectedEOF)?;
    Ok(SysTime {
        year,
        month,
        weekday,
        day,
        hour,
        minute,
        seconds,
        milliseconds,
    })
}

pub fn parse<I>(mut file_bytes: I) -> Result<ESS>
where
    I: bytes::BytesLE,
{
    let file_id: [u8; 12] = file_bytes.next_array().replace_err(Error::NoHeader)?;

    let file_id: String = if &file_id == b"TES4SAVEGAME" {
        // Should be normal.
        "TES4SAVEGAME".into()
    } else if file_id.starts_with(b"CON ") {
        return err!(XboxContainer);
    } else {
        return err!(BadFileHeader(file_id));
    };

    let major_version = file_bytes.next_u8().or_err(Error::UnexpectedEOF)?;
    let minor_version = file_bytes.next_u8().or_err(Error::UnexpectedEOF)?;

    debug_assert_eq!(major_version, 0);
    debug_assert!(minor_version <= 126);

    let sys_time = if minor_version >= 82 {
        Some(read_systime(&mut file_bytes)?)
    } else {
        None
    };

    let file_header = FileHeader::new(file_id, major_version, minor_version, sys_time);

    Ok(ESS::new(file_header))
}

#[cfg(test)]
mod tests;
