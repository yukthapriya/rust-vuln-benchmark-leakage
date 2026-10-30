// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0070 (FIXED)
// Crate: pingora-core
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: cloudflare/pingora
// Commit: fa1d565
// File: pingora-core/src/lib.rs


//! If looking to build a (reverse) proxy, see [`pingora-proxy`](https://docs.rs/pingora-proxy) crate.
 //!
 //! # Optional features
//!
//! ## TLS backends (mutually exclusive)
//! - `openssl`: Use OpenSSL as the TLS library (default if no TLS feature is specified)
//! - `boringssl`: Use BoringSSL as the TLS library (FIPS compatible)
//! - `rustls`: Use Rustls as the TLS library
//!
//! ## Additional features
//! - `connection_filter`: Enable early TCP connection filtering before TLS handshake.
//!   This allows implementing custom logic to accept/reject connections based on peer address
//!   with zero overhead when disabled.
//! - `sentry`: Enable Sentry error reporting integration
//! - `patched_http1`: Enable patched HTTP/1 parser
//!
//! # Connection Filtering
//!
//! With the `connection_filter` feature enabled, you can implement early connection filtering
//! at the TCP level, before any TLS handshake or HTTP processing occurs. This is useful for:
//! - IP-based access control
//! - Rate limiting at the connection level
//! - Geographic restrictions
//! - DDoS mitigation
//!
//! ## Example
//!
//! ```rust,ignore
//! # #[cfg(feature = "connection_filter")]
//! # {
//! use async_trait::async_trait;
//! use pingora_core::listeners::ConnectionFilter;
//! use std::net::SocketAddr;
//! use std::sync::Arc;
//!
//! #[derive(Debug)]
//! struct MyFilter;
//!
//! #[async_trait]
//! impl ConnectionFilter for MyFilter {
//!     async fn should_accept(&self, addr: &SocketAddr) -> bool {
//!         // Custom logic to filter connections
//!         !is_blocked_ip(addr.ip())
//!     }
//! }
//!
//! // Apply the filter to a service
//! let mut service = my_service();
//! service.set_connection_filter(Arc::new(MyFilter));
//! # }
//! ```
//!
//! When the `connection_filter` feature is disabled, the filter API remains available
//! but becomes a no-op, ensuring zero overhead for users who don't need this functionality.
 
 // This enables the feature that labels modules that are only available with
 // certain pingora features