// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0092 (FIXED)
// Crate: concread
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: kanidm/concread
// Commit: 8394e6e
// File: src/internals/lincowcell/mod.rs


}
 }
 
impl<R> Drop for LinCowCellInner<R> {
    fn drop(&mut self) {
        // Ensure the default drop won't recursively drop the chain
        let mut current = self.pin.lock().unwrap().take();

        // Drop the chain iteratively to avoid stack overflow
        while let Some(arc) = current {
            // Try to get exclusive ownership of the next link
            match Arc::try_unwrap(arc) {
                Ok(inner) => {
                    // Continue with the next link.
                    current = inner.pin.lock().unwrap().take();
                }
                Err(_) => {
                    // Another reference exists, so we can safely let it drop normally without recursion
                    break;
                }
            }
        }
    }
}

 impl<T, R, U> LinCowCell<T, R, U>
 where
     T: LinCowCellCapable<R, U>,
 
         assert!(GC_COUNT.load(Ordering::Acquire) >= 50);
     }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_long_chain_drop_no_stack_overflow() {
        let data = TestData { x: 0 };
        let cc = LinCowCell::new(data);

        // Simulate a read txn that is not dropped.
        let initial_read = cc.read();

        // Create a long chain of versions by committing many writes.
        for i in 0..10000 {
            let mut write_txn = cc.write();
            write_txn.get_mut().x = i;
            write_txn.commit();
        }

        drop(initial_read);

        // Verify the final state is correct.
        let final_read = cc.read();
        assert_eq!(final_read.work.data.x, 9999);
    }
 }
 
 #[cfg(test)]