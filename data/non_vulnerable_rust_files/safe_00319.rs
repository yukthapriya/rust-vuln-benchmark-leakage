// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0094 (FIXED)
// Crate: mimalloc
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: purpleprotocol/mimalloc_rust
// Commit: edee487
// File: libmimalloc-sys/sys-test/build.rs


"{cargo_manifest_dir}/../c_src/mimalloc/{version}/include"
         ))
         .cfg("feature", Some("extended"))
        .cfg("feature", (version == "v3").then_some("v3"))
         .fn_cname(|rust, link_name| link_name.unwrap_or(rust).to_string())
         // ignore whether or not the option enum is signed.
         .skip_signededness(|c| c.ends_with("_t") || c.ends_with("_e"))