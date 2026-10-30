// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0112
// CVE: CVE-2020-36218
// CWE: N/A
// Crate: buttplug
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/buttplug
// Extraction Method: repo_search
// GitHub Repo: buttplugio/buttplug-rs
// Commit: 3218102
// File: crates/intiface_engine/src/engine.rs


.clone();
 
     if let Some(rest_port) = options.rest_api_port() {
      IntifaceRestServer::run(server).await;
       return Ok(());
     }