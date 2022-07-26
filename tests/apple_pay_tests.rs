use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use openssl::pkcs12::Pkcs12;

#[test]
fn apple_pay_test() {
    let pass = "password";
    let cert_path = OsString::from(format!("{}\\tests\\resources\\keystore.p12", env!("CARGO_MANIFEST_DIR")));
    let cert = Path::new(&cert_path);
    let mut cert_file = File::open(cert).unwrap();
    let mut buf = Vec::new();
    let _ = cert_file.read_to_end(&mut buf).unwrap();
    let pkcs = Pkcs12::from_der(&buf).unwrap();
    let cert_data = pkcs.parse(pass);
    assert_eq!(cert_data.is_ok(), true)
}