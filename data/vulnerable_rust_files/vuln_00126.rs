// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0114
// CVE: CVE-2020-36220
// CWE: N/A
// Crate: va-ts
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/va-ts
// Extraction Method: repo_search
// GitHub Repo: video-audio/va-ts
// Commit: e3e90e1
// File: src/demuxer.rs


events: T,
 }
 
unsafe impl<T> Send for Demuxer<T> where T: DemuxerEvents {}
 
 impl<T> Demuxer<T>
 where