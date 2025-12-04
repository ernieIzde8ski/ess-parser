#![allow(dead_code, clippy::too_many_arguments)]

mod record;
mod screenshot;
mod system_time;

pub use record::*;
pub use screenshot::*;
pub use system_time::*;

use std::fmt::Debug;

use derive_more::*;
use derive_new::new as New;

// https://en.uesp.net/wiki/Oblivion_Mod:Save_File_Format

/// An immutable list. Like an owned slice.
pub type List<T> = Box<[T]>;

#[repr(transparent)]
#[derive(New, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormID(u32);

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

#[derive(New, Debug)]
pub struct PlayerLocation {
    /// "Not iref"
    cell: FormID,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(New, Debug, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct IRef(u32);

#[derive(New, Debug)]
pub struct DeathCount {
    actor: IRef,
    /// Number of times this actor has died.
    total: u16,
}

pub type Region = (IRef, u32);

#[derive(New, Debug)]
pub struct GlobalSection {
    next_object_id: FormID,
    world_id: FormID,
    world_x: u32,
    world_y: u32,

    /// Player location - that is, cell location & position within cell.
    player_location: PlayerLocation,

    /// List of global variables, mapping from iref to value.
    /// Always a float in this array, regardless of their real types.
    globals: List<(IRef, f32)>,

    death_counts: List<DeathCount>,

    game_mode_seconds: f32,

    /// "Processes data."
    processes: List<u8>,
    /// "Spectator Event data."
    spectator_events: List<u8>,
    /// "Sky/Weather data."
    weather: List<u8>,

    /// Number of actors in combat with the player.
    player_combat_count: u32,
    /// Items created in-game.
    created_items: List<Record>,

    quick_keys: List<Option<IRef>>,
    // "HUD Reticule."
    reticule: List<u8>,
    interface: List<u8>,
    regions: List<Region>,
}

/// An Elder Scrolls (IV) Save.
#[derive(New, Debug)]
pub struct ESS {
    pub file_header: FileHeader,
    pub save_game_header: SaveGameHeader,
    pub plugins: List<String>,
    pub globals: GlobalSection,
}
