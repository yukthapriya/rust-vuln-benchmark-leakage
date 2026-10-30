// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0107
// CVE: CVE-2020-36215
// CWE: N/A
// Crate: hashconsing
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hashconsing
// Extraction Method: repo_search
// GitHub Repo: AdrienChampion/hashconsing
// Commit: 70008d0
// File: src/hash_coll/hashers.rs


use std::hash::{BuildHasher as StdBuilderExt, Hasher as StdHasherExt};
 
     /// P-hash-er factory.
    #[derive(Clone)]
     pub struct Builder {}
     impl Builder {
         /// Empty factory constructor.
     use std::hash::{BuildHasher as StdBuilderExt, Hasher as StdHasherExt};
 
     /// P-hash-er factory.
    #[derive(Clone)]
     pub struct Builder {}
     impl Builder {
         /// Empty factory constructor.