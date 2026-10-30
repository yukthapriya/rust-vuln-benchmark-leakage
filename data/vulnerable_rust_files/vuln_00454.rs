// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0401
// CVE: N/A
// CWE: N/A
// Crate: zlib-rs
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/zlib-rs
// Extraction Method: repo_search
// GitHub Repo: trifectatechfoundation/zlib-rs
// Commit: 3d80ae1
// File: fuzz/fuzz_targets/inflate_chunked.rs


ReturnCode::StreamEnd => continue,
             _ => {
                 if stream.msg.is_null() {
                    panic!("{:?}: <no error message>", return_code)
                 } else {
                     let msg = unsafe { std::ffi::CStr::from_ptr(stream.msg) };
                    panic!("{:?}: {:?}", return_code, msg)
                 }
             }
         }