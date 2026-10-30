// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0153 (FIXED)
// Crate: encoding
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: lifthrasiir/rust-encoding
// Commit: 280564a
// File: src/lib.rs


#![cfg_attr(test, feature(test))] // lib stability features as per RFC #507
 
extern crate encoding_types;
 extern crate encoding_index_singlebyte as index_singlebyte;
 extern crate encoding_index_korean as index_korean;
 extern crate encoding_index_japanese as index_japanese;
 pub use self::types::{CodecError, ByteWriter, StringWriter,
                       RawEncoder, RawDecoder, EncodingRef, Encoding,
                       EncoderTrapFunc, DecoderTrapFunc, DecoderTrap,
                      EncoderTrap}; // reexport
use std::borrow::Cow;
 
 #[macro_use] mod util;
 #[cfg(test)] #[macro_use] mod testutils;
 pub mod all;
 pub mod label;
 
/// Determine the encoding by looking for a Byte Order Mark (BOM)
/// and decoded a single string in memory.
/// Return the result and the used encoding.
pub fn decode(input: &[u8], trap: DecoderTrap, fallback_encoding: EncodingRef)
           -> (Result<String, Cow<'static, str>>, EncodingRef) {
    use all::{UTF_8, UTF_16LE, UTF_16BE};
    if input.starts_with(&[0xEF, 0xBB, 0xBF]) {
        (UTF_8.decode(&input[3..], trap), UTF_8 as EncodingRef)
    } else if input.starts_with(&[0xFE, 0xFF]) {
        (UTF_16BE.decode(&input[2..], trap), UTF_16BE as EncodingRef)
    } else if input.starts_with(&[0xFF, 0xFE]) {
        (UTF_16LE.decode(&input[2..], trap), UTF_16LE as EncodingRef)
    } else {
        (fallback_encoding.decode(input, trap), fallback_encoding)
    }
}

 #[cfg(test)]
 mod tests {
     use super::*;