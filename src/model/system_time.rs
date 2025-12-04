// TODO: Should we just convert from this structure to a proper datetime object?

/// See: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime
#[derive(Debug)]
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
