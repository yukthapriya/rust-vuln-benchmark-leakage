// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0121 (FIXED)
// Crate: crypto2
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: shadowsocks/crypto2
// Commit: 284cc6d
// File: src/aeadcipher/chacha20_poly1305.rs


pub const NONCE_LEN: usize = Chacha20::NONCE_LEN; // 12 bytes
     pub const TAG_LEN: usize = Poly1305::TAG_LEN; // 16 bytes
 
    pub const A_MAX: usize = usize::MAX;
     #[cfg(target_pointer_width = "32")]
    pub const P_MAX: usize = usize::MAX;
    #[cfg(target_pointer_width = "64")]
     pub const P_MAX: usize = 274877906880; // (2^32 - 1) * BLOCK_LEN
    #[cfg(target_pointer_width = "32")]
    pub const C_MAX: usize = Self::P_MAX - Self::TAG_LEN;
    #[cfg(target_pointer_width = "64")]
     pub const C_MAX: usize = Self::P_MAX + Self::TAG_LEN; // 274,877,906,896
     pub const N_MIN: usize = Self::NONCE_LEN;
     pub const N_MAX: usize = Self::NONCE_LEN;