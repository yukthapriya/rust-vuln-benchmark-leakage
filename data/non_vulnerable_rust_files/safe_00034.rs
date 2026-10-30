// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0013 (FIXED)
// Crate: spin
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mvdnes/spin-rs
// Commit: 71a1d8e
// File: src/rwlock.rs


/// A lock that provides data access to either one writer or many readers.
 ///
 /// This lock behaves in a similar manner to its namesake `std::sync::RwLock` but uses
/// spinning for synchronisation instead. Unlike its namesake, this lock does not
 /// track lock poisoning.
 ///
 /// This type of lock allows a number of readers or at most one writer at any