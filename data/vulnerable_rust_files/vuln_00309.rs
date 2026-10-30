// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0079
// CVE: N/A
// CWE: N/A
// Crate: elf_rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/elf_rs
// Extraction Method: repo_search
// GitHub Repo: vincenthouyi/elf_rs
// Commit: 3ec9935
// File: examples/readelf.rs


use elf_rs::*;
 
fn read_elf(filename: &String) -> Result<(), ()> {
     let mut elf_file = File::open(filename).map_err(|e| {
         println!("failed to open file {}: {}", filename, e);
         ()
         ()
     })?;
 
    println!("{:?} header: {:?}", elf, elf.elf_header());
 
     for p in elf.program_header_iter() {
        println!("{:x?}", p);
     }
 
     for s in elf.section_header_iter() {
        println!("{:x?}", s);
     }
 
     if let Some(s) = elf.lookup_section(b".text") {
        println!(".test section {:?}", s);
     }
 
     Ok(())