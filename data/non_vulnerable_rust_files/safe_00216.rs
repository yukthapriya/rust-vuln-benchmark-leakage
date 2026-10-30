// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0086 (FIXED)
// Crate: flumedb
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: sunrise-choir/flumedb-rs
// Commit: 14b7440
// File: src/go_offset_log.rs


{
     // Entry is [payload size: u64, payload ]
 
    let mut buf = vec![0; frame.data_size];
 
     let n = read_at(&mut buf, frame.data_start())?;
     if n < frame.data_size {