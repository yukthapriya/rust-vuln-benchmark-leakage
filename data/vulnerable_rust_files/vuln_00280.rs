// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0030
// CVE: CVE-2022-31099
// CWE: N/A
// Crate: rulex
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/rulex
// Extraction Method: repo_search
// GitHub Repo: rulex-rs/rulex
// Commit: 1dc87c2
// File: pomsky-bin/src/args/help.rs


const ENGINES: &[HelpSection] = sections![
     table Compact {
         "pcre2"       => { ["PCRE2 regex engine, using the " c:"pcre" " flavor"] }
     }
 ];
 
     Use " c:"-h" " for short descriptions and " c:"--help" " for more details."]
 
     "Usage" {
        ["pomsky test [OPTIONS] --path <PATH>"]
    }

    "Args" {
        table Auto {
            "<INPUT>" => {
                ["Pomsky expression to test"]
                Long ["\n\
                To learn about the pomsky language, start here:\n\
                https://pomsky-lang.org/docs/"]
            }
        }
     }
 
     "Options" {
         table Auto {
            "    --allowed-features <FEATURE>..." => {
                ["Comma-separated list of allowed features [default: all enabled]"]
                Long ["Supported features are listed below."]
             }
             "-f, --flavor <FLAVOR>" => {
                 ["Regex flavor"]
                 ["Regex engine used for testing"]
                 Long ["If " c:"--flavor" " is specified, the engine can be omitted."]
             }
             "-h, --help" => {
                 ["Print help information"]
                 Long ["Use " c:"-h" " for short descriptions and " c:"--help" " for more details."]
             }
            "-p, --path <PATH>" => {
                ["File or directory containing the pomsky expressions to compile"]
                Long ["If a directory is specified, all contained " c:"*.pomsky" " files are tested.
Note that pomsky respects " c:".gitignore" " files."]
            }
            "    --pass-with-no-tests" => {
                ["Don't error if the specified directory contains no " c:"*.pomsky" " files"]
            }
             "-W, --warnings <DIAGNOSTICS>" => WARNINGS
             "    --json" => {
                 Long ["Report test results as JSON"]