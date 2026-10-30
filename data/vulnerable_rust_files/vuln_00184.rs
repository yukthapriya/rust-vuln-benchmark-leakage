// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0032
// CVE: CVE-2021-28033
// CWE: N/A
// Crate: byte_struct
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/byte_struct
// Extraction Method: repo_search
// GitHub Repo: wwylele/byte-struct-rs
// Commit: 26d0b15
// File: byte_struct/src/lib.rs


/// A type that can be packed into or unpacked from raw bytes under given default byte order.
 ///
 /// This trait is implemented for most numeric primitive types,
/// except for `bool`, `char`, `isize` and `usize`.
///
/// This is also implemented for array types whose element type implements `ByteStructUnspecifiedByteOrder`
/// and whose size is between 1 and 32 (inclusive). Similarly, this is implemented for `GenericArray` types (
/// re-expored from generic-array crate) for the same element type and for all sizes.
 ///
 /// This trait is automatically implemented for all types that implements [`ByteStruct`].
 /// In this case, all members of `ByteStructUnspecifiedByteOrder` are direct wrappers of [`ByteStruct`] members.