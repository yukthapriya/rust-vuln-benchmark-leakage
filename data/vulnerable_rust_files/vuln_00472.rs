// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0436
// CVE: N/A
// CWE: N/A
// Crate: paste
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/paste
// Extraction Method: repo_search
// GitHub Repo: dtolnay/paste
// Commit: 2ee7b01
// File: tests/test_item.rs


use paste::paste;
 
    fn _jit_address(_node: ()) {}
 
     macro_rules! jit_reexport {
         ($fn:ident, $arg:ident : $typ:ty) => {
             paste! {
                 pub fn $fn($arg: $typ) {
                    [<_jit_ $fn>]($arg);
                 }
             }
         };