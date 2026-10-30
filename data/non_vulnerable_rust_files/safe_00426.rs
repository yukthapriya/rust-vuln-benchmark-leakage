// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0360 (FIXED)
// Crate: xmp_toolkit
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: adobe/xmp-toolkit-rs
// Commit: 0766706
// File: src/xmp_gps.rs


/// Will return `None` if the value can not be parsed.
 ///
 /// # Example
///
 /// ```
 /// # use xmp_toolkit::xmp_gps;
 /// assert_eq!(xmp_gps::exif_latitude_to_decimal("47,0N"), Some(47.0));