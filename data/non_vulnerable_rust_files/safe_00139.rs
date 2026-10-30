// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0129 (FIXED)
// Crate: kekbit
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: motoras/kekbit
// Commit: 7a5771e
// File: examples/child_ps.rs


use kekbit::api::ReadError::*;
 use kekbit::api::{EncoderHandler, Reader, Writer};
 use kekbit::core::*;
 use log::{error, info};
 use nix::sys::wait::waitpid;
 use nix::unistd::{fork, getpid, ForkResult};
use simple_logger::SimpleLogger;
 use std::path::Path;
use std::process::exit;
 use std::result::Result;
 
 const ITERATIONS: u32 = 1 * 1_000_000_0;
 }
 
 fn main() {
    SimpleLogger::new().init().unwrap();
     info!("Kekbit Driver PID is {}.", getpid());
     let w_pid = match fork() {
         Ok(ForkResult::Child) => {