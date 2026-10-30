// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0045 (FIXED)
// Crate: static_cell
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: embassy-rs/static-cell
// Commit: 3ad046b
// File: src/lib.rs


$(#[$m])*
         static STATIC_CELL: $crate::StaticCell<T> = $crate::StaticCell::new();
         #[deny(unused_attributes)]
        let (x,) = STATIC_CELL.uninit().write(($val,));
         x
     }};
 }