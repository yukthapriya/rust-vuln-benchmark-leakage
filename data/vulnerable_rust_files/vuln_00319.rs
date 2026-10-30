// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0094
// CVE: N/A
// CWE: N/A
// Crate: mimalloc
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/mimalloc
// Extraction Method: repo_search
// GitHub Repo: purpleprotocol/mimalloc_rust
// Commit: edee487
// File: libmimalloc-sys/sys-test/build.rs


"{cargo_manifest_dir}/../c_src/mimalloc/{version}/include"
         ))
         .cfg("feature", Some("extended"))
        .cfg("feature", (version == "v3").then(|| "v3"))
         .fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string())
         // ignore whether or not the option enum is signed.
         .skip_signededness(|c| c.ends_with("_t") || c.ends_with("_e"))