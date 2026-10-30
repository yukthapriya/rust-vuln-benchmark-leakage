// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0377
// CVE: N/A
// CWE: N/A
// Crate: dbn
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/dbn
// Extraction Method: repo_search
// GitHub Repo: databento/dbn
// Commit: 31dbdec
// File: rust/dbn/src/decode/dbn/fsm.rs


fn consume(&mut self, read: usize, compat: usize, compat_fill: usize, expand_compat: bool) {
         self.buffer.consume(read);
         if compat_fill > 0 {
            self.compat_buffer.fill(compat);
         }
         if compat > 0 {
             self.compat_buffer.consume(compat);