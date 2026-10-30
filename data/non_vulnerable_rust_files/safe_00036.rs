// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0015 (FIXED)
// Crate: compact_arena
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: llogiq/compact_arena
// Commit: d01ed1c
// File: tests/tiny_arena.rs


#[test]
 fn two_tiny_arenas() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(|| {
        assert_eq!(3, in_tiny_arena!(a, {
            in_tiny_arena!(b, {
                let x = a.add(1usize);
                let y = b.add(2usize);
                a[x] + b[y]
            })
        }))
    }).unwrap().join().unwrap();
 }
 
 #[test]
 fn two_tiny_arenas_one_scope() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(|| {
        mk_tiny_arena!(a);
        mk_tiny_arena!(b);
        let x = a.add(1usize);
        let y = b.add(2usize);
        assert_eq!(3, a[x] + b[y])
    }).unwrap().join().unwrap();
 }