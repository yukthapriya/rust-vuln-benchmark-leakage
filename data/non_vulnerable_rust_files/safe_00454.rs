// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0401 (FIXED)
// Crate: zlib-rs
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: trifectatechfoundation/zlib-rs
// Commit: 3d80ae1
// File: fuzz/fuzz_targets/inflate_chunked.rs


ReturnCode::StreamEnd => continue,
             _ => {
                 if stream.msg.is_null() {
                    panic!("{return_code:?}: <no error message>")
                 } else {
                     let msg = unsafe { std::ffi::CStr::from_ptr(stream.msg) };
                    panic!("{return_code:?}: {msg:?}")
                 }
             }
         }