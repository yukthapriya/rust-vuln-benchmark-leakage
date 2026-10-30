// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0007
// CVE: CVE-2019-15549
// CWE: N/A
// Crate: asn1_der
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/asn1_der
// Extraction Method: repo_search
// GitHub Repo: KizzyCode/asn1_der
// Commit: 62b94c7
// File: src/lib.rs


//! object.encode(&mut encoded_object).expect("Failed to encode object");
 //!
 //! // Decode a `u8`
//! let number = u8::decode(INT7).expect("Failed to decode string");
 //! assert_eq!(number, 7);
 //!
 //! // Encode a new `u8`
 //! let mut encoded_number = Vec::new();
//! 7u8.encode(&mut encoded_number).expect("Failed to encode string");
 //! # }
 //! ```
 //!