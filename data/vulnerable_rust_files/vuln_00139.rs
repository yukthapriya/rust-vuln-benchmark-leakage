// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0129
// CVE: CVE-2020-36449
// CWE: N/A
// Crate: kekbit
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/kekbit
// Extraction Method: repo_search
// GitHub Repo: motoras/kekbit
// Commit: 7a5771e
// File: examples/child_ps.rs


use kekbit::api::ReadError::*;
 use kekbit::api::{EncoderHandler, Reader, Writer};
 use kekbit::core::*;
use std::process::exit;

 use log::{error, info};
 use nix::sys::wait::waitpid;
 use nix::unistd::{fork, getpid, ForkResult};
 use std::path::Path;
 use std::result::Result;
 
 const ITERATIONS: u32 = 1 * 1_000_000_0;
 }
 
 fn main() {
    simple_logger::init().unwrap();
     info!("Kekbit Driver PID is {}.", getpid());
     let w_pid = match fork() {
         Ok(ForkResult::Child) => {