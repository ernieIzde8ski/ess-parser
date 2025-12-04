use crate::parse;

const SAVE325: &[u8] = include_bytes!("../tests/Save325Kheros.ess");

#[test]
fn read_save_325() {
    let mut bytes = SAVE325.iter().cloned();
    let ess = parse(&mut bytes).expect("should be able to parse Save325Kheros.ess");
    let header = ess.file_header;
    assert_eq!(header.minor_version, 125); // 125

    let exe_time = header.exe_time.expect("should exist");
    assert_eq!(exe_time.year, 2022);
    assert_eq!(exe_time.month, 6);
    assert_eq!(exe_time.day, 2);
}
