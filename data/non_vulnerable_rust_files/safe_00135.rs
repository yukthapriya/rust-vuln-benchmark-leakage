// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0123 (FIXED)
// Crate: libp2p-deflate
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: libp2p/rust-libp2p
// Commit: ac1404e
// File: transports/webtransport-websys/src/stream.rs


// messages were flushed.
             self.poll_writer_ready(cx)
         } else {
             Poll::Ready(Ok(()))
         }
     }