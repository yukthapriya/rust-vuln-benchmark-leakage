// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0053
// CVE: N/A
// CWE: N/A
// Crate: dirs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/dirs
// Extraction Method: repo_search
// GitHub Repo: dirs-dev/dirs-rs
// Commit: 291cbed
// File: src/lib.rs


/// Returns the path to the user's home directory.
 ///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None.
 ///
 /// |Platform | Value                | Example        |
 /// | ------- | -------------------- | -------------- |