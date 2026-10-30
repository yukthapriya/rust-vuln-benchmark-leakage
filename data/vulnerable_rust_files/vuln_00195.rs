// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0048
// CVE: CVE-2021-29939
// CWE: N/A
// Crate: stackvector
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/stackvector
// Extraction Method: repo_search
// GitHub Repo: Alexhuszagh/rust-stackvector
// Commit: 2f445b4
// File: src/lib.rs


v.push(x);
         }
         v.insert_many(1, BadBoundsIterator2::new());
        assert_eq!(
            &v.iter().map(|v| *v).collect::<Vec<_>>(),
            &[0, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 1, 2, 3]
        );
 
         let mut v: StackVec<[u8; 64]> = StackVec::new();
         for x in 0..4 {
             v.push(x);
         }
         v.insert_many(1, BadSizeHint::new(1));
        assert_eq!(
            &v.iter().map(|v| *v).collect::<Vec<_>>(),
            &[0, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 1, 2, 3]
        );
     }
 
     #[should_panic]