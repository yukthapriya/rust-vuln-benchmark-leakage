// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0074 (FIXED)
// Crate: prettytable-rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: phsym/prettytable-rs
// Commit: 4d66e6e
// File: src/cell.rs


}
         }
         Cell {
            content,
            width,
            align,
             style: Vec::new(),
             hspan: 1,
         }
                 // Unknown colors, fallback to blakc
                 _ => "#000000",
             }
        }
 
         let colspan = if self.hspan > 1 {
             format!(" colspan=\"{}\"", self.hspan)