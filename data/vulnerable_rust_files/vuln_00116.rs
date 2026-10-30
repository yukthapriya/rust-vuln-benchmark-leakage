// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0103
// CVE: CVE-2020-36210
// CWE: N/A
// Crate: autorand
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/autorand
// Extraction Method: repo_search
// GitHub Repo: mersinvald/autorand-rs
// Commit: 39f9eae
// File: src/lib.rs


$(
         impl<T: Random> Random for [T; $s] {
             fn random() -> Self {
                 unsafe {
                    let mut array: [T; $s] = std::mem::uninitialized();
                    for i in 0..$s {
                        std::ptr::write(&mut array[i], T::random());
                     }
                    array
                 }
             }
         }