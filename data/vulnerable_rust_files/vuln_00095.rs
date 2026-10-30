// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0073
// CVE: CVE-2020-35916
// CWE: N/A
// Crate: image
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/image
// Extraction Method: repo_search
// GitHub Repo: image-rs/image
// Commit: a983704
// File: src/imageops/sample.rs


);
         let image = crate::open(path).unwrap();
         b.iter(|| {
            image.clone();
            test::black_box(image.resize(image.width(), image.height(), FilterType::CatmullRom));
         });
         b.bytes = u64::from(image.width() * image.height() * 3);
     }