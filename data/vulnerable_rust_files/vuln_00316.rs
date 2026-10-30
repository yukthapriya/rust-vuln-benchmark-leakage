// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0088
// CVE: CVE-2022-39215
// CWE: N/A
// Crate: tauri
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tauri
// Extraction Method: repo_search
// GitHub Repo: tauri-apps/tauri
// Commit: 79a7d9e
// File: crates/tauri-cli/src/info/plugins.rs


crate_version(tauri_dir, manifest.as_ref(), lock.as_ref(), crate_name).version?;
       let crate_version = semver::Version::parse(&crate_version)
         .inspect_err(|_| {
          log::error!("Failed to parse version `{crate_version}` for crate `{crate_name}`");
         })
         .ok()?;
       Some((crate_name.clone(), crate_version))