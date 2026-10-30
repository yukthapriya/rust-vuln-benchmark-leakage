// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0071
// CVE: N/A
// CWE: N/A
// Crate: rusoto_credential
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rusoto_credential
// Extraction Method: repo_search
// GitHub Repo: rusoto/rusoto
// Commit: 001054d
// File: rusoto/core/src/request.rs


"DELETE" => Method::DELETE,
         "GET" => Method::GET,
         "HEAD" => Method::HEAD,
         v => {
             return Err(HttpDispatchError {
                 message: format!("Unsupported HTTP verb {}", v),