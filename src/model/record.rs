use bitfields::bitfield;
use derive_more::*;
use derive_new::new as New;

#[bitfield(u32)]
#[derive(Clone, Copy)]
pub struct RecordFlag {
    esm_file: bool,
    #[bits(4)]
    _pad: u8,
    deleted: bool,
    #[bits(3)]
    _pad: u8,
    casts_shadows: bool,
    persistent_reference: bool,
    initially_disabled: bool,
    ignored: bool,
    #[bits(2)]
    _pad: u8,
    visible_when_distant: bool,
    #[bits(1)]
    _pad: u8,
    dangerous: bool,
    compressed: bool,
    cant_wait: bool,
    #[bits(12)]
    _pad: u16,
}

#[derive(New, Debug, From, Clone)]
pub struct Record {
    kind: [u8; 4],
    flags: RecordFlag,
}
