// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0069
// CVE: N/A
// CWE: N/A
// Crate: hyper-staticfile
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hyper-staticfile
// Extraction Method: repo_search
// GitHub Repo: stephank/hyper-staticfile
// Commit: 5153e68
// File: src/resolve.rs


return Ok(ResolveResult::Found(ResolvedFile::new(
                     file,
                     zstd_path.into(),
                    mime,
                     Some(Encoding::Zstd),
                 )));
             }