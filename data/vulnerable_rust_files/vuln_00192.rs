// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0044
// CVE: CVE-2021-29935
// CWE: N/A
// Crate: rocket
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rocket
// Extraction Method: repo_search
// GitHub Repo: SergioBenitez/Rocket
// Commit: 504efef
// File: examples/tls/src/main.rs


#[macro_use]
 extern crate rocket;
 
 #[cfg(test)]
 mod tests;
 mod redirector;
 
 use rocket::mtls::Certificate;
 use rocket::listener::Endpoint;