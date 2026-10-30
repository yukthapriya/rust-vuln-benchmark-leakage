// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0084 (FIXED)
// Crate: libp2p
// Type: unknown (FIXED)
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