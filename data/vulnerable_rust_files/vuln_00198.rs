// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0052
// CVE: CVE-2021-30455
// CWE: N/A
// Crate: id-map
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/id-map
// Extraction Method: repo_search
// GitHub Repo: andrewhickman/id-map
// Commit: fab6922
// File: src/lib.rs


//! [`IdMap`] is a container that gives each item a unique id. Adding and removing by index is O(1).
//! 
 //! # Examples
//! 
 //! ```
 //! # use id_map::IdMap;
 //! #
 //! assert!(!map.contains(red_id));
 //! assert_eq!(map[blue_id], "blue");
 //! ```
//! 
 //! [`IdMap`]: struct.IdMap.html
 
#![deny(missing_docs, missing_debug_implementations)]
 
 extern crate id_set;
 
 
 use std::iter::FromIterator;
 use std::ops::{Index, IndexMut};
use std::{cmp, fmt, mem, ptr};
 
 use id_set::IdSet;
 
 /// A container that gives each item a unique id. Internally all elements are stored contiguously.
 pub struct IdMap<T> {
     // The set of valid indices for values.
     ids: IdSet,
     // The buffer of values. Indices not in ids are invalid.
    values: Vec<T>,
    // The smallest empty space in the vector of values, or values.capacity() if no space is left.
     space: Id,
 }
 
     #[inline]
     /// Removes all values from the map.
     pub fn clear(&mut self) {
        unsafe { self.drop_values() }
         self.ids.clear();
     }
 
     /// Resizes the map to minimize allocated memory.
     pub fn shrink_to_fit(&mut self) {
         self.ids.shrink_to_fit();
        unsafe {
            let cap = cmp::min(self.values.capacity(), self.ids.capacity());
            self.values.set_len(cap);
            self.values.shrink_to_fit();
            self.values.set_len(0);
        }
     }
 
     #[inline]
     /// Inserts a value into an empty slot in the map and returns its id.
     pub fn insert(&mut self, val: T) -> Id {
         let id = self.space;
        if id == self.values.capacity() {
            self.values.reserve(id + 1);
         }
        unsafe { ptr::write(self.values.get_unchecked_mut(id), val) }
         self.ids.insert(id);
         self.find_space();
         id
             if id == self.space {
                 self.find_space();
             }
            if self.values.capacity() < id + 1 {
                self.values.reserve(id + 1);
             }
            unsafe { ptr::write(self.values.get_unchecked_mut(id), val) }
             None
         } else {
             // val was previously in the map
            Some(mem::replace(unsafe { self.values.get_unchecked_mut(id) }, val))
         }
     }
 
     pub fn remove(&mut self, id: Id) -> Option<T> {
         if self.ids.remove(id) {
             self.space = cmp::min(self.space, id);
            Some(unsafe { ptr::read(self.values.get_unchecked(id)) })
         } else {
             None
         }
     pub fn get_or_insert(&mut self, id: Id, val: T) -> &mut T {
         self.get_or_insert_with(id, || val)
     }
    
     #[inline]
     /// If the id has a value, returns it, otherwise inserts a new value with the provided closure.
     pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, id: Id, f: F) -> &mut T {
             if id == self.space {
                 self.find_space();
             }
            if self.values.capacity() < id + 1 {
                self.values.reserve(id + 1);
             }
            unsafe {
                let space = self.values.get_unchecked_mut(id);
                ptr::write(space, f());
                space
            }
        } else {
            unsafe { self.values.get_unchecked_mut(id) }
         }
     }
 
     #[inline]
             if let Some(first) = iter.next() {
                 // Set iterators are increasing so we only need to change start once.
                 self.space = cmp::min(self.space, first);
                unsafe {
                    ptr::drop_in_place(self.values.get_unchecked_mut(first))
                }
                 for id in iter {
                    unsafe {
                        ptr::drop_in_place(self.values.get_unchecked_mut(id))
                    }
                 }
             }
         }
         let values = &mut self.values;
         let space = &mut self.space;
         ids.retain(|id| {
            unsafe {
                if pred(id, values.get_unchecked(id)) {
                    true
                } else {
                    *space = cmp::min(*space, id);
                    ptr::drop_in_place(values.get_unchecked_mut(id));
                    false
                }
             }
         })
     }
     /// Returns a reference to the value at the specified id if it is in the map.
     pub fn get(&self, id: Id) -> Option<&T> {
         if self.ids.contains(id) {
            Some(unsafe { self.values.get_unchecked(id) })
         } else {
             None
         }
     /// Returns a mutable reference to the value at the specified id if it is in the map.
     pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
         if self.ids.contains(id) {
            Some(unsafe { self.values.get_unchecked_mut(id) })
         } else {
             None
         }
     pub fn values(&self) -> Values<T> {
         Values {
             ids: self.ids.iter(),
            values: self.values.as_ptr(),
         }
     }
 
     pub fn values_mut(&mut self) -> ValuesMut<T> {
         ValuesMut {
             ids: self.ids.iter(),
            values: self.values.as_mut_ptr(),
         }
     }
 
     pub fn iter(&self) -> Iter<T> {
         Iter {
             ids: self.ids.iter(),
            values: self.values.as_ptr(),
         }
     }
 
     pub fn iter_mut(&mut self) -> IterMut<T> {
         IterMut {
             ids: self.ids.iter(),
            values: self.values.as_mut_ptr(),
         }
     }
 
     #[inline]
     /// A consuming iterator over id-value pairs, in order of increasing id.
     pub fn into_iter(self) -> IntoIter<T> {
        // we cannot move out of self because of the drop impl.
        let (ids, values) = unsafe {
            (ptr::read(&self.ids), ptr::read(&self.values))
        };
        mem::forget(self);
         IntoIter {
            ids: ids.into_iter(),
            values: values,
         }
     }
 
             assert!(self.ids.contains(id));
         }
         assert!(!self.ids.contains(self.space));
        // values.capacity() should an upper bound on ids.
         for id in &self.ids {
            assert!(id < self.values.capacity())
         }
     }
 
    /// Clear the values vec. Unsafe since ids is not updated.
    unsafe fn drop_values(&mut self) {
         for id in &self.ids {
            ptr::drop_in_place(self.values.get_unchecked_mut(id))
         }
     }
 
    /// Find the next empty space after has been filled.
     fn find_space(&mut self) {
         // Each id corresponds to an entry in the storage so ids can never fill up.
         self.space += 1;
     }
 }
 
impl<T> Drop for IdMap<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.drop_values() }
    }
}

impl<T: Clone> Clone for IdMap<T> {
    #[inline]
    fn clone(&self) -> Self {
        let ids = self.ids.clone();
        let cap = self.values.capacity();
        let mut values = Vec::with_capacity(cap);
        unsafe {
            for id in &ids {
                ptr::write(values.get_unchecked_mut(id),
                           self.values.get_unchecked(id).clone());
            }
        }
        IdMap {
            ids,
            values,
            space: cap,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        unsafe { self.drop_values() };
        self.ids.clone_from(&other.ids);

        let cap = other.values.capacity();
        self.values.reserve(cap);
        unsafe {
            for id in &self.ids {
                ptr::write(self.values.get_unchecked_mut(id),
                           other.values.get_unchecked(id).clone());
            }
        }
    }
}

 impl<T: fmt::Debug> fmt::Debug for IdMap<T> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{{")?;
 
 impl<T: PartialEq> PartialEq for IdMap<T> {
     fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids && self.ids
            .iter()
            .zip(&other.ids)
            .all(|(l, r)| unsafe { self.values.get_unchecked(l) == other.values.get_unchecked(r) })
     }
 }
 
 impl<T> FromIterator<T> for IdMap<T> {
     #[inline]
     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut values = Vec::from_iter(iter);
        unsafe { values.set_len(0) }
        let space = values.capacity();
        let ids = IdSet::new_filled(values.capacity());
        IdMap {
            values,
            space,
            ids,
        }
     }
 }
 
     #[inline]
     fn index(&self, id: Id) -> &Self::Output {
         assert!(self.ids.contains(id), "id {} out of bounds", id);
        unsafe { self.values.get_unchecked(id) }
     }
 }
 
 impl<T> IndexMut<Id> for IdMap<T> {
     #[inline]
     fn index_mut(&mut self, id: Id) -> &mut Self::Output {
         assert!(self.ids.contains(id), "id {} out of bounds", id);
        unsafe { self.values.get_unchecked_mut(id) }
     }
 }
 
 /// An iterator over all values, in order of increasing id.
 pub struct Values<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: *const T,
 }
 
 impl<'a, T: 'a> Iterator for Values<'a, T> {
     type Item = &'a T;
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| unsafe { &*self.values.offset(id as isize) })
     }
 
     #[inline]
 /// A mutable iterator over all values, in order of increasing id.
 pub struct ValuesMut<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: *mut T,
 }
 
 impl<'a, T: 'a> Iterator for ValuesMut<'a, T> {
     type Item = &'a mut T;
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| unsafe { &mut *self.values.offset(id as isize) })
     }
 
     #[inline]
 /// An iterator over id-value pairs, in order of increasing id.
 pub struct Iter<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: *const T,
 }
 
 impl<'a, T: 'a> Iterator for Iter<'a, T> {
     type Item = (Id, &'a T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| (id, unsafe { &*self.values.offset(id as isize) }))
     }
 
     #[inline]
 /// A mutable iterator over id-value pairs, in order of increasing id.
 pub struct IterMut<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: *mut T,
 }
 
 impl<'a, T: 'a> Iterator for IterMut<'a, T> {
     type Item = (Id, &'a mut T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| (id, unsafe { &mut *self.values.offset(id as isize) }))
     }
 
     #[inline]
 /// A consuming iterator over id-value pairs, in order of increasing id.
 pub struct IntoIter<T> {
     ids: id_set::IntoIter,
    values: Vec<T>,    
 }
 
 impl<T> Iterator for IntoIter<T> {
     type Item = (Id, T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| (id, unsafe { ptr::read(self.values.get_unchecked(id)) }))
     }
 
     #[inline]
     fn size_hint(&self) -> (usize, Option<usize>) {
         self.ids.size_hint()
     }
}