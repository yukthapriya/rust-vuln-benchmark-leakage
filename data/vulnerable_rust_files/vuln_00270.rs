// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0007
// CVE: N/A
// CWE: N/A
// Crate: qcell
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/qcell
// Extraction Method: repo_search
// GitHub Repo: uazu/qcell
// Commit: a37533f
// File: src/assertions.rs


use crate::{LCell, LCellOwner, QCell, QCellOwnerPinned};
 
#[cfg(features = "alloc")]
 use crate::QCellOwner;
 
 #[cfg(feature = "std")]
 
 // Check owners
 assert_impl_all!(LCellOwner<'_>: Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
#[cfg(features = "alloc")]
 assert_impl_all!(QCellOwner: Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
 assert_impl_all!(QCellOwnerPinned: Send, Sync, UnwindSafe, RefUnwindSafe);
 assert_not_impl_any!(QCellOwnerPinned: Unpin);
 assert_not_impl_any!(TLCell<Q, Cell<i32>>: Sync);
 
 // Check cells for a !Send Sync type
 struct Test(*const i32);
 unsafe impl Sync for Test {}
 assert_impl_all!(LCell<'_, Test>: Unpin, UnwindSafe);