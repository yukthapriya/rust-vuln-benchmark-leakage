// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0124 (FIXED)
// Crate: async-coap
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: google/rust-async-coap
// Commit: 889179e
// File: async-coap-uri/src/escape/unescape_uri.rs


/// Attempts to losslessly decode the string, returning it as a copy-on-write type.
     ///
    /// # Errors
    ///
    /// If the string cannot be decoded losslessly, then a [`DecodingError`] is
    /// returned, from which the location of the decoding error can be obtained
    /// with [`DecodingError::index`].
     #[cfg(feature = "std")]
     pub fn try_to_cow(&self) -> Result<Cow<'a, str>, DecodingError> {
         let as_str = self.iter.as_str();