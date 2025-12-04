#![allow(dead_code)]
use std::fmt::Debug;

use derive_new::new as New;

// https://en.uesp.net/wiki/Oblivion_Mod:Save_File_Format

/// An immutable list. Like an owned slice.
pub type List<T> = Box<[T]>;

// TODO: Should we just convert from this structure to a proper datetime object?
#[derive(Debug)]
/// See: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime
pub struct SysTime {
    pub(crate) year: u16,
    pub(crate) month: u16,
    pub(crate) weekday: u16,
    pub(crate) day: u16,
    pub(crate) hour: u16,
    pub(crate) minute: u16,
    pub(crate) seconds: u16,
    pub(crate) milliseconds: u16,
}

#[derive(New, Debug)]
pub struct RGB(u8, u8, u8);

/// TODO: Make this smarter lmao
#[derive(New)]
pub struct Screenshot {
    width: u32,
    height: u32,
    screen: List<RGB>,
}

impl Debug for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Screenshot ({}x{})", self.width, self.height))
    }
}

#[derive(New, Debug)]
pub struct FileHeader {
    pub file_id: String,
    /// Should always be zero.
    pub major_version: u8,
    /// Should be at most 125. 0.80 is valid; lower bounds unknown.
    pub minor_version: u8,
    /// "Time when game executable was last modified." Bizarre, I know.
    ///
    /// Only available with `version>=0.81`.
    pub exe_time: Option<SysTime>,
}

#[allow(clippy::too_many_arguments)]
#[derive(New, Debug)]
pub struct SaveGameHeader {
    /// Should be equal to FileHeader::minor_version for any particular ESS file.
    pub header_version: u32,

    // Skipping saveHeaderSize: lol?
    /// Save number. Used in default save filename.
    pub save_number: u32,

    /// Player name.
    pub name: String,
    /// Player level.
    pub level: u16,
    /// Player's current location.
    pub cell: String,

    /// Days that have passed in-game.
    pub game_days: f32,

    /// Total number of ticks elapsed uring gameplay.
    /// Equivalent to milliseconds spent in-game.
    pub game_ticks: u32,

    /// Real time at which the save file was created.
    ///
    /// TODO: Figure out why FileHeader::exe_time was introduced.
    pub game_time: SysTime,

    /// Screenshot at time of save.
    pub screenshot: Screenshot,
}

/// An Elder Scrolls (IV) Save.
#[derive(New, Debug)]
pub struct ESS {
    pub file_header: FileHeader,
    pub save_game_header: SaveGameHeader,
    pub plugins: List<String>,
}
