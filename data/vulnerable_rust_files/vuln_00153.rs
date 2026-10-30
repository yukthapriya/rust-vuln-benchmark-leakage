// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0150
// CVE: CVE-2020-36470
// CWE: N/A
// Crate: disrustor
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/disrustor
// Extraction Method: repo_search
// GitHub Repo: sklose/disrustor
// Commit: 0be7aed
// File: src/dsl.rs


#[derive(Debug)]
 pub struct DisrustorBuilder {}
 
pub struct WithDataProvider<D: DataProvider<T>, T> {
     data_provider: Arc<D>,
     _element: PhantomData<T>,
 }
 
pub struct WithWaitStrategy<W: WaitStrategy, D: DataProvider<T>, T> {
     with_data_provider: WithDataProvider<D, T>,
     _wait_strategy: PhantomData<W>,
 }
 
pub struct WithSequencer<S: Sequencer, W: WaitStrategy, D: DataProvider<T>, T> {
     with_wait_strategy: WithWaitStrategy<W, D, T>,
     sequencer: S,
 }
     _element: PhantomData<T>,
 }
 
pub struct WithEventHandlers<'a, S: Sequencer, W: WaitStrategy, D: DataProvider<T>, T> {
     with_sequencer: WithSequencer<S, W, D, T>,
     event_handlers: Vec<Box<dyn Runnable + 'a>>,
     gating_sequences: Vec<Arc<AtomicSequence>>,
 }
 
 impl DisrustorBuilder {
     #[allow(clippy::new_ret_no_self)]
    pub fn new<D: DataProvider<T>, T>(data_provider: Arc<D>) -> WithDataProvider<D, T> {
         WithDataProvider {
             data_provider,
             _element: Default::default(),
         }
     }
 
    pub fn with_ring_buffer<T: Default>(capacity: usize) -> WithDataProvider<RingBuffer<T>, T> {
         Self::new(Arc::new(RingBuffer::new(capacity)))
     }
 }
 
impl<D: DataProvider<T>, T> WithDataProvider<D, T> {
     pub fn with_wait_strategy<W: WaitStrategy>(self) -> WithWaitStrategy<W, D, T> {
         WithWaitStrategy {
             with_data_provider: self,
     }
 }
 
impl<W: WaitStrategy, D: DataProvider<T>, T> WithWaitStrategy<W, D, T> {
     pub fn with_sequencer<S: Sequencer>(self, sequencer: S) -> WithSequencer<S, W, D, T> {
         WithSequencer {
             with_wait_strategy: self,
     }
 }
 
impl<'a, S: Sequencer + 'a, W: WaitStrategy, D: DataProvider<T> + 'a, T: Send + 'a>
     WithSequencer<S, W, D, T>
 {
     pub fn with_barrier(
     }
 }
 
impl<'a, S: Sequencer + 'a, W: WaitStrategy, D: DataProvider<T> + 'a, T: Send + 'a>
     WithEventHandlers<'a, S, W, D, T>
 {
     pub fn with_barrier(mut self, f: impl FnOnce(&mut BarrierScope<'a, S, D, T>)) -> Self {