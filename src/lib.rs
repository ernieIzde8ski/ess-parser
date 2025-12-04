mod error;
mod ess;
pub(crate) mod result_helper;
mod scanner;

pub use error::*;
pub use ess::*;

use error::ParseError as Error;
use error::ParseResult as Result;
use itermore::IterNextChunk as _;
use result_helper::ResultHelper as _;
use scanner::Scanner;

macro_rules! err {
    ($name:ident $( ( $($arg:expr),* ) )? ) => {
        ::std::result::Result::Err(
            crate::error::ParseError::$name $( ( $($arg),* ) )?
        )
    };
}

pub fn parse<I: Iterator<Item = u8>>(file_bytes: &mut I) -> Result<ESS> {
    let mut scanner: Scanner<'_, I> = Scanner::new(file_bytes);

    ////////////// FILE HEADER //////////////
    let file_id: [u8; 12] = scanner.next_array().replace_err(Error::NoHeader)?;

    let file_id: String = if &file_id == b"TES4SAVEGAME" {
        // Should be normal.
        "TES4SAVEGAME".into()
    } else if file_id.starts_with(b"CON ") {
        return err!(XboxContainer);
    } else {
        return err!(BadFileID(file_id));
    };

    let major_version = scanner.u8()?;
    let minor_version = scanner.u8()?;

    debug_assert_eq!(major_version, 0);
    debug_assert!(minor_version <= 126);

    let sys_time = if minor_version >= 82 {
        Some(scanner.systime()?)
    } else {
        None
    };

    let file_header = FileHeader::new(file_id, major_version, minor_version, sys_time);

    ////////////// FILE HEADER //////////////
    ////////////// SAVE HEADER //////////////

    let header_version = scanner.u32()?;
    debug_assert_eq!(header_version, minor_version as u32);

    let _save_header_size = scanner.u32()?;

    let save_number = scanner.u32()?;

    let name = scanner.bzstring()?;
    let level = scanner.u16()?;
    let cell = scanner.bzstring()?;

    let game_days = scanner.f32()?;
    let game_ticks = scanner.u32()?;
    let game_time = scanner.systime()?;

    let screenshot = {
        let size = scanner.u32()?;
        let width = scanner.u32()?;
        let height = scanner.u32()?;
        debug_assert_eq!(size, width * height * 3 + 8);
        let screen: Box<[RGB]> = {
            let size = width * height * 3;
            let bytes: Box<[u8]> = scanner.take(size as usize).collect();
            bytes
                .chunks(3)
                .map(|c| RGB::new(c[0], c[1], c[2]))
                .collect()
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

    ////////////// SAVE HEADER //////////////

    Ok(ESS::new(file_header, save_header))
}

#[cfg(test)]
mod tests;
