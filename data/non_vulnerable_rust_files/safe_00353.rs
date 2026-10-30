// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0046 (FIXED)
// Crate: cyfs-base
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: buckyos/CYFS
// Commit: 134e6f9
// File: src/component/cyfs-lib/src/stack/stack.rs


}
 
     pub async fn stop(&self) {
        let requestor_holder = self.requestor_holder.read().unwrap().clone();
         requestor_holder.stop().await;
 
         self.router_handlers.stop().await;