// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0047 (FIXED)
// Crate: lmdb-rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: vhbit/lmdb-rs
// Commit: 4849484
// File: src/core.rs


fn commit(&mut self) -> MdbResult<()> {
         assert_state_eq!(txn, self.state, TransactionState::Normal);
         debug!("commit txn");
         self.state = if self.is_readonly() {
             TransactionState::Released
         } else {
             TransactionState::Invalid
         };
        try_mdb!(unsafe { ffi::mdb_txn_commit(self.handle) } );
         Ok(())
     }