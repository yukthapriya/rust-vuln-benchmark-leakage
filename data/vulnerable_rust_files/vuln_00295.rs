// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0057
// CVE: N/A
// CWE: N/A
// Crate: badge
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/badge
// Extraction Method: repo_search
// GitHub Repo: rust-lang/docs.rs
// Commit: b4e619d
// File: src/utils/consistency/db.rs


releases.id IS NULL
              )
          ) AS inp
         ORDER BY name, version"#,
         config.build_attempts as i32,
     )
     .fetch_all(conn)
     let mut crates = Crates::new();
 
     for (crate_name, release_rows) in &rows.iter().chunk_by(|row| row.name.clone()) {
        let releases: Releases = release_rows
             .map(|row| Release {
                 version: row.version.clone(),
                 yanked: row.yanked,
             })
             .collect();
 
         crates.push(Crate {
             name: crate_name,
             releases,
 mod tests {
     use super::*;
     use crate::test::{V1, V2, V3, async_wrapper};
 
     #[test]
     fn test_load() {
                 .create()
                 .await?;
 
             let mut conn = env.async_db().async_conn().await;
             let result = load(&mut conn, env.config()).await?;
 
                     Crate {
                         name: "krate".into(),
                         releases: vec![
                             Release {
                                 version: V2,
                                 yanked: Some(false),