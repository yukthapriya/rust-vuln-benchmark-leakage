// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0013 (FIXED)
// Crate: resolve
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: murarth/resolve
// Commit: f83509e
// File: examples/resolver.rs


fn main() {
     let config = DnsConfig::with_name_servers(vec![
         // Use Google's public DNS servers instead of the system default.
        "8.8.8.8:53".parse().unwrap(),
        "8.8.4.4:53".parse().unwrap(),
     ]);
 
     let resolver = match DnsResolver::new(config) {