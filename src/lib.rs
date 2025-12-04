mod error;
mod model;
pub(crate) mod result_helper;
mod scanner;

pub use error::*;
pub use model::*;

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

pub fn parse_record<I: Iterator<Item = u8>>(_scanner: &mut Scanner<'_, I>) -> Result<Record> {
    todo!("create record parser")
}

pub fn parse_ess<I: Iterator<Item = u8>>(file_bytes: &mut I) -> Result<ESS> {
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
    /////////////////////////////////////////

    /////////////////////////////////////////
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
        let screen: List<RGB> = {
            let size = width * height * 3;
            let bytes: List<u8> = scanner.take(size as usize).collect();
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
    /////////////////////////////////////////

    /////////////////////////////////////////
    //////////////// PLUGINS ////////////////

    type Plugins = List<String>;
    let plugins: Plugins = {
        let len = scanner.u8()?;
        let plugins: Result<Plugins> = (0..len).map(|_| scanner.bstring()).collect();
        plugins?
    };

    //////////////// PLUGINS ////////////////
    /////////////////////////////////////////

    /////////////////////////////////////////
    //////////////// GLOBALS ////////////////

    let _form_ids_offset = scanner.u32()?;
    let _records_length = scanner.form_id()?; // not gonna need THIS for a while
    let next_object_id = scanner.form_id()?;
    let world_id = scanner.form_id()?;
    let world_x = scanner.u32()?;
    let world_y = scanner.u32()?;
    let player_location = PlayerLocation::new(
        scanner.form_id()?,
        scanner.f32()?,
        scanner.f32()?,
        scanner.f32()?,
    );
    let globals = {
        let len = scanner.u16()?;
        let mut res = vec![];
        for _ in 0..len {
            let iref = scanner.iref()?;
            let data = scanner.f32()?;
            res.push((iref, data));
        }
        res.into_boxed_slice()
    };
    let death_counts = {
        let _ = scanner.u16()?;
        let len = scanner.u32()?;
        let mut death_counts = vec![];
        for _ in 0..len {
            death_counts.push(DeathCount::new(scanner.iref()?, scanner.u16()?))
        }
        death_counts.into_boxed_slice()
    };
    let game_mode_seconds = scanner.f32()?;
    let processes: List<u8> = {
        let len = scanner.u16()?;
        scanner.take(len as usize).collect()
    };
    let spectator_events: List<u8> = {
        let len = scanner.u16()?;
        scanner.take(len as usize).collect()
    };
    let weather: List<u8> = {
        let len = scanner.u16()?;
        scanner.take(len as usize).collect()
    };
    let player_combat_count = scanner.u32()?;
    let created_items: List<Record> = {
        let len = scanner.u32()?;
        let mut res = vec![];
        for _ in 0..len {
            let record = parse_record(&mut scanner)?;
            res.push(record)
        }
        res.into_boxed_slice()
    };
    let quick_keys: List<Option<IRef>> = {
        let len = scanner.u16()?;
        (0..len)
            .map(|_| match scanner.bool()? {
                true => Ok(Some(scanner.iref()?)),
                false => Ok(None),
            })
            .collect::<Result<List<_>>>()?
    };
    let reticule: List<u8> = {
        let len = scanner.u16()?;
        scanner.take(len as usize).collect()
    };
    let interface: List<u8> = {
        let len = scanner.u16()?;
        scanner.take(len as usize).collect()
    };
    let regions: List<Region> = {
        let _ = scanner.u16()?; // ???
        let len = scanner.u16()?;
        let mut res = vec![];
        for _ in 0..len {
            let region = (scanner.iref()?, scanner.u32()?);
            res.push(region);
        }
        res.into_boxed_slice()
    };

    let global = GlobalSection::new(
        next_object_id,
        world_id,
        world_x,
        world_y,
        player_location,
        globals,
        death_counts,
        game_mode_seconds,
        processes,
        spectator_events,
        weather,
        player_combat_count,
        created_items,
        quick_keys,
        reticule,
        interface,
        regions,
    );
    Ok(ESS::new(file_header, save_header, plugins, global))
}

#[cfg(test)]
mod tests;
