use mktape::*;

use std::path::Path;

#[test]
fn tap_file_created_with_valid_input() {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR").expect("std::env::var(CARGO_MANIFEST_DIR)");
    let data_dir = format!("{}/data", base_dir);

    let tap_filename = std::env::temp_dir()
        .join("outfile.tap")
        .display()
        .to_string();

    let args = format!("mktape {} create {}/f0:512 {}/f1:512 {}/f2:512 {}/f3:512 {}/f4:512 {}/f5:10240 {}/f6:10240",
        tap_filename,
        data_dir, data_dir, data_dir, data_dir, data_dir, data_dir, data_dir);
    let args = Vec::from_iter(args.split(' ').map(String::from));

    assert_eq!(mktape(args), Ok(()));

    let tap_path = Path::new(&tap_filename);
    assert!(tap_path.exists());

    let content = std::fs::read(tap_path).ok().unwrap();
    // Housekeeping...
    std::fs::remove_file(tap_path).expect("tap_file_created_with_valid_input:: test failed to housekeep");

    use openssl::sha::Sha1;
    let mut sha1 = Sha1::new();
    sha1.update(&content);
    let hash = sha1.finish();

    let hex = hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    assert_eq!(hex, "e6188335c0c9a3e3fbdc9c29615f940233722432".to_string());
}
