// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0019 (FIXED)
// Crate: blake2
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: RustCrypto/MACs
// Commit: 678a63e
// File: belt-mac/benches/mod.rs


#![feature(test)]
 extern crate test;
 
 use belt_mac::{BeltMac, KeyInit};
 use test::Bencher;
 
 digest::bench_update!(
    BeltMac::new(&Default::default());
     belt_mac_10 10;
     belt_mac_100 100;
     belt_mac_1000 1000;