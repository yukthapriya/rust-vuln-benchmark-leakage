// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0430 (FIXED)
// Crate: magic-crypt
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: magiclen/rust-magiccrypt
// Commit: ee516ad
// File: src/ciphers/aes128.rs


#[cfg(feature = "std")]
 use block_modes::block_padding::Padding;
 use block_modes::block_padding::Pkcs7;
#[cfg(feature = "std")]
use block_modes::BlockModeError;
use block_modes::{BlockMode, Cbc};
 
 use aes::cipher::{Block, BlockCipherKey};
 use aes::Aes128;