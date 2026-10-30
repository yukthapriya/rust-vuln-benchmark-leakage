// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0094
// CVE: N/A
// CWE: N/A
// Crate: martin-mbtiles
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/martin-mbtiles
// Extraction Method: commit_diff
// GitHub Repo: maplibre/martin
// Commit: 8a8814e
// File: martin-mbtiles/src/copier.rs


#[derive(PartialEq, Eq, Default, Debug, Clone, EnumDisplay)]
 #[enum_display(case = "Kebab")]
 #[cfg_attr(feature = "cli", derive(ValueEnum))]
 pub enum CopyDuplicateMode {
     #[default]
     Override,
 
 #[derive(Clone, Default, PartialEq, Eq, Debug)]
 #[cfg_attr(feature = "cli", derive(Args))]
 pub struct MbtilesCopier {
     /// MBTiles file to read from
     pub src_file: PathBuf,
 
 impl MbtilesCopier {
     #[must_use]
     pub fn new(src_filepath: PathBuf, dst_filepath: PathBuf) -> Self {
         Self {
             src_file: src_filepath,