// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0038 (FIXED)
// Crate: fltk
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: MoAlyousef/fltk-rs
// Commit: 1378ebd
// File: fltk-sys/build/link.rs


if !cfg!(feature = "no-gdiplus") {
                     println!("cargo:rustc-link-lib{}gdiplus", linkage);
                 }
                if target_triple.contains("gnullvm") {
                    println!("cargo:rustc-link-lib=c++");
                } else if target_triple.contains("gnu") {
                     println!("cargo:rustc-link-lib=supc++");
                     println!("cargo:rustc-link-lib=gcc");
                 }