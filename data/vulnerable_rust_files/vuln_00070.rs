// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0032
// CVE: CVE-2020-35885
// CWE: N/A
// Crate: alpm-rs
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/alpm-rs
// Extraction Method: repo_search
// GitHub Repo: pigeonhands/rust-arch
// Commit: dccb1a1
// File: alpm-rs/src/question.rs


pub struct QuestionConflict{
     pub question_type: i32,
     pub answer: i32,
    pub oldpkg: Package,
    pub newpkg: Package,
    pub newdb: AlpmDB,
 }
 
 
 impl From<*mut alpm_question_t> for QuestionConflict{
     fn from(q_raw: *mut alpm_question_t) -> Self{
        let q = unsafe{ *(q_raw as *mut alpm_question_replace_t) };
         QuestionConflict{
             question_type: q.question_type,
             answer: q.answer,
            oldpkg: q.oldpkg.into(),
            newpkg: q.newpkg.into(),
            newdb: q.newdb.into(),
         }
     }
 }