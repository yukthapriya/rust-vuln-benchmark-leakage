// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0051 (FIXED)
// Crate: outer_cgi
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: SolraBizna/outer_cgi
// Commit: 439d239
// File: src/fcgi.rs


Ok(()) => (),
             Err(x) => return Some(Err(x)),
         }
        Some(Ok((((b1[0] ^ 0x80) as u32) << 24)
                 | ((b2[0] as u32) << 16)
                 | ((b2[1] as u32) << 8)
                 | ((b2[2] as u32))))