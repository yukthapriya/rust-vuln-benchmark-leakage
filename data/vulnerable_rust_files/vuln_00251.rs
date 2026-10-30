// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0133
// CVE: N/A
// CWE: N/A
// Crate: cargo-download
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cargo-download
// Extraction Method: repo_search
// GitHub Repo: Xion/cargo-download
// Commit: bac5eac
// File: src/args.rs


/// (*all* arguments, including binary name).
 #[inline]
 pub fn parse_from_argv<I, T>(argv: I) -> Result<Options, ArgsError>
    where I: IntoIterator<Item=T>, T: Clone + Into<OsString>
 {
     let parser = create_parser();
     let matches = try!(parser.get_matches_from_safe(argv));
     Options::try_from(matches)