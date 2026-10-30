// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0002 (FIXED)
// Crate: prost
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: danburkert/prost
// Commit: 28e4522
// File: prost-build/src/code_generator.rs


for (idx, oneof) in message.oneof_decl.into_iter().enumerate() {
                 let idx = idx as i32;
                // optional fields create a synthetic oneof that we want to skip
                let fields = match oneof_fields.remove(&idx) {
                    Some(fields) => fields,
                    None => continue,
                };
                self.append_oneof(&fq_message_name, oneof, idx, fields);
             }
 
             self.pop_mod();