// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0374
// CVE: N/A
// CWE: N/A
// Crate: ouch
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/ouch
// Extraction Method: repo_search
// GitHub Repo: ouch-org/ouch
// Commit: 96cbdb4
// File: src/archive/rar.rs


/// Unpacks the archive given by `archive_path` into the folder given by `output_folder`.
 /// Assumes that output_folder is empty
pub fn unpack_archive(
    archive_path: &Path,
    output_folder: &Path,
    password: Option<&[u8]>,
    quiet: bool,
) -> crate::Result<Unpacked> {
     let archive = match password {
         Some(password) => Archive::with_password(archive_path, password),
         None => Archive::new(archive_path),
     while let Some(header) = archive.read_header()? {
         let entry = header.entry();
         archive = if entry.is_file() {
            if !quiet {
                info!(
                    "extracted ({}) {}",
                    Bytes::new(entry.unpacked_size),
                    entry.filename.display(),
                );
            }
             unpacked += 1;
             header.extract_with_base(output_folder)?
         } else {