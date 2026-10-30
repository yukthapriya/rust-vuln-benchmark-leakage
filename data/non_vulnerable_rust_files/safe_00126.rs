// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0114 (FIXED)
// Crate: va-ts
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: video-audio/va-ts
// Commit: e3e90e1
// File: src/demuxer.rs


events: T,
 }
 
unsafe impl<T> Send for Demuxer<T> where T: DemuxerEvents + Send {}
 
 impl<T> Demuxer<T>
 where