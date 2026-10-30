// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0077
// CVE: N/A
// CWE: N/A
// Crate: claim
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/claim
// Extraction Method: repo_search
// GitHub Repo: svartalf/rust-claim
// Commit: d5ed345
// File: build.rs


cfg.emit_path_cfg("core::task::Poll", "has_task_poll");
     cfg.emit_path_cfg("std::task::Poll", "has_task_poll");
 
    // Needed for `assert_matches!` and `?` macro repetition
    // See https://doc.rust-lang.org/edition-guide/rust-2018/macros/at-most-once.html
    cfg.emit_rustc_version(1, 32);
 
     if cfg.probe_rustc_version(1, 15) && !cfg.probe_rustc_version(1, 16) {
         autocfg::emit("has_private_in_public_issue");