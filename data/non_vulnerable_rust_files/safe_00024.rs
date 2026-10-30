// REAL FIXED CODE
// RustSec ID: RUSTSEC-2018-0021 (FIXED)
// Crate: libpulse-binding
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: jnqnfe/pulse-binding-rust
// Commit: fc15f1f
// File: pulse-binding/src/context/ext_device_restore.rs


/// Creates a copy with owned data.
     pub fn to_owned(&self) -> Self {
         Info {
            formats: self.formats.iter().map(format::Info::to_owned).collect(),
             ..*self
         }
     }