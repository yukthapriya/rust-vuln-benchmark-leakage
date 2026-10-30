// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0135
// CVE: CVE-2020-36455
// CWE: N/A
// Crate: slock
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/slock
// Extraction Method: repo_search
// GitHub Repo: BrokenLamp/slock-rs
// Commit: 719df35
// File: src/lib.rs


//!     println!("{}", value); // 6
 //!
 //!     // Use in multiple threads
//!     futures::join!(
 //!         do_something_in_a_thread(lock.split()),
 //!         do_something_else_in_a_thread(lock.split()),
 //!         do_another_thing_in_a_thread(lock.split()),
 //! ### Don't access a Slock from within another
 //!
 //! Bad:
//! ```rust
 //! # use slock::*;
 //! # use futures::executor::block_on;
 //! # async {
 //! # };
 //! ```
 
pub use async_std::future::{timeout, TimeoutError};
use std::{
    cmp::Eq,
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
 };
 
 pub struct SlockData<T> {
     /// let name = lock.map(|v| v.name).await;
     /// # };
     /// ```
    pub async fn map<F, U>(&self, mapper: F) -> Result<U, TimeoutError>
     where
         F: FnOnce(&T) -> U,
     {
        match self.lock.read() {
            Ok(v) => {
                timeout(std::time::Duration::from_secs(1), async {
                    mapper(&v.value)
                })
                .await
            }
            Err(_) => panic!("Slock could not read for map!"),
        }
     }
 
     /// A setter for changing the internal data of the lock.
     /// ```rust
     /// # use slock::*;
    /// # let lock = Slock::new(1i32);
     /// # async {
     /// lock.set(|v| v + 1).await;
     /// lock.set(|_| 6).await;
     where
         F: FnOnce(T) -> T,
     {
        match self.lock.write() {
            Ok(mut data) => {
                let ptr = &mut data.value as *mut T;
                unsafe {
                    let new = timeout(std::time::Duration::from_secs(1), async {
                        setter(ptr.read())
                    })
                    .await;
                    if let Ok(new) = new {
                        timeout(std::time::Duration::from_secs(1), async {
                            data.hook.as_mut().map(|hook| hook(&new));
                        })
                        .await
                        .ok();
                        ptr.write(new);
                    }
                }
                data.version += 1;
            }
            Err(_) => panic!("Slock could not write!"),
         }
     }
 
     /// Create's a new lock pointing to the same data.
     /// let lock = Slock::new(0i32);
     /// let the_same_lock = lock.split();
     /// ```
     pub fn split(&self) -> Self {
         Self {
             lock: self.lock.clone(),
         self.lock.clone()
     }
 
    pub fn hook<F: 'static>(&self, hook: F)
     where
         F: FnMut(&T),
     {
        match self.lock.write() {
            Ok(mut data) => {
                data.hook = Some(Box::new(hook));
            }
            Err(_) => panic!("Slock could not write!"),
         }
     }
 }
 
 impl<T: Clone> Slock<T> {
     /// Returns a clone of the lock's data.
     pub async fn get_clone(&self) -> T {
        match self.lock.read() {
            Ok(v) => v.value.clone(),
            Err(_) => panic!("Slock could not read for clone!"),
        }
     }
 
    /// Creates a clone of the lock and its data.
    pub async fn clone_async(&self) -> Self {
         return Slock::new(self.get_clone().await);
     }
 }
 impl<T> Slock<Slock<T>> {
     /// Converts from `Slock<Slock<T>>` to `Slock<T>`
     pub async fn flatten(&self) -> Slock<T> {
        self.map(|inner| inner.split()).await.unwrap()
     }
 }
 
     pub async fn from_key(&self, key: K) -> Option<Slock<V>> {
         self.map(|hash_map| {
             let key = key;
            hash_map.get(&key).map(|inner| inner.split())
         })
         .await
         .unwrap()
 impl<T: Copy> Slock<T> {
     /// If a lock's data implements copy, this will return an owned copy of it.
     pub async fn get(&self) -> T {
        match self.lock.read() {
            Ok(v) => v.value,
            Err(_) => panic!("Slock could not read for clone!"),
        }
     }
 }
 
 pub mod blocking;
 
unsafe impl<T> Send for Slock<T> {}
unsafe impl<T> Sync for Slock<T> {}