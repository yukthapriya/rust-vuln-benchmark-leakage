// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0054 (FIXED)
// Crate: rkyv
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: djkoloski/rkyv
// Commit: 998a0a3
// File: rkyv/src/lib.rs


rustdoc::missing_crate_level_docs
 )]
 #![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(docsrs, not(doctest)), feature(doc_cfg))]
 #![doc(html_favicon_url = r#"
     data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0
     26.458 26.458'%3E%3Cpath d='M0 0v26.458h26.458V0zm9.175 3.772l8.107 8.106