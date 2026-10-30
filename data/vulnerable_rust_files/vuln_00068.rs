// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0030
// CVE: CVE-2020-35883
// CWE: N/A
// Crate: mozwire
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/mozwire
// Extraction Method: repo_search
// GitHub Repo: NilsIrl/MozWire
// Commit: 05d110c
// File: src/constants.rs


pub const V1_API: &str = "/api/v1";
 pub const V2_API: &str = "/api/v2";
 pub const IPV4_GATEWAY: Ipv4Addr = Ipv4Addr::new(10, 64, 0, 1);
pub const PORT_RANGES: [(u16, u16); 5] = [
     (53, 53),
     (123, 123),
     (4000, 33433),
     (33565, 51820),
    (52000, 60000),
 ];
 pub const EXPLOITATION_ATTEMPT_MESSAGE: &str = "INVALID DATA RETURNED FROM SERVER, THE CONTENT \
                                                 COULD HAVE BEEN TEMPERED IN AN ATTEMPT AT \