// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0043 (FIXED)
// Crate: ws
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: housleyjk/ws-rs
// Commit: 339e782
// File: examples/html_chat.rs


}
     }
 
    // Handle messages received in the websocket (in this case, only on /ws)
     fn on_message(&mut self, msg: Message) -> Result<()> {
         // Broadcast to all connections
         self.out.broadcast(msg)