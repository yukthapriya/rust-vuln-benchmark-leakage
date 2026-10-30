// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0086
// CVE: CVE-2021-45684
// CWE: N/A
// Crate: flumedb
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/flumedb
// Extraction Method: repo_search
// GitHub Repo: sunrise-choir/flumedb-rs
// Commit: 14b7440
// File: src/go_offset_log.rs


{
     // Entry is [payload size: u64, payload ]
 
    let mut buf = Vec::with_capacity(frame.data_size);
    unsafe { buf.set_len(frame.data_size) };
 
     let n = read_at(&mut buf, frame.data_start())?;
     if n < frame.data_size {