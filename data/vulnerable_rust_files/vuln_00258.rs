// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0143
// CVE: CVE-2021-21235
// CWE: N/A
// Crate: kamadak-exif
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/kamadak-exif
// Extraction Method: commit_diff
// GitHub Repo: kamadak/exif-rs
// Commit: 1b05eab
// File: src/util.rs


fn discard_exact(&mut self, mut len: usize) -> io::Result<()> {
         while len > 0 {
             let consume_len = match self.fill_buf() {
                 Ok(buf) => buf.len().min(len),
                 Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                 Err(e) => return Err(e),
     use std::io::Read;
     use super::*;
 
     #[test]
     fn read8_len() {
         let data = [];