// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0032 (FIXED)
// Crate: crust
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: maidsafe/crust
// Commit: 59edb62
// File: examples/secure_p2p_connection.rs


box_pointers,
     missing_copy_implementations,
     missing_debug_implementations,
    variant_size_differences,

    // FIXME: `needless_pass_by_value` and `clone_on_ref_ptr` required to make no intrusive changes
    // on code in the master branch
    clippy::clone_on_ref_ptr,
    clippy::needless_pass_by_value
 )]
 
 #[macro_use]
             Arg::with_name("test-extreach")
                 .short("t")
                 .long("test-extreach")
                .help("Enables the External Reachability Test."),
         )
         .get_matches();