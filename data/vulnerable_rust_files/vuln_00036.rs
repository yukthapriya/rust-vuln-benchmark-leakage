// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0015
// CVE: CVE-2019-16139
// CWE: N/A
// Crate: compact_arena
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/compact_arena
// Extraction Method: repo_search
// GitHub Repo: llogiq/compact_arena
// Commit: d01ed1c
// File: tests/tiny_arena.rs


#[test]
 fn two_tiny_arenas() {
    assert_eq!(3, in_tiny_arena!(a, {
        in_tiny_arena!(b, {
            let x = a.add(1usize);
            let y = b.add(2usize);
            a[x] + b[y]
        })
    }));
 }
 
 #[test]
 fn two_tiny_arenas_one_scope() {
    mk_tiny_arena!(a);
    mk_tiny_arena!(b);
    let x = a.add(1usize);
    let y = b.add(2usize);
    assert_eq!(3, a[x] + b[y]);
 }