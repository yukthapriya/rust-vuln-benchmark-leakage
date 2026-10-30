// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0088 (FIXED)
// Crate: tauri
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tauri-apps/tauri
// Commit: 79a7d9e
// File: crates/tauri-cli/src/info/plugins.rs


crate_version(tauri_dir, manifest.as_ref(), lock.as_ref(), crate_name).version?;
       let crate_version = semver::Version::parse(&crate_version)
         .inspect_err(|_| {
          // On first run there's no lockfile yet so we get the version requirement from Cargo.toml.
          // In our templates that's `2` which is not a valid semver version but a version requirement.
          // log::error confused users so we use log::debug to still be able to see this error if needed.
          log::debug!("Failed to parse version `{crate_version}` for crate `{crate_name}`");
         })
         .ok()?;
       Some((crate_name.clone(), crate_version))