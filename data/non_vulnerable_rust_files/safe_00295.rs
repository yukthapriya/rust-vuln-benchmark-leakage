// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0057 (FIXED)
// Crate: badge
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang/docs.rs
// Commit: b4e619d
// File: src/utils/consistency/db.rs


releases.id IS NULL
              )
          ) AS inp
         ORDER BY name"#,
         config.build_attempts as i32,
     )
     .fetch_all(conn)
     let mut crates = Crates::new();
 
     for (crate_name, release_rows) in &rows.iter().chunk_by(|row| row.name.clone()) {
        let mut releases: Releases = release_rows
             .map(|row| Release {
                 version: row.version.clone(),
                 yanked: row.yanked,
             })
             .collect();
 
        releases.sort_by(|lhs, rhs| lhs.version.cmp(&rhs.version));

         crates.push(Crate {
             name: crate_name,
             releases,
 mod tests {
     use super::*;
     use crate::test::{V1, V2, V3, async_wrapper};
    use pretty_assertions::assert_eq;
 
     #[test]
     fn test_load() {
                 .create()
                 .await?;
 
            // these two releases are there to ensure we sort correctly.
            // In the past, we sorted the version (from the crates index & our database)
            // as string, which lead to "0.10.3" coming before "0.9.3".
            // When both sides are sorted the same way, this is fine and doesn't break the
            // consistency check.
            // But after migrating everything to using `semver::Version`, the sorting changed
            // on the index-side, while we still sorted by string on the database side.
            //
            // Since I still run the consistency check manually, every now and then, this wasn't
            // an issue, because I saw the odd huge difference.
            //
            // The solution is to sort both sides semver correctly.
            const V0_9_3: Version = Version::new(0, 9, 3);
            const V0_10_3: Version = Version::new(0, 10, 3);
            env.fake_release()
                .await
                .name("krate")
                .version(V0_9_3)
                .yanked(false)
                .create()
                .await?;
            env.fake_release()
                .await
                .name("krate")
                .version(V0_10_3)
                .yanked(false)
                .create()
                .await?;

             let mut conn = env.async_db().async_conn().await;
             let result = load(&mut conn, env.config()).await?;
 
                     Crate {
                         name: "krate".into(),
                         releases: vec![
                            Release {
                                version: V0_9_3,
                                yanked: Some(false),
                            },
                            Release {
                                version: V0_10_3,
                                yanked: Some(false),
                            },
                             Release {
                                 version: V2,
                                 yanked: Some(false),