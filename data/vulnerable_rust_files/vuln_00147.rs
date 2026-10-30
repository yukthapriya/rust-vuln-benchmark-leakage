// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0143
// CVE: CVE-2020-36463
// CWE: N/A
// Crate: multiqueue
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/multiqueue
// Extraction Method: repo_search
// GitHub Repo: schets/multiqueue
// Commit: 0f050ca
// File: src/lib.rs


//! In many cases, ```MultiQueue``` will be a good replacement for channels and it's broadcast
 //! capabilities can replace more complex concurrency systems with a single queue.
 //!
//! #Queue Model:
 //! ```MultiQueue``` functions similarly to the LMAX Disruptor from a high level view.
 //! There's an incoming FIFO data stream that is broadcast to a set of subscribers
 //! as if there were multiple streams being written to.
 //! the workload completely on one core, @-2 is doing expensive work handling requests
 //! and is split into multiple workers dealing with the data stream.
 //!
//! #MPMC Mode:
 //! One might notice that the broadcast queue modes requires that a type be Clone,
 //! and the single-reader inplace variants require that a type be Sync as well.
 //! This is only required for broadcast queues and not normal mpmc queues,
 //! so there's an mpmc api as well. It doesn't require that a type be Clone or Sync
 //! for any api, and also moves items directly out of the queue instead of cloning them.
 //!
//! #Futures Mode:
 //! For both mpmc and broadcast, a futures mode is supported. The datastructures are quite
 //! similar to the normal ones, except they implement the Futures Sink/Stream traits for
 //! senders and receivers. This comes at a bit of a performance cost, which is why the
 //! futures types are separate
 //!
//! #Usage:
 //! From the receiving side, this behaves quite similarly to a channel receiver.
 //! The .recv function will block until data is available and then return the data.
 //!