// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0072
// CVE: CVE-2020-35915
// CWE: N/A
// Crate: futures-intrusive
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/futures-intrusive
// Extraction Method: repo_search
// GitHub Repo: Matthias247/futures-intrusive
// Commit: 842c2b1
// File: src/intrusive_double_linked_list.rs


}
     }
 
    /// Returns the first node in the linked list without removing it from the list
     /// The function is only safe as long as valid pointers are stored inside
     /// the linked list.
     /// The returned pointer is only guaranteed to be valid as long as the list
     /// is not mutated
    pub fn peek_first(&self) -> Option<&mut ListNode<T>> {
         // Safety: When the node was inserted it was promised that it is alive
         // until it gets removed from the list.
         // The returned node has a pointer which constrains it to the lifetime
         }
     }
 
    /// Returns the last node in the linked list without removing it from the list
     /// The function is only safe as long as valid pointers are stored inside
     /// the linked list.
     /// The returned pointer is only guaranteed to be valid as long as the list
     /// is not mutated
    pub fn peek_last(&self) -> Option<&mut ListNode<T>> {
         // Safety: When the node was inserted it was promised that it is alive
         // until it gets removed from the list.
         // The returned node has a pointer which constrains it to the lifetime