// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0033 (FIXED)
// Crate: borsh
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: near/borsh-rs
// Commit: e01e16d
// File: borsh/src/de/mod.rs


}
 }
 
#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L39
// license: MIT OR Apache-2.0
impl<K, V, S> BorshDeserialize for indexmap::IndexMap<K, V, S>
where
    K: BorshDeserialize + Eq + core::hash::Hash,
    V: BorshDeserialize,
    S: core::hash::BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<K>()?;
        let vec = <Vec<(K, V)>>::deserialize_reader(reader)?;
        Ok(vec.into_iter().collect::<indexmap::IndexMap<K, V, S>>())
    }
}

#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L75
// license: MIT OR Apache-2.0
impl<T, S> BorshDeserialize for indexmap::IndexSet<T, S>
where
    T: BorshDeserialize + Eq + core::hash::Hash,
    S: core::hash::BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<T>()?;
        let vec = <Vec<T>>::deserialize_reader(reader)?;
        Ok(vec.into_iter().collect::<indexmap::IndexSet<T, S>>())
    }
}

 impl<T> BorshDeserialize for Cow<'_, T>
 where
     T: ToOwned + ?Sized,