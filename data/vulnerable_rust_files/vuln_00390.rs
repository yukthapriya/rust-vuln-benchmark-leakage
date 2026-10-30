// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0005
// CVE: N/A
// CWE: N/A
// Crate: threadalone
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/threadalone
// Extraction Method: repo_search
// GitHub Repo: cr0sh/threadalone
// Commit: 2ad1bd4
// File: src/lib.rs


//!     message: String,
 //! }
 //!
//! fn main() {
//!     let err = Error {
//!         span: ThreadAlone::new(Span {
//!             index: 99,
//!             marker: PhantomData,
//!         }),
//!         message: "fearless concurrency".to_owned(),
//!     };
 //!
//!     // Original thread can see the contents.
//!     assert_eq!(err.span.get_ref().unwrap().index, 99);
 //!
//!     let err = Arc::new(err);
//!     let err2 = err.clone();
//!     std::thread::spawn(move || {
//!         // Other threads cannot get access. Maybe they use
//!         // a default value or a different codepath.
//!         assert!(err2.span.get_ref().is_none());
//!     });
 //!
//!     // Original thread can still see the contents.
//!     assert_eq!(err.span.get_ref().unwrap().index, 99);
//! }
 //! ```
 
 #![doc(html_root_url = "https://docs.rs/threadalone/0.1.0")]