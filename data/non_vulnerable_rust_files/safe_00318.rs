// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0092 (FIXED)
// Crate: rmp-serde
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: 3Hren/msgpack-rust
// Commit: a59d577
// File: rmp/tests/func/decode/mod.rs


#[cfg(feature = "std")]
 pub type Cursor<'a> = std::io::Cursor<&'a [u8]>;
 #[cfg(not(feature = "std"))]
pub type Cursor<'a> = rmp::decode::Bytes<'a>;