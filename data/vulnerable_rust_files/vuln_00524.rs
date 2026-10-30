// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0057
// CVE: N/A
// CWE: N/A
// Crate: fxhash
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/fxhash
// Extraction Method: repo_search
// GitHub Repo: cbreeden/fxhash
// Commit: 04993c8
// File: lib.rs


//! # Fx Hash
 //!
 //! This hashing algorithm was extracted from the Rustc compiler.  This is the same hashing
//! algorithm used for some internal operations in FireFox.  The strength of this algorithm
 //! is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one
 //! byte at a time.
 //!
 
 use std::collections::{HashMap, HashSet};
 use std::default::Default;
use std::hash::{Hasher, Hash, BuildHasherDefault};
 use std::ops::BitXor;
 
 extern crate byteorder;
 pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;
 
 const ROTATE: u32 = 5;
const SEED64: u64 = 0x517cc1b727220a95;
const SEED32: u32 = (SEED64 & 0xFFFF_FFFF) as u32;
 
 #[cfg(target_pointer_width = "32")]
 const SEED: usize = SEED32 as usize;
 #[inline]
 fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {
     while bytes.len() >= 4 {
        let n = NativeEndian::read_u32(bytes);
        hash.hash_word(n);
        bytes = bytes.split_at(4).1;
     }
 
    for byte in bytes {
        hash.hash_word(*byte as u32);
     }
     hash
 }
 
 #[inline]
 fn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {
     while bytes.len() >= 8 {
        let n = NativeEndian::read_u64(bytes);
        hash.hash_word(n);
        bytes = bytes.split_at(8).1;
     }
 
     if bytes.len() >= 4 {
        let n = NativeEndian::read_u32(bytes);
        hash.hash_word(n as u64);
        bytes = bytes.split_at(4).1;
     }
 
    for byte in bytes {
        hash.hash_word(*byte as u64);
     }
     hash
 }
 
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in FireFox.
 /// The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms,
 /// where the FNV algorithm works on one byte at a time.
 ///
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in FireFox.
 /// The strength of this algorithm is in hashing 8 bytes at a time on any platform,
 /// where the FNV algorithm works on one byte at a time.
 ///
 
     #[inline]
     fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(i as u64);
     }
 
     #[inline]
     fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(i as u64);
     }
 
     #[inline]
     fn write_u32(&mut self, i: u32) {
        self.hash.hash_word(i as u64);
     }
 
     fn write_u64(&mut self, i: u64) {
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in FireFox.
 /// The strength of this algorithm is in hashing 4 bytes at a time on any platform,
 /// where the FNV algorithm works on one byte at a time.
 ///
 
     #[inline]
     fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(i as u32);
     }
 
     #[inline]
     fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(i as u32);
     }
 
     #[inline]
 
     #[inline]
     fn finish(&self) -> u64 {
        self.hash as u64
     }
 }
 
     let mut state = FxHasher::default();
     v.hash(&mut state);
     state.finish() as usize
}