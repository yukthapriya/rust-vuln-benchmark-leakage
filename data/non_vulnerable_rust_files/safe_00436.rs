// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0374 (FIXED)
// Crate: ouch
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ouch-org/ouch
// Commit: 96cbdb4
// File: src/archive/rar.rs


/// Unpacks the archive given by `archive_path` into the folder given by `output_folder`.
 /// Assumes that output_folder is empty
pub fn unpack_archive(archive_path: &Path, output_folder: &Path, password: Option<&[u8]>) -> crate::Result<Unpacked> {
     let archive = match password {
         Some(password) => Archive::with_password(archive_path, password),
         None => Archive::new(archive_path),
     while let Some(header) = archive.read_header()? {
         let entry = header.entry();
         archive = if entry.is_file() {
            info!(
                "extracted ({}) {}",
                Bytes::new(entry.unpacked_size),
                entry.filename.display(),
            );
             unpacked += 1;
             header.extract_with_base(output_folder)?
         } else {