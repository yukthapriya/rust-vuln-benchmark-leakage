// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0056 (FIXED)
// Crate: adler
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: jonas-schievink/adler
// Commit: 27ab6c2
// File: src/lib.rs


#[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn zeroes() {
         assert_eq!(adler.checksum(), 0x8e88ef11); // from above
     }
 
    #[cfg(feature = "std")]
     #[test]
     fn bufread() {
        use std::io::BufReader;
         fn test(data: &[u8], checksum: u32) {
             // `BufReader` uses an 8 KB buffer, so this will test buffer refilling.
             let mut buf = BufReader::new(data);