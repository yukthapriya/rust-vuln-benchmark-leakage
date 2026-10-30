// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0404 (FIXED)
// Crate: anstream
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-cli/anstyle
// Commit: 6fa6aeb
// File: crates/anstyle-svg/src/lib.rs


.unwrap();
     }
     if effects_in_use.contains(anstyle::Effects::DIMMED) {
        writeln!(buffer, r#"    .dimmed {{ opacity: 0.4; }}"#).unwrap();
     }
     if effects_in_use.contains(anstyle::Effects::HIDDEN) {
         writeln!(buffer, r#"    .hidden {{ opacity: 0; }}"#).unwrap();