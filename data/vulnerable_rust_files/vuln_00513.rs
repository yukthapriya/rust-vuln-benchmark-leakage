// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0045
// CVE: N/A
// CWE: N/A
// Crate: static_cell
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/static_cell
// Extraction Method: repo_search
// GitHub Repo: embassy-rs/static-cell
// Commit: 3ad046b
// File: src/lib.rs


$(#[$m])*
         static STATIC_CELL: $crate::StaticCell<T> = $crate::StaticCell::new();
         #[deny(unused_attributes)]
        let (x,) = unsafe { STATIC_CELL.uninit().write(($val,)) };
         x
     }};
 }