// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0030 (FIXED)
// Crate: mozwire
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: NilsIrl/MozWire
// Commit: 05d110c
// File: src/constants.rs


pub const V1_API: &str = "/api/v1";
 pub const V2_API: &str = "/api/v2";
 pub const IPV4_GATEWAY: Ipv4Addr = Ipv4Addr::new(10, 64, 0, 1);
pub const PORT_RANGES: [(u16, u16); 6] = [
     (53, 53),
     (123, 123),
    (443, 443),
     (4000, 33433),
     (33565, 51820),
    (52001, 60000),
 ];
 pub const EXPLOITATION_ATTEMPT_MESSAGE: &str = "INVALID DATA RETURNED FROM SERVER, THE CONTENT \
                                                 COULD HAVE BEEN TEMPERED IN AN ATTEMPT AT \