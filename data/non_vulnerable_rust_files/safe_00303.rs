// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0072 (FIXED)
// Crate: hyper-staticfile
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: stephank/hyper-staticfile
// Commit: f12cadc
// File: src/resolve.rs


/// The requested file could not be accessed.
     PermissionDenied,
     /// A directory was requested as a file.
    IsDirectory {
        /// Path to redirect to.
        redirect_to: String,
    },
     /// The requested file was found.
     Found(ResolvedFile<F>),
 }
 
         // We may have opened a directory for a file request, in which case we redirect.
         if !is_dir_request && file.is_dir {
            // Build the redirect path. On Windows, we can't just append the entire path, because
            // it contains Windows path separators. Instead, append each component separately.
            let mut target = String::with_capacity(path.as_os_str().len() + 2);
            target.push('/');
            for component in path.components() {
                target.push_str(&component.as_os_str().to_string_lossy());
                target.push('/');
            }

            return Ok(ResolveResult::IsDirectory {
                redirect_to: target,
            });
         }
 
         // If not a directory, serve this file.