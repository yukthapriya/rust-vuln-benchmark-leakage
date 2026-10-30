// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0430
// CVE: N/A
// CWE: N/A
// Crate: magic-crypt
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/magic-crypt
// Extraction Method: repo_search
// GitHub Repo: magiclen/rust-magiccrypt
// Commit: ee516ad
// File: src/ciphers/aes128.rs


#[cfg(feature = "std")]
 use block_modes::block_padding::Padding;
 use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, BlockModeError, Cbc};
 
 use aes::cipher::{Block, BlockCipherKey};
 use aes::Aes128;