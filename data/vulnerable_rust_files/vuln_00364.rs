// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0068
// CVE: CVE-2024-21530
// CWE: N/A
// Crate: cocoon
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cocoon
// Extraction Method: repo_search
// GitHub Repo: fadeevab/cocoon
// Commit: 604914e
// File: src/lib.rs


//! In the end, you use [`Cocoon`] to put the final image into an encrypted container.
 //!
 //! ```
//! use borsh::BorshSerialize;
 //! use cocoon::{Cocoon, Error};
 //!
 //! use std::collections::HashMap;
 //! use std::fs::File;
 //!
 //! // Your data can be represented in any way.
//! #[derive(BorshSerialize)]
 //! struct Database {
 //!     inner: HashMap<String, String>,
 //! }
 //!     // Dump the serialized database into a file as an encrypted container.
 //!     let container = cocoon.dump(encoded, &mut file)?;
 //!
 //!     Ok(())
 //! }
 //! ```