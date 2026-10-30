// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0091 (FIXED)
// Crate: gfx-auxil
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: gfx-rs/gfx
// Commit: bc77309
// File: src/backend/metal/src/internal.rs


let data = if cfg!(target_os = "macos") {
             &include_bytes!("./../shaders/gfx-shaders-macos.metallib")[..]
         } else if cfg!(target_arch = "aarch64") {
            if env!("TARGET") == "aarch64-apple-ios-sim" { 
                &include_bytes!("./../shaders/gfx-shaders-ios-simulator.metallib")[..]
            } else {
                &include_bytes!("./../shaders/gfx-shaders-ios.metallib")[..]
            }
         } else {
             &include_bytes!("./../shaders/gfx-shaders-ios-simulator.metallib")[..]
         };