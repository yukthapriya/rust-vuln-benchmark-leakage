// REAL FIXED CODE
// RustSec ID: RUSTSEC-2018-0015 (FIXED)
// Crate: term
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Stebalien/term
// Commit: 31f7072
// File: src/terminfo/searcher.rs


pub fn get_dbpath_for_term(term: &str) -> Option<PathBuf> {
     let mut dirs_to_search = Vec::new();
     let mut default_locations = DEFAULT_LOCATIONS.iter().map(PathBuf::from);
    let first_char = term.chars().next()?;
 
     // From the manual.
     //