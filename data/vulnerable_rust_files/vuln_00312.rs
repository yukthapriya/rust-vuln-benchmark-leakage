// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0082
// CVE: N/A
// CWE: N/A
// Crate: warp
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/warp
// Extraction Method: repo_search
// GitHub Repo: seanmonstar/warp
// Commit: 8673081
// File: src/server.rs


pub fn graceful<Fut>(self, shutdown_signal: Fut) -> Server<F, A, run::Graceful<Fut>>
     where
        F: Future<Output = ()> + Send + 'static,
     {
         Server {
             acceptor: self.acceptor,