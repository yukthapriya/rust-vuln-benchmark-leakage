// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0434
// CVE: CVE-2024-52813
// CWE: N/A
// Crate: matrix-sdk-crypto
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/matrix-sdk-crypto
// Extraction Method: repo_search
// GitHub Repo: matrix-org/matrix-rust-sdk
// Commit: eed5f11
// File: crates/matrix-sdk-common/src/linked_chunk/updates.rs


Clear,
 }
 
 /// A collection of [`Update`]s that can be observed.
 ///
 /// Get a value for this type with [`LinkedChunk::updates`].
     use futures_util::pin_mut;
 
     use super::{super::LinkedChunk, ChunkIdentifier, Position, UpdatesInner};
 
     #[test]
     fn test_updates_take_and_garbage_collector() {
         drop(linked_chunk);
         assert_matches!(updates_subscriber1.as_mut().poll_next(&mut context1), Poll::Ready(None));
     }
 }