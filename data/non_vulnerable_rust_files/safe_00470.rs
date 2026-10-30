// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0434 (FIXED)
// Crate: matrix-sdk-crypto
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: matrix-org/matrix-rust-sdk
// Commit: eed5f11
// File: crates/matrix-sdk-common/src/linked_chunk/updates.rs


Clear,
 }
 
impl<Item, Gap> Update<Item, Gap> {
    /// Get the items from the [`Update`] if any.
    ///
    /// This function is useful if you only care about the items from the
    /// [`Update`] and not what kind of update it was and where the items
    /// should be placed.
    ///
    /// [`Update`] variants which don't contain any items will return an empty
    /// [`Vec`].
    pub fn into_items(self) -> Vec<Item> {
        match self {
            Update::NewItemsChunk { .. }
            | Update::NewGapChunk { .. }
            | Update::RemoveChunk(_)
            | Update::RemoveItem { .. }
            | Update::DetachLastItems { .. }
            | Update::StartReattachItems
            | Update::EndReattachItems
            | Update::Clear => vec![],
            Update::PushItems { items, .. } => items,
            Update::ReplaceItem { item, .. } => vec![item],
        }
    }
}

 /// A collection of [`Update`]s that can be observed.
 ///
 /// Get a value for this type with [`LinkedChunk::updates`].
     use futures_util::pin_mut;
 
     use super::{super::LinkedChunk, ChunkIdentifier, Position, UpdatesInner};
    use crate::linked_chunk::Update;
 
     #[test]
     fn test_updates_take_and_garbage_collector() {
         drop(linked_chunk);
         assert_matches!(updates_subscriber1.as_mut().poll_next(&mut context1), Poll::Ready(None));
     }

    #[test]
    fn test_update_into_items() {
        let updates: Update<_, u32> =
            Update::PushItems { at: Position::new(ChunkIdentifier(0), 0), items: vec![1, 2, 3] };

        assert_eq!(updates.into_items(), vec![1, 2, 3]);

        let updates: Update<u32, u32> = Update::Clear;
        assert!(updates.into_items().is_empty());

        let updates: Update<u32, u32> =
            Update::RemoveItem { at: Position::new(ChunkIdentifier(0), 0) };
        assert!(updates.into_items().is_empty());

        let updates: Update<u32, u32> =
            Update::ReplaceItem { at: Position::new(ChunkIdentifier(0), 0), item: 42 };
        assert_eq!(updates.into_items(), vec![42]);
    }
 }