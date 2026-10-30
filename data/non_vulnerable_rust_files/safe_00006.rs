// REAL FIXED CODE
// RustSec ID: RUSTSEC-2016-0006 (FIXED)
// Crate: cassandra
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tupshin/cassandra-rs
// Commit: 771c923
// File: src/cassandra/result.rs


}
 }
 
//FIXME Understand why this drop results in double freeing when binding by name
//impl Drop for CassResult {
//    fn drop(&mut self) {
//        unsafe { cass_result_free(self.0) }
//    }
//}
 
 impl CassResult {
     ///Gets the number of rows for the specified result.