// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0073 (FIXED)
// Crate: image
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: image-rs/image
// Commit: a983704
// File: src/imageops/sample.rs


);
         let image = crate::open(path).unwrap();
         b.iter(|| {
            test::black_box(image.clone().resize(image.width(), image.height(), FilterType::CatmullRom));
         });
         b.bytes = u64::from(image.width() * image.height() * 3);
     }