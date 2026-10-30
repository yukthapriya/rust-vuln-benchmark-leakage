// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0072 (FIXED)
// Crate: futures-intrusive
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Matthias247/futures-intrusive
// Commit: 842c2b1
// File: src/intrusive_double_linked_list.rs


}
     }
 
    /// Returns a reference to the first node in the linked list
     /// The function is only safe as long as valid pointers are stored inside
     /// the linked list.
     /// The returned pointer is only guaranteed to be valid as long as the list
     /// is not mutated
    pub fn peek_first(&self) -> Option<&ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.head
                .map(|node| &*(node.as_ptr() as *const ListNode<T>))
        }
    }

    /// Returns a mutable reference to the first node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_first_mut(&mut self) -> Option<&mut ListNode<T>> {
         // Safety: When the node was inserted it was promised that it is alive
         // until it gets removed from the list.
         // The returned node has a pointer which constrains it to the lifetime
         }
     }
 
    /// Returns a reference to the last node in the linked list
    /// The function is only safe as long as valid pointers are stored inside
    /// the linked list.
    /// The returned pointer is only guaranteed to be valid as long as the list
    /// is not mutated
    pub fn peek_last(&self) -> Option<&ListNode<T>> {
        // Safety: When the node was inserted it was promised that it is alive
        // until it gets removed from the list.
        // The returned node has a pointer which constrains it to the lifetime
        // of the list. This is ok, since the Node is supposed to outlive
        // its insertion in the list.
        unsafe {
            self.tail
                .map(|node| &*(node.as_ptr() as *const ListNode<T>))
        }
    }

    /// Returns a mutable reference to the last node in the linked list
     /// The function is only safe as long as valid pointers are stored inside
     /// the linked list.
     /// The returned pointer is only guaranteed to be valid as long as the list
     /// is not mutated
    pub fn peek_last_mut(&mut self) -> Option<&mut ListNode<T>> {
         // Safety: When the node was inserted it was promised that it is alive
         // until it gets removed from the list.
         // The returned node has a pointer which constrains it to the lifetime