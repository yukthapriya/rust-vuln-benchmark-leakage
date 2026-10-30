// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0165
// CVE: N/A
// CWE: N/A
// Crate: mozjpeg
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/mozjpeg
// Extraction Method: repo_search
// GitHub Repo: ImageOptim/mozjpeg-rust
// Commit: 4b2dec4
// File: src/compress.rs


unsafe {
                 let mut row_ptrs = [[ptr::null::<u8>(); MAX_MCU_HEIGHT]; MAX_COMPONENTS];
 
                for ((comp_info, image_src), (row_ptrs, comp_ptrs)) in self.components().iter().zip(image_src).zip(row_ptrs.iter_mut().zip(comp_ptrs.iter_mut())) {
                     let row_stride = comp_info.row_stride();
 
                     let input_height = image_src.len() / row_stride;
                     assert!(comp_height >= 8);
 
                     // row_ptrs were initialized to null
                    for (image_src, row_ptr) in image_src[comp_start_row..].chunks_exact(row_stride).zip(row_ptrs.iter_mut()).take(comp_height) {
                        *row_ptr = image_src.as_ptr();
                     }
                 }