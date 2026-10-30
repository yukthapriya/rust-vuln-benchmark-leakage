// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0405 (FIXED)
// Crate: rustyscript
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rscarson/rustyscript
// Commit: bca5dc8
// File: src/ext/web_stub/mod.rs


mod encoding;
 mod timers;
use timers::StartTime;
 
 extension!(
     deno_web,
     ],
     esm_entry_point = "ext:deno_web/init_stub.js",
     esm = [ dir "src/ext/web_stub", "init_stub.js", "01_dom_exception.js", "02_timers.js", "05_base64.js" ],
    state = |state| {
        state.put(StartTime::default());
    }
 );
 impl ExtensionTrait<()> for deno_web {
     fn init((): ()) -> Extension {