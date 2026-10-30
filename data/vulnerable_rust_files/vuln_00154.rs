// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0151
// CVE: CVE-2020-36471
// CWE: N/A
// Crate: generator
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/generator
// Extraction Method: repo_search
// GitHub Repo: Xudong-Huang/generator-rs
// Commit: b74452c
// File: src/reg_context.rs


let func: fn() = unsafe { transmute(f) };
         func();
 
        let ctx: &RegContext = unsafe { transmute(arg) };
         RegContext::load(ctx);
 
         unreachable!("Should never comeback");