// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0038
// CVE: CVE-2021-28306
// CWE: N/A
// Crate: fltk
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/fltk
// Extraction Method: repo_search
// GitHub Repo: MoAlyousef/fltk-rs
// Commit: 1378ebd
// File: fltk-sys/build/link.rs


if !cfg!(feature = "no-gdiplus") {
                     println!("cargo:rustc-link-lib{}gdiplus", linkage);
                 }
                if target_triple.contains("gnu") {
                     println!("cargo:rustc-link-lib=supc++");
                     println!("cargo:rustc-link-lib=gcc");
                 }