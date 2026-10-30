// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0057 (FIXED)
// Crate: fxhash
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: cbreeden/fxhash
// Commit: 04993c8
// File: lib.rs


//! # Fx Hash
 //!
 //! This hashing algorithm was extracted from the Rustc compiler.  This is the same hashing
//! algorithm used for some internal operations in Firefox.  The strength of this algorithm
 //! is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one
 //! byte at a time.
 //!
 
 use std::collections::{HashMap, HashSet};
 use std::default::Default;
use std::hash::{BuildHasherDefault, Hash, Hasher};
 use std::ops::BitXor;
 
 extern crate byteorder;
 pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;
 
 const ROTATE: u32 = 5;
const SEED64: u64 = 0x51_7c_c1_b7_27_22_0a_95;
const SEED32: u32 = 0x9e_37_79_b9;
 
 #[cfg(target_pointer_width = "32")]
 const SEED: usize = SEED32 as usize;
 #[inline]
 fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {
     while bytes.len() >= 4 {
        hash.hash_word(NativeEndian::read_u32(bytes));
        bytes = &bytes[4..];
     }
 
    if bytes.len() >= 2 {
        hash.hash_word(u32::from(NativeEndian::read_u16(bytes)));
        bytes = &bytes[2..];
     }

    if let Some(&byte) = bytes.first() {
        hash.hash_word(u32::from(byte));
    }

     hash
 }
 
 #[inline]
 fn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {
     while bytes.len() >= 8 {
        hash.hash_word(NativeEndian::read_u64(bytes));
        bytes = &bytes[8..];
     }
 
     if bytes.len() >= 4 {
        hash.hash_word(u64::from(NativeEndian::read_u32(bytes)));
        bytes = &bytes[4..];
     }
 
    if bytes.len() >= 2 {
        hash.hash_word(u64::from(NativeEndian::read_u16(bytes)));
        bytes = &bytes[2..];
     }

    if let Some(&byte) = bytes.first() {
        hash.hash_word(u64::from(byte));
    }

     hash
 }
 
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in Firefox.
 /// The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms,
 /// where the FNV algorithm works on one byte at a time.
 ///
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in Firefox.
 /// The strength of this algorithm is in hashing 8 bytes at a time on any platform,
 /// where the FNV algorithm works on one byte at a time.
 ///
 
     #[inline]
     fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(u64::from(i));
     }
 
     #[inline]
     fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(u64::from(i));
     }
 
     #[inline]
     fn write_u32(&mut self, i: u32) {
        self.hash.hash_word(u64::from(i));
     }
 
     fn write_u64(&mut self, i: u64) {
 }
 
 /// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algorithm used for some internal operations in Firefox.
 /// The strength of this algorithm is in hashing 4 bytes at a time on any platform,
 /// where the FNV algorithm works on one byte at a time.
 ///
 
     #[inline]
     fn write_u8(&mut self, i: u8) {
        self.hash.hash_word(u32::from(i));
     }
 
     #[inline]
     fn write_u16(&mut self, i: u16) {
        self.hash.hash_word(u32::from(i));
     }
 
     #[inline]
 
     #[inline]
     fn finish(&self) -> u64 {
        u64::from(self.hash)
     }
 }
 
     let mut state = FxHasher::default();
     v.hash(&mut state);
     state.finish() as usize
}