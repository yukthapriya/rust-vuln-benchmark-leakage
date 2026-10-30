// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0133 (FIXED)
// Crate: scottqueue
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rossdylan/rust-scottqueue
// Commit: 4c8a90e
// File: src/tlqueue.rs


drop(tx);
             });
         }
        for _ in 0..nmsgs {
             let msg = rx.recv().unwrap();
             assert!(!end_set.contains(&msg));
             end_set.insert(msg);
     #[bench]
     fn pop_bench(b: &mut Bencher) {
         let q = super::Queue::new();
        for i in 0..100000 {
             q.push(i);
         }
         b.iter(|| q.pop());
     }

    #[bench]
    fn channel_send_bench(b: &mut Bencher) {
        let (tx, rx) = channel();
        b.iter(|| tx.send(0).unwrap());
        drop(tx);
        drop(rx);
    }

    #[bench]
    fn channel_recv_bench(b: &mut Bencher) {
        let (tx, rx) = channel();
        for i in 0..100000 {
            tx.send(i).unwrap();
        }
        b.iter(|| rx.recv().unwrap());
        drop(tx);
        drop(rx);
    }

 }