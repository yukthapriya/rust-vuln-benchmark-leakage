// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0026 (FIXED)
// Crate: comrak
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: kivikakk/comrak
// Commit: 7d1b3c8
// File: fuzz/fuzz_targets/gfm.rs


use comrak::{markdown_to_html, options, Options};
 
// Note that what I'm targeting here isn't exactly the same
 // as --gfm, but rather an approximation of what cmark-gfm
 // options are routinely used by Commonmarker users.