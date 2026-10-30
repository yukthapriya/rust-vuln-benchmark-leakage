// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0050
// CVE: CVE-2020-35903
// CWE: N/A
// Crate: dync
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/dync
// Extraction Method: repo_search
// GitHub Repo: elrnv/dync
// Commit: 6c75aff
// File: benches/type_erasure.rs


#[inline]
 fn make_random_vec(n: usize) -> Vec<[i64; 3]> {
     let mut rng: StdRng = SeedableRng::from_seed(SEED);
    (0..n).map(move |_| [rng.gen::<i64>(); 3]).collect()
 }
 
 #[inline]
     let mut rng: StdRng = SeedableRng::from_seed(SEED);
     (0..n)
         .map(move |_| {
            let b: Box<dyn Any> = Box::new([rng.gen::<i64>(); 3]);
             b
         })
         .collect()
     let mut rng: StdRng = SeedableRng::from_seed(SEED);
     (0..n)
         .map(move |_| {
            let b: Arc<dyn Any + Send + Sync> = Arc::new([rng.gen::<i64>(); 3]);
             b
         })
         .collect()