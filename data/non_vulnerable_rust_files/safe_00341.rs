// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0032 (FIXED)
// Crate: ntru
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: FrinkGlobal/ntru-rs
// Commit: b31bb88
// File: src/ffi.rs


pub fn ntru_rand_release(rand_ctx: *mut RandContext) -> uint8_t;
 
     #[cfg(target_os = "windows")]
    pub fn ntru_rand_wincrypt_init(rand_ctx: *mut RandContext,
                                    rand_gen: *const RandGen)
                                    -> uint8_t;
     #[cfg(target_os = "windows")]
     pub fn ntru_rand_wincrypt_generate(rand_data: *mut uint8_t,
                                        len: uint16_t,
                                       rand_ctx: *const RandContext)
                                        -> uint8_t;
     #[cfg(target_os = "windows")]
    pub fn ntru_rand_wincrypt_release(rand_ctx: *mut RandContext) -> uint8_t;
 
     #[cfg(not(target_os = "windows"))]
     pub fn ntru_rand_devrandom_init(rand_ctx: *mut RandContext,