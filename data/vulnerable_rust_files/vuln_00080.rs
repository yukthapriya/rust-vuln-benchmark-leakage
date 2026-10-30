// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0045
// CVE: CVE-2020-35898
// CWE: N/A
// Crate: actix-utils
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/actix-utils
// Extraction Method: repo_search
// GitHub Repo: actix/actix-net
// Commit: 5837740
// File: actix-tls/src/lib.rs


#![doc(html_logo_url = "https://actix.rs/img/logo.png")]
 #![doc(html_favicon_url = "https://actix.rs/favicon.ico")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
 
 #[cfg(feature = "openssl")]
 #[allow(unused_extern_crates)]