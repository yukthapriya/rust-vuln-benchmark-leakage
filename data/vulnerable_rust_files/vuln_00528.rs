// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0070
// CVE: CVE-2025-8671
// CWE: N/A
// Crate: pingora-core
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pingora-core
// Extraction Method: repo_search
// GitHub Repo: cloudflare/pingora
// Commit: fa1d565
// File: pingora-core/src/lib.rs


//! If looking to build a (reverse) proxy, see [`pingora-proxy`](https://docs.rs/pingora-proxy) crate.
 //!
 //! # Optional features
//! `boringssl`: Switch the internal TLS library from OpenSSL to BoringSSL.
 
 // This enables the feature that labels modules that are only available with
 // certain pingora features