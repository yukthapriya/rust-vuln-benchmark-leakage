// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0153
// CVE: N/A
// CWE: N/A
// Crate: encoding
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/encoding
// Extraction Method: repo_search
// GitHub Repo: lifthrasiir/rust-encoding
// Commit: 280564a
// File: src/lib.rs


#![cfg_attr(test, feature(test))] // lib stability features as per RFC #507
 
 extern crate encoding_index_singlebyte as index_singlebyte;
 extern crate encoding_index_korean as index_korean;
 extern crate encoding_index_japanese as index_japanese;
 pub use self::types::{CodecError, ByteWriter, StringWriter,
                       RawEncoder, RawDecoder, EncodingRef, Encoding,
                       EncoderTrapFunc, DecoderTrapFunc, DecoderTrap,
                      EncoderTrap, decode}; // reexport
 
 #[macro_use] mod util;
 #[cfg(test)] #[macro_use] mod testutils;
 pub mod all;
 pub mod label;
 
 #[cfg(test)]
 mod tests {
     use super::*;