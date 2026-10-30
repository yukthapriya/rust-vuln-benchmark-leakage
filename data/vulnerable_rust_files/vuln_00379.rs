// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0086
// CVE: N/A
// CWE: N/A
// Crate: lexical-core
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lexical-core
// Extraction Method: repo_search
// GitHub Repo: Alexhuszagh/rust-lexical
// Commit: 57dfd21
// File: extras/benchmark/write-float/special.rs


let value = fastrand::$i($exp_mask..);
             // NOTE: We want mem::transmute, not from_bits because we
             // don't want the special handling of from_bits
            #[allow(clippy::transmute_int_to_float)]
             vec.push(unsafe { mem::transmute::<$i, $f>(value) });
         }
         vec