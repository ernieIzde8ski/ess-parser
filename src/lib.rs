mod bytes;
mod error;
mod ess;
pub(crate) mod result_helper;

pub use error::*;
pub use ess::*;

use bytes::BytesLE;
use encoding_rs::WINDOWS_1252;
use error::ParseError as Error;
use error::ParseResult as Result;
use itermore::IterNextChunk as _;
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
    let year = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let month = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let weekday = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let day = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let hour = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let minute = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let seconds = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let milliseconds = bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
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

// TODO: Move all of this & the `bytes` file into a new Scanner struct.
fn read_bzstring(bytes: &mut impl BytesLE) -> Result<String> {
    let len = bytes.next_u8().ok_or(Error::UnexpectedEOF)? as usize;
    let encoded_bytes: Vec<u8> = bytes.take(len).take_while(|b| *b != 0).collect();
    let (cowstr, _, _) = WINDOWS_1252.decode(&encoded_bytes);
    Ok(cowstr.into())
}

pub fn parse<I>(mut file_bytes: I) -> Result<ESS>
where
    I: bytes::BytesLE,
{
    ////////////// FILE HEADER //////////////
    let file_id: [u8; 12] = file_bytes.next_array().replace_err(Error::NoHeader)?;

    let file_id: String = if &file_id == b"TES4SAVEGAME" {
        // Should be normal.
        "TES4SAVEGAME".into()
    } else if file_id.starts_with(b"CON ") {
        return err!(XboxContainer);
    } else {
        return err!(BadFileHeader(file_id));
    };

    let major_version = file_bytes.next_u8().ok_or(Error::UnexpectedEOF)?;
    let minor_version = file_bytes.next_u8().ok_or(Error::UnexpectedEOF)?;

    debug_assert_eq!(major_version, 0);
    debug_assert!(minor_version <= 126);

    let sys_time = if minor_version >= 82 {
        Some(read_systime(&mut file_bytes)?)
    } else {
        None
    };

    let file_header = FileHeader::new(file_id, major_version, minor_version, sys_time);

    ////////////// SAVE HEADER //////////////

    let header_version = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;
    debug_assert_eq!(header_version, minor_version as u32);

    let _save_header_size = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;

    let save_number = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;

    let name = read_bzstring(&mut file_bytes)?;
    let level = file_bytes.next_u16().ok_or(Error::UnexpectedEOF)?;
    let cell = read_bzstring(&mut file_bytes)?;

    let game_days = file_bytes.next_f32().ok_or(Error::UnexpectedEOF)?;
    let game_ticks = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;
    let game_time = read_systime(&mut file_bytes)?;

    let screenshot = {
        let size = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;
        let width = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;
        let height = file_bytes.next_u32().ok_or(Error::UnexpectedEOF)?;
        debug_assert_eq!(size, width * height * 3 + 8);
        let screen: Box<[RGB]> = {
            let size = width * height * 3;
            let bytes: Box<[u8]> = file_bytes.take(size as usize).collect();
            bytes.chunks(3).map(|c| RGB::new(c[0], c[1], c[2])).collect()
        };
        Screenshot::new(width, height, screen)
    };

    let save_header = SaveGameHeader::new(
        header_version,
        save_number,
        name,
        level,
        cell,
        game_days,
        game_ticks,
        game_time,
        screenshot,
    );

    Ok(ESS::new(file_header, save_header))
}

#[cfg(test)]
mod tests;
