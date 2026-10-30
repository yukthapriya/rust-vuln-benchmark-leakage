// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0029
// CVE: CVE-2021-28030
// CWE: N/A
// Crate: truetype
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/truetype
// Extraction Method: repo_search
// GitHub Repo: bodoni/truetype
// Commit: 1d50ea8
// File: src/tables/names/encoding/macintosh.rs


}
     };
     for character in value.chars() {
        if let Some(value) = mapping.get(&character) {
            data.push(*value);
         } else {
             raise!("found an unknown Macintosh character ({character})")
         }