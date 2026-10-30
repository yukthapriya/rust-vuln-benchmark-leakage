// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2018-0021
// CVE: CVE-2018-25027
// CWE: N/A
// Crate: libpulse-binding
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/libpulse-binding
// Extraction Method: repo_search
// GitHub Repo: jnqnfe/pulse-binding-rust
// Commit: fc15f1f
// File: pulse-binding/src/context/ext_device_restore.rs


/// Creates a copy with owned data.
     pub fn to_owned(&self) -> Self {
         Info {
            formats: self.formats.clone(), //Our implementation makes an owned copy!
             ..*self
         }
     }