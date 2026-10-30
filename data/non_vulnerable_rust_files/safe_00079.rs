// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0044 (FIXED)
// Crate: atom
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: slide-rs/atom
// Commit: bd18124
// File: src/lib.rs


/// Transforms lifetime of the second pointer to match the first.
 #[inline]
 unsafe fn copy_lifetime<'a, S: ?Sized, T: ?Sized + 'a>(_ptr: &'a S, ptr: &T) -> &'a T {
    &*(ptr as *const T)
 }
 
 /// Transforms lifetime of the second pointer to match the first.
 #[inline]
#[allow(unknown_lints, mut_from_ref)]
 unsafe fn copy_mut_lifetime<'a, S: ?Sized, T: ?Sized + 'a>(_ptr: &'a S, ptr: &mut T) -> &'a mut T {
    &mut *(ptr as *mut T)
 }
 
 /// This is a restricted version of the Atom. It allows for only
     P: IntoRawPtr + FromRawPtr + Deref<Target = T>,
 {
     /// If the Atom is set, get the value
    pub fn get(&self, order: Ordering) -> Option<&T> {
         let ptr = self.inner.inner.load(order);
         let val = unsafe { Atom::inner_from_raw(ptr) };
         val.map(|v: P| {
 
 impl<T> AtomSetOnce<Box<T>> {
     /// If the Atom is set, get the value
    pub fn get_mut(&mut self, order: Ordering) -> Option<&mut T> {
         let ptr = self.inner.inner.load(order);
         let val = unsafe { Atom::inner_from_raw(ptr) };
         val.map(move |mut v: Box<T>| {
     T: Clone + IntoRawPtr + FromRawPtr,
 {
     /// Duplicate the inner pointer if it is set
    pub fn dup(&self, order: Ordering) -> Option<T> {
         let ptr = self.inner.inner.load(order);
         let val = unsafe { Atom::inner_from_raw(ptr) };
         val.map(|v: T| {