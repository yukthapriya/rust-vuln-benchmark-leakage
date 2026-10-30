// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0079 (FIXED)
// Crate: elf_rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: vincenthouyi/elf_rs
// Commit: 3ec9935
// File: examples/readelf.rs


use elf_rs::*;
 
fn read_elf(filename: &str) -> Result<(), ()> {
     let mut elf_file = File::open(filename).map_err(|e| {
         println!("failed to open file {}: {}", filename, e);
         ()
         ()
     })?;
 
    println!("{:#x?}", elf);

    println!("{:#?}", elf.elf_header());
 
     for p in elf.program_header_iter() {
        println!("{:#x?}", p);
     }
 
     for s in elf.section_header_iter() {
        println!("{:#x?}", s);
     }
 
     if let Some(s) = elf.lookup_section(b".text") {
        println!(".test section: {:#x?}", s);
     }
 
     Ok(())