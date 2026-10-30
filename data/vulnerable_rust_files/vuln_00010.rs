// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2017-0006
// CVE: N/A
// CWE: N/A
// Crate: rmpv
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rmpv
// Extraction Method: repo_search
// GitHub Repo: 3Hren/msgpack-rust
// Commit: a59d577
// File: rmp/tests/func/decode/mod.rs


#[cfg(feature = "std")]
 pub type Cursor<'a> = std::io::Cursor<&'a [u8]>;
 #[cfg(not(feature = "std"))]
pub type Cursor<'a> = crate::msgpack::decode::Bytes<'a>;