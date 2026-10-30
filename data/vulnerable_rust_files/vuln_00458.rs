// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0405
// CVE: N/A
// CWE: N/A
// Crate: rustyscript
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rustyscript
// Extraction Method: repo_search
// GitHub Repo: rscarson/rustyscript
// Commit: bca5dc8
// File: src/ext/web_stub/mod.rs


mod encoding;
 mod timers;
 
 extension!(
     deno_web,
     ],
     esm_entry_point = "ext:deno_web/init_stub.js",
     esm = [ dir "src/ext/web_stub", "init_stub.js", "01_dom_exception.js", "02_timers.js", "05_base64.js" ],
 );
 impl ExtensionTrait<()> for deno_web {
     fn init((): ()) -> Extension {