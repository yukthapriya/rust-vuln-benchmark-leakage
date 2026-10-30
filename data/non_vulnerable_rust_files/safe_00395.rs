// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0011 (FIXED)
// Crate: snow
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mcginty/snow
// Commit: aea6ec1
// File: examples/oneway.rs


#[cfg(any(feature = "default-resolver", feature = "ring-accelerated"))]
 fn main() {
     let server_mode =
        std::env::args().next_back().is_none_or(|arg| arg == "-s" || arg == "--server");
 
     if server_mode {
         run_server(&RESPONDER.private, SECRET);