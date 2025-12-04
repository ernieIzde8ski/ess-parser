use crate::RecordFlag;
use crate::parse_ess;

const SAVE325: &[u8] = include_bytes!("../tests/Save325Kheros.ess");

#[test]
fn read_save_325() {
    let mut bytes = SAVE325.iter().cloned();
    let ess = parse_ess(&mut bytes).expect("should be able to parse Save325Kheros.ess");
    // panic!("{:#?}", ess);
    let header = ess.file_header;
    assert_eq!(header.minor_version, 125); // 125

    let exe_time = header.exe_time.expect("should exist");
    assert_eq!(exe_time.year, 2022);
    assert_eq!(exe_time.month, 6);
    assert_eq!(exe_time.day, 2);

    let header = ess.save_game_header;
    assert_eq!(header.name, "Kheros");

    let plugins = ess.plugins;
    assert_eq!(plugins.len(), 28);
}

#[test]
fn new_record_flag() {
    let record_flag = RecordFlag::from_bits(0x0000021);
    assert!(record_flag.esm_file());
    assert!(record_flag.deleted());
    assert!(!record_flag.casts_shadows());
}
