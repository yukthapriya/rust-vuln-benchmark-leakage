// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0069 (FIXED)
// Crate: hyper-staticfile
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: stephank/hyper-staticfile
// Commit: 5153e68
// File: src/resolve.rs


return Ok(ResolveResult::Found(ResolvedFile::new(
                     file,
                     zstd_path.into(),
                    mimetype,
                     Some(Encoding::Zstd),
                 )));
             }