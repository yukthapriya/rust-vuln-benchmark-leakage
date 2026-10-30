// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0068 (FIXED)
// Crate: cocoon
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: fadeevab/cocoon
// Commit: 604914e
// File: src/lib.rs


//! In the end, you use [`Cocoon`] to put the final image into an encrypted container.
 //!
 //! ```
//! use borsh::{BorshDeserialize, BorshSerialize};
 //! use cocoon::{Cocoon, Error};
 //!
 //! use std::collections::HashMap;
 //! use std::fs::File;
 //!
 //! // Your data can be represented in any way.
//! #[derive(BorshDeserialize, BorshSerialize)]
 //! struct Database {
 //!     inner: HashMap<String, String>,
 //! }
 //!     // Dump the serialized database into a file as an encrypted container.
 //!     let container = cocoon.dump(encoded, &mut file)?;
 //!
//!     // Let's look at how to decrypt the container and parse it back.
//!     let mut file = File::open("target/test.db").unwrap();
//!     let encoded = cocoon.parse(&mut file).unwrap();
//!     let decoded = Database::try_from_slice(&encoded).unwrap();
//!
 //!     Ok(())
 //! }
 //! ```