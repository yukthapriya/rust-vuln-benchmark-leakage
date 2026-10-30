// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0047 (FIXED)
// Crate: array-queue
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: raviqqe/array-queue
// Commit: d8931b7
// File: src/array_queue.rs


if self.is_empty() {
             None
         } else {
            let x = replace(&mut self.array[self.index(0)], MaybeUninit::uninit());
             self.start = self.index(1);
             self.length -= 1;
 
         if self.is_empty() {
             None
         } else {
            let x = replace(
                &mut self.array[self.index(self.length - 1)],
                MaybeUninit::uninit(),
            );
             self.length -= 1;
 
             // SAFETY: An element exists at the last index.
 
         assert_eq!(unsafe { BAR_SUM }, 32);
     }

    #[test]
    fn pop_back_from_middle() {
        const LENGTH: usize = 42;
        const MIDDLE: usize = 17;

        let mut xs = ArrayQueue::<Box<usize>, LENGTH>::new();

        for x in 0..LENGTH {
            assert!(xs.push_back(Box::new(x)).is_ok());
        }

        for _ in 0..MIDDLE {
            xs.pop_front();
        }

        for x in LENGTH..MIDDLE {
            assert_eq!(xs.pop_back(), Some(Box::new(x - 1)));
        }
    }
 }