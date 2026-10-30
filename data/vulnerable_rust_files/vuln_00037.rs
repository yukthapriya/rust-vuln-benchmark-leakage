// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0016
// CVE: CVE-2019-16140
// CWE: N/A
// Crate: chttp
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/chttp
// Extraction Method: crate_versions
// GitHub Repo: chttp
// Commit: N/A
// File: lib.rs


extern crate curl;
pub extern crate http;
#[cfg(feature = "json")]
extern crate json;

pub mod client;
pub mod transport;
mod body;
mod buffer;
mod error;
mod redirect;

pub use body::Body;
pub use client::Client;
pub use error::Error;
pub use redirect::RedirectPolicy;


pub type Request = http::Request<Body>;
pub type Response = http::Response<Body>;


/// Sends a GET request.
pub fn get(uri: &str) -> Result<Response, Error> {
    Client::default().get(uri)
}

/// Sends a POST request.
pub fn post<B: Into<Body>>(uri: &str, body: B) -> Result<Response, Error> {
    Client::default().post(uri, body)
}

/// Sends a PUT request.
pub fn put<B: Into<Body>>(uri: &str, body: B) -> Result<Response, Error> {
    Client::default().put(uri, body)
}

/// Sends a DELETE request.
pub fn delete(uri: &str) -> Result<Response, Error> {
    Client::default().delete(uri)
}
