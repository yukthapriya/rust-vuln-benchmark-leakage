// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0044 (FIXED)
// Crate: rocket
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: SergioBenitez/Rocket
// Commit: 504efef
// File: examples/tls/src/main.rs


#[macro_use]
 extern crate rocket;
extern crate self as tracing;
 
 #[cfg(test)]
 mod tests;
 mod redirector;
 
use rocket::tracing::*;
 use rocket::mtls::Certificate;
 use rocket::listener::Endpoint;