// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0142
// CVE: CVE-2020-36462
// CWE: N/A
// Crate: syncpool
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/syncpool
// Extraction Method: repo_search
// GitHub Repo: Chopinsky/byte_buffer
// Commit: 15b2828
// File: syncpool/src/bucket.rs


}
 }
 
unsafe impl<T> Send for Bucket2<T> {}
 
 pub(crate) struct RingBucket<T> {
     /// The actual data store. Data are stored in heap and not managed by the runtime, so we must