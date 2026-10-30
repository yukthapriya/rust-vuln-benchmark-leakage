// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0389
// CVE: N/A
// CWE: N/A
// Crate: openslide
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/openslide
// Extraction Method: repo_search
// GitHub Repo: ojskrede/openslide-rust
// Commit: d64a5cb
// File: src/bindings.rs


let null_terminated_array_ptr = openslide_get_property_names(osr);
         let mut counter = 0;
         let mut loc = null_terminated_array_ptr;
        while (*loc).is_null() {
             counter += 1;
             loc = loc.offset(1);
         }