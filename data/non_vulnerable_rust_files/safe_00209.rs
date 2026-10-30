// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0073 (FIXED)
// Crate: prost-types
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tokio-rs/prost
// Commit: 1d7344b
// File: prost/src/encoding.rs


//! Utility functions and types for encoding and decoding Protobuf types.
 //!
//! This module contains the encoding and decoding primitives for Protobuf as described in
 //! <https://protobuf.dev/programming-guides/encoding/>.
 //!
 //! This module is `pub`, but is only for prost internal use. The `prost-derive` crate needs access for its `Message` implementations.
     ]);
 
     #[test]
    /// `decode_varint` accepts a `Buf`, which can be multiple concatenated buffers.
     /// This test ensures that future optimizations don't break the
     /// `decode_varint` for non-continuous memory.
     fn split_varint_decoding() {