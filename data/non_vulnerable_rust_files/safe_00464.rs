// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0426 (FIXED)
// Crate: spl-token-swap
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: solana-labs/solana-program-library
// Commit: 63b2173
// File: libraries/math-example/tests/instruction_count.rs


#[tokio::test]
 async fn test_precise_sqrt_u64_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     // This is way too big!  It's possible to dial down the numbers to get to
     // something reasonable, but the better option is to do everything in u64
 
 #[tokio::test]
 async fn test_precise_sqrt_u32_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(170_000);
 
 
 #[tokio::test]
 async fn test_sqrt_u64() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     // Dial down the BPF compute budget to detect if the operation gets bloated in
     // the future
 
 #[tokio::test]
 async fn test_sqrt_u128() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     // Dial down the BPF compute budget to detect if the operation gets bloated in
     // the future
 
 #[tokio::test]
 async fn test_sqrt_u128_max() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(7_000);
 
 
 #[tokio::test]
 async fn test_u64_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1350);
 
 
 #[tokio::test]
 async fn test_u64_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1650);
 
 
 #[tokio::test]
 async fn test_f32_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1600);
 
 
 #[tokio::test]
 async fn test_f32_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1650);
 
 
 #[tokio::test]
 async fn test_f32_exponentiate() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1400);
 
 
 #[tokio::test]
 async fn test_f32_natural_log() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(3500);
 
 
 #[tokio::test]
 async fn test_f32_normal_cdf() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     // Dial down the BPF compute budget to detect if the operation gets bloated in
     // the future
 
 #[tokio::test]
 async fn test_f64_pow() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(30_000);
 
 
 #[tokio::test]
 async fn test_u128_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(10000);
 
 
 #[tokio::test]
 async fn test_u128_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(10000);
 
 
 #[tokio::test]
 async fn test_f64_multiply() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(10000);
 
 
 #[tokio::test]
 async fn test_f64_divide() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(10000);
 
 
 #[tokio::test]
 async fn test_noop() {
    let mut pc = ProgramTest::new("spl_math_example", id(), processor!(process_instruction));
 
     pc.set_compute_max_units(1200);