// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0056
// CVE: N/A
// CWE: N/A
// Crate: adler
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/adler
// Extraction Method: repo_search
// GitHub Repo: jonas-schievink/adler
// Commit: 27ab6c2
// File: src/lib.rs


#[cfg(test)]
 mod tests {
     use super::*;
    use std::io::BufReader;
 
     #[test]
     fn zeroes() {
         assert_eq!(adler.checksum(), 0x8e88ef11); // from above
     }
 
     #[test]
     fn bufread() {
         fn test(data: &[u8], checksum: u32) {
             // `BufReader` uses an 8 KB buffer, so this will test buffer refilling.
             let mut buf = BufReader::new(data);