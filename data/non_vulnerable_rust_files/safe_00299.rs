// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0067 (FIXED)
// Crate: lzf
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: badboy/lzf-rs
// Commit: a3d8e4b
// File: src/compress.rs


return Err(LzfError::NoCompressionPossible);
             }
 
            len += 1;
            while len < maxlen && data[ref_offset + len] == data[current_offset + len] {
                 len += 1;
             }
 
             len -= 2; /* len is now #octets - 1 */