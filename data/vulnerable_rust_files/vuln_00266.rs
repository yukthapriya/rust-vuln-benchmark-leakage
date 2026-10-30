// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0001
// CVE: N/A
// CWE: N/A
// Crate: lmdb
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lmdb
// Extraction Method: repo_search
// GitHub Repo: danburkert/lmdb-rs
// Commit: ef25044
// File: lmdb-sys/build.rs


lmdb.push("liblmdb");
 
     if !pkg_config::find_library("liblmdb").is_ok() {
        cc::Build::new()
                    .file(lmdb.join("mdb.c"))
                    .file(lmdb.join("midl.c"))
                    // https://github.com/LMDB/lmdb/blob/LMDB_0.9.21/libraries/liblmdb/Makefile#L25
                    .opt_level(2)
                    .compile("liblmdb.a")
     }
 }