// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0032 (FIXED)
// Crate: alpm-rs
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: pigeonhands/rust-arch
// Commit: dccb1a1
// File: alpm-rs/src/question.rs


pub struct QuestionConflict{
     pub question_type: i32,
     pub answer: i32,
    pub conflict: Conflict,
 }
 
 
 impl From<*mut alpm_question_t> for QuestionConflict{
     fn from(q_raw: *mut alpm_question_t) -> Self{
        let q = unsafe{ *(q_raw as *mut alpm_question_conflict_t) };
         QuestionConflict{
             question_type: q.question_type,
             answer: q.answer,
            conflict: q.conflict.into(),
         }
     }
 }