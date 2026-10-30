// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0105 (FIXED)
// Crate: abi_stable
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rodrimati1992/abi_stable_crates
// Commit: 072723a
// File: abi_stable/src/erased_types/dyn_trait.rs


/// };
         ///
         /// let to: DynTrait<'static, RBox<()>, ()> =
        ///     DynTrait::from_value(RString::from("foobarbaz"));
         ///
        /// let string = to.sabi_with_value(|x| {
        ///     // SAFETY: the erased object is an RString constructed in the current binary.
        ///     unsafe {
        ///         MovePtr::into_inner(MovePtr::transmute::<RString>(x))
        ///     }
         /// });
         ///
         /// assert_eq!(string, "foobarbaz");