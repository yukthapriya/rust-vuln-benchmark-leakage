// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0043 (FIXED)
// Crate: ftp
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mattnenterprise/rust-ftp
// Commit: 3ad9eff
// File: examples/connecting.rs


extern crate ftp;
 
 use ftp::{FtpError, FtpStream};
use std::{io::Cursor, str};
 
 fn test_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    let (mut ftp_stream, _welcome_msg) = FtpStream::connect((addr, 21)).unwrap();
     ftp_stream.login(user, pass).unwrap();
     println!("current dir: {}", ftp_stream.pwd().unwrap());