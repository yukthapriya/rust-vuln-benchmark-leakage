// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0032 (FIXED)
// Crate: byte_struct
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: wwylele/byte-struct-rs
// Commit: 26d0b15
// File: byte_struct/src/lib.rs


/// A type that can be packed into or unpacked from raw bytes under given default byte order.
 ///
 /// This trait is implemented for most numeric primitive types,
/// except for `bool`, `char`, `isize` and `usize`. This is also implemented for array types
/// whose element type implements `ByteStructUnspecifiedByteOrder`.
 ///
 /// This trait is automatically implemented for all types that implements [`ByteStruct`].
 /// In this case, all members of `ByteStructUnspecifiedByteOrder` are direct wrappers of [`ByteStruct`] members.