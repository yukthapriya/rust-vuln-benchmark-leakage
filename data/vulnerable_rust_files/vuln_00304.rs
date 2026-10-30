// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0074
// CVE: N/A
// CWE: N/A
// Crate: prettytable-rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/prettytable-rs
// Extraction Method: repo_search
// GitHub Repo: phsym/prettytable-rs
// Commit: 4d66e6e
// File: src/cell.rs


}
         }
         Cell {
            content: content,
            width: width,
            align: align,
             style: Vec::new(),
             hspan: 1,
         }
                 // Unknown colors, fallback to blakc
                 _ => "#000000",
             }
        };
 
         let colspan = if self.hspan > 1 {
             format!(" colspan=\"{}\"", self.hspan)