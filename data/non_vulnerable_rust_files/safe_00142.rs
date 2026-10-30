// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0135 (FIXED)
// Crate: slock
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: BrokenLamp/slock-rs
// Commit: 719df35
// File: src/lib.rs


//!     println!("{}", value); // 6
 //!
 //!     // Use in multiple threads
//!     tokio::join!(
 //!         do_something_in_a_thread(lock.split()),
 //!         do_something_else_in_a_thread(lock.split()),
 //!         do_another_thing_in_a_thread(lock.split()),
 //! ### Don't access a Slock from within another
 //!
 //! Bad:
//! ```rust,ignore
 //! # use slock::*;
 //! # use futures::executor::block_on;
 //! # async {
 //! # };
 //! ```
 
use std::{cmp::Eq, collections::HashMap, hash::Hash, sync::Arc};

use tokio::{
    sync::RwLock,
    time::{error::Elapsed, timeout},
 };
 
 pub struct SlockData<T> {
     /// let name = lock.map(|v| v.name).await;
     /// # };
     /// ```
    pub async fn map<F, U>(&self, mapper: F) -> Result<U, Elapsed>
     where
         F: FnOnce(&T) -> U,
     {
        let v = self.lock.read().await;
        timeout(std::time::Duration::from_secs(1), async {
            mapper(&v.value)
        })
        .await
     }
 
     /// A setter for changing the internal data of the lock.
     /// ```rust
     /// # use slock::*;
    /// let lock = Slock::new(1i32);
    ///
     /// # async {
     /// lock.set(|v| v + 1).await;
     /// lock.set(|_| 6).await;
     where
         F: FnOnce(T) -> T,
     {
        let mut data = self.lock.write().await;
        let ptr = &mut data.value as *mut T;
        let new = timeout(std::time::Duration::from_secs(1), async {
            setter(unsafe { ptr.read() })
        })
        .await;
        if let Ok(new) = new {
            timeout(std::time::Duration::from_secs(1), async {
                data.hook.as_mut().map(|hook| hook(&new));
            })
            .await
            .ok();
            unsafe { ptr.write(new) };
         }

        data.version += 1;
     }
 
     /// Create's a new lock pointing to the same data.
     /// let lock = Slock::new(0i32);
     /// let the_same_lock = lock.split();
     /// ```
    #[deprecated = "Use `clone()` instead"]
     pub fn split(&self) -> Self {
         Self {
             lock: self.lock.clone(),
         self.lock.clone()
     }
 
    pub async fn hook<F: 'static>(&self, hook: F)
     where
         F: FnMut(&T),
     {
        let mut data = self.lock.write().await;
        data.hook = Some(Box::new(hook));
    }
}

impl<T> Clone for Slock<T> {
    fn clone(&self) -> Self {
        Self {
            lock: self.lock.clone(),
         }
     }
 }
 
 impl<T: Clone> Slock<T> {
     /// Returns a clone of the lock's data.
     pub async fn get_clone(&self) -> T {
        let data = self.lock.read().await;
        data.value.clone()
     }
 
    /// Create a new lock with data clone from this one.
    pub async fn clone_deep(&self) -> Self {
         return Slock::new(self.get_clone().await);
     }
 }
 impl<T> Slock<Slock<T>> {
     /// Converts from `Slock<Slock<T>>` to `Slock<T>`
     pub async fn flatten(&self) -> Slock<T> {
        self.map(|inner| inner.clone()).await.unwrap()
     }
 }
 
     pub async fn from_key(&self, key: K) -> Option<Slock<V>> {
         self.map(|hash_map| {
             let key = key;
            hash_map.get(&key).map(|inner| inner.clone())
         })
         .await
         .unwrap()
 impl<T: Copy> Slock<T> {
     /// If a lock's data implements copy, this will return an owned copy of it.
     pub async fn get(&self) -> T {
        let data = self.lock.read().await;
        data.value
     }
 }
 
 pub mod blocking;
 
unsafe impl<T: Send> Send for Slock<T> {}
unsafe impl<T: Send> Sync for Slock<T> {}