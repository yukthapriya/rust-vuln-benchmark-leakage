// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0091
// CVE: CVE-2021-45689
// CWE: N/A
// Crate: gfx-auxil
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/gfx-auxil
// Extraction Method: repo_search
// GitHub Repo: gfx-rs/gfx
// Commit: bc77309
// File: src/backend/metal/src/internal.rs


let data = if cfg!(target_os = "macos") {
             &include_bytes!("./../shaders/gfx-shaders-macos.metallib")[..]
         } else if cfg!(target_arch = "aarch64") {
            &include_bytes!("./../shaders/gfx-shaders-ios.metallib")[..]
         } else {
             &include_bytes!("./../shaders/gfx-shaders-ios-simulator.metallib")[..]
         };