// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0047
// CVE: N/A
// CWE: N/A
// Crate: lmdb-rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lmdb-rs
// Extraction Method: repo_search
// GitHub Repo: vhbit/lmdb-rs
// Commit: 4849484
// File: src/core.rs


fn commit(&mut self) -> MdbResult<()> {
         assert_state_eq!(txn, self.state, TransactionState::Normal);
         debug!("commit txn");
        try_mdb!(unsafe { ffi::mdb_txn_commit(self.handle) } );
         self.state = if self.is_readonly() {
             TransactionState::Released
         } else {
             TransactionState::Invalid
         };
         Ok(())
     }