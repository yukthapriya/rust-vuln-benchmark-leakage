// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0105
// CVE: CVE-2020-36212
// CWE: N/A
// Crate: abi_stable
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/abi_stable
// Extraction Method: repo_search
// GitHub Repo: rodrimati1992/abi_stable_crates
// Commit: 072723a
// File: abi_stable/src/erased_types/dyn_trait.rs


/// };
         ///
         /// let to: DynTrait<'static, RBox<()>, ()> =
        ///     DynTrait::from_value(RVec::<u8>::from_slice(b"foobarbaz"));
         ///
        /// let string = to.sabi_with_value(|x| unsafe {
        ///     MovePtr::into_inner(MovePtr::transmute::<String>(x))
         /// });
         ///
         /// assert_eq!(string, "foobarbaz");