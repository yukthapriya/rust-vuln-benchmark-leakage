// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0082 (FIXED)
// Crate: warp
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: seanmonstar/warp
// Commit: 8673081
// File: src/server.rs


pub fn graceful<Fut>(self, shutdown_signal: Fut) -> Server<F, A, run::Graceful<Fut>>
     where
        Fut: Future<Output = ()> + Send + 'static,
     {
         Server {
             acceptor: self.acceptor,