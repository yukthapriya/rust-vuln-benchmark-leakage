// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0159 (FIXED)
// Crate: chrono
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: chronotope/chrono
// Commit: 68fd4b6
// File: src/datetime/mod.rs


}
 }
 
/// Number of days between January 1, 1970 and December 31, 1 BCE which we define to be day 0.
 /// 4 full leap year cycles until December 31, 1600     4 * 146097 = 584388
 /// 1 day until January 1, 1601                                           1
/// 369 years until January 1, 1970                      369 * 365 = 134685
 /// of which floor(369 / 4) are leap years          floor(369 / 4) =     92
 /// except for 1700, 1800 and 1900                                       -3 +
 ///                                                                  --------