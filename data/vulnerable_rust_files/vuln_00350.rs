// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0043
// CVE: N/A
// CWE: N/A
// Crate: ftp
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ftp
// Extraction Method: repo_search
// GitHub Repo: mattnenterprise/rust-ftp
// Commit: 3ad9eff
// File: examples/connecting.rs


extern crate ftp;
 
 use ftp::{FtpError, FtpStream};
use std::io::Cursor;
use std::str;
 
 fn test_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    let mut ftp_stream = FtpStream::connect((addr, 21)).unwrap();
     ftp_stream.login(user, pass).unwrap();
     println!("current dir: {}", ftp_stream.pwd().unwrap());