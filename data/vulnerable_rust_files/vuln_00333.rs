// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0016
// CVE: N/A
// CWE: N/A
// Crate: partial_sort
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/partial_sort
// Extraction Method: repo_search
// GitHub Repo: sundy-li/partial_sort
// Commit: 54c63c5
// File: benches/sort.rs


b.iter_batched_ref(
             || create_vec::<u64>(n),
             |v| {
                let (p, _, _) = v.select_nth_unstable(20);
                 p.sort_unstable();
             },
             BatchSize::SmallInput,