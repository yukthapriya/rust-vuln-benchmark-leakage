// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0020 (FIXED)
// Crate: generator
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Xudong-Huang/generator-rs
// Commit: b74452c
// File: src/reg_context.rs


let func: fn() = unsafe { transmute(f) };
         func();
 
        let ctx: &RegContext = unsafe { &*std::ptr::with_exposed_provenance(arg) };
         RegContext::load(ctx);
 
         unreachable!("Should never comeback");