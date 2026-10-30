// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0065
// CVE: CVE-2023-43669
// CWE: N/A
// Crate: tungstenite
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tungstenite
// Extraction Method: repo_search
// GitHub Repo: snapview/tungstenite-rs
// Commit: 7f3d46e
// File: examples/autobahn-client.rs


}
 
 fn run_test(case: u32) -> Result<()> {
    info!("Running test case {}", case);
     let case_url = format!("ws://localhost:9001/runCase?case={case}&agent={AGENT}");
     let (mut socket, _) = connect(case_url)?;
     loop {