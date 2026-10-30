// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0123
// CVE: CVE-2020-36443
// CWE: N/A
// Crate: libp2p-deflate
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/libp2p-deflate
// Extraction Method: repo_search
// GitHub Repo: libp2p/rust-libp2p
// Commit: ac1404e
// File: transports/webtransport-websys/src/stream.rs


// messages were flushed.
             self.poll_writer_ready(cx)
         } else {
            debug_assert!(
                false,
                "libp2p_webtransport_websys::Stream: poll_flush called after poll_close"
            );
             Poll::Ready(Ok(()))
         }
     }