// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0024
// CVE: CVE-2020-15093
// CWE: N/A
// Crate: tough
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tough
// Extraction Method: repo_search
// GitHub Repo: awslabs/tough
// Commit: 3f51ff6
// File: tough/src/lib.rs


pub use crate::transport::IntoVec;
 pub use crate::transport::{
     DefaultTransport, FilesystemTransport, Transport, TransportError, TransportErrorKind,
 };
 pub use crate::urlpath::SafeUrlPath;
 use async_recursion::async_recursion;