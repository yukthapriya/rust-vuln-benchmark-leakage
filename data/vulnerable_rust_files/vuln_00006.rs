// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2016-0006
// CVE: N/A
// CWE: N/A
// Crate: cassandra
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cassandra
// Extraction Method: repo_search
// GitHub Repo: tupshin/cassandra-rs
// Commit: 771c923
// File: src/cassandra/result.rs


}
 }
 
impl Drop for CassResult {
    fn drop(&mut self) {
        unsafe { cass_result_free(self.0) }
    }
}
 
 impl CassResult {
     ///Gets the number of rows for the specified result.