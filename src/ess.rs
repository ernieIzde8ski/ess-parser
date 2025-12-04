#![allow(dead_code)]
use derive_new::new as New;

#[derive(Debug)]
/// See: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime
///
/// TODO: Should we just convert from this structure to a proper datetime object?
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

/// An Elder Scrolls (IV) Save.
#[derive(New, Debug)]
pub struct ESS {
    pub file_header: FileHeader,
}
