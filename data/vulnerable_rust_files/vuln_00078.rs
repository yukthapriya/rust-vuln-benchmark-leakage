// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0043
// CVE: CVE-2020-35896
// CWE: N/A
// Crate: ws
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/ws
// Extraction Method: repo_search
// GitHub Repo: housleyjk/ws-rs
// Commit: 339e782
// File: examples/html_chat.rs


}
     }
 
    // Handle messages recieved in the websocket (in this case, only on /ws)
     fn on_message(&mut self, msg: Message) -> Result<()> {
         // Broadcast to all connections
         self.out.broadcast(msg)