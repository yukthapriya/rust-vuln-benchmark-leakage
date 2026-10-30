// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0165 (FIXED)
// Crate: mozjpeg
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ImageOptim/mozjpeg-rust
// Commit: 4b2dec4
// File: src/compress.rs


unsafe {
                 let mut row_ptrs = [[ptr::null::<u8>(); MAX_MCU_HEIGHT]; MAX_COMPONENTS];
 
                for ((comp_info, &image_src), comp_row_ptrs) in self.components().iter().zip(image_src).zip(row_ptrs.iter_mut()) {
                     let row_stride = comp_info.row_stride();
 
                     let input_height = image_src.len() / row_stride;
                     assert!(comp_height >= 8);
 
                     // row_ptrs were initialized to null
                    for (src_row, row_ptr) in image_src.chunks_exact(row_stride).skip(comp_start_row).take(comp_height).zip(comp_row_ptrs.iter_mut()) {
                        *row_ptr = src_row.as_ptr();
                     }
                 }