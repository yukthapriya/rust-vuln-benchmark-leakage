// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0071 (FIXED)
// Crate: rusoto_credential
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rusoto/rusoto
// Commit: 001054d
// File: rusoto/core/src/request.rs


"DELETE" => Method::DELETE,
         "GET" => Method::GET,
         "HEAD" => Method::HEAD,
        "PATCH" => Method::PATCH,
         v => {
             return Err(HttpDispatchError {
                 message: format!("Unsupported HTTP verb {}", v),