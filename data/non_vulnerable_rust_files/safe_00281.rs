// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0031 (FIXED)
// Crate: rulex
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rulex-rs/rulex
// Commit: 1dc87c2
// File: pomsky-bin/src/args/help.rs


const ENGINES: &[HelpSection] = sections![
     table Compact {
         "pcre2"       => { ["PCRE2 regex engine, using the " c:"pcre" " flavor"] }
        "rust"        => { ["Rust regex engine, using the " c:"rust" " flavor"] }
     }
 ];
 
     Use " c:"-h" " for short descriptions and " c:"--help" " for more details."]
 
     "Usage" {
        ["pomsky test [OPTIONS]"]
     }
 
     "Options" {
         table Auto {
            "-p, --path <PATH>" => {
                ["File or directory containing the pomsky expressions to compile"]
                Long ["If a directory is specified, all contained " c:"*.pomsky" " files are tested.
Note that pomsky respects " c:".gitignore" " files."]
             }
             "-f, --flavor <FLAVOR>" => {
                 ["Regex flavor"]
                 ["Regex engine used for testing"]
                 Long ["If " c:"--flavor" " is specified, the engine can be omitted."]
             }
            "    --pass-with-no-tests" => {
                ["Don't error if the specified directory contains no " c:"*.pomsky" " files"]
            }
            "    --allowed-features <FEATURE>..." => {
                ["Comma-separated list of allowed features [default: all enabled]"]
                Long ["Supported features are listed below."]
            }
             "-h, --help" => {
                 ["Print help information"]
                 Long ["Use " c:"-h" " for short descriptions and " c:"--help" " for more details."]
             }
             "-W, --warnings <DIAGNOSTICS>" => WARNINGS
             "    --json" => {
                 Long ["Report test results as JSON"]