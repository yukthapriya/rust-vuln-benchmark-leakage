// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0002
// CVE: CVE-2020-35858
// CWE: N/A
// Crate: prost
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/prost
// Extraction Method: repo_search
// GitHub Repo: danburkert/prost
// Commit: 28e4522
// File: prost-build/src/code_generator.rs


for (idx, oneof) in message.oneof_decl.into_iter().enumerate() {
                 let idx = idx as i32;
                self.append_oneof(
                    &fq_message_name,
                    oneof,
                    idx,
                    oneof_fields.remove(&idx).unwrap(),
                );
             }
 
             self.pop_mod();