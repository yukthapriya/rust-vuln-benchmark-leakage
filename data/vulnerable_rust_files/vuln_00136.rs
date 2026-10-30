// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0124
// CVE: CVE-2020-36444
// CWE: N/A
// Crate: async-coap
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/async-coap
// Extraction Method: repo_search
// GitHub Repo: google/rust-async-coap
// Commit: 889179e
// File: async-coap-uri/src/escape/unescape_uri.rs


/// Attempts to losslessly decode the string, returning it as a copy-on-write type.
     ///
    /// If the string cannot be decoded losslessly, then the location of the encoding
    /// error is returned as an `Err`.
     #[cfg(feature = "std")]
     pub fn try_to_cow(&self) -> Result<Cow<'a, str>, DecodingError> {
         let as_str = self.iter.as_str();