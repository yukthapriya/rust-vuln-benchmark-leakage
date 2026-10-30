// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0133 (FIXED)
// Crate: cargo-download
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Xion/cargo-download
// Commit: bac5eac
// File: src/args.rs


/// (*all* arguments, including binary name).
 #[inline]
 pub fn parse_from_argv<I, T>(argv: I) -> Result<Options, ArgsError>
    where I: IntoIterator<Item=T>, T: Clone + Into<OsString> + PartialEq<str>
 {
    // Detect `cargo download` invocation, and remove the subcommand name.
    let mut argv: Vec<_> = argv.into_iter().collect();
    if argv.len() >= 2 && &argv[1] == "download" {
        argv.remove(1);
    }

     let parser = create_parser();
     let matches = try!(parser.get_matches_from_safe(argv));
     Options::try_from(matches)