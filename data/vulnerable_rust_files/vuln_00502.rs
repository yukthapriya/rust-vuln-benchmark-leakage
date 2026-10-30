// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0027
// CVE: N/A
// CWE: N/A
// Crate: mp3-metadata
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/mp3-metadata
// Extraction Method: repo_search
// GitHub Repo: GuillaumeGomez/mp3-metadata
// Commit: 0923880
// File: src/metadata.rs


use std::path::Path;
 use std::time::Duration;
 
use consts::{BITRATES, SAMPLING_FREQ};
use enums::{ChannelType, Copyright, Emphasis, Error, Genre, Layer, Status, Version, CRC};
use types::{AudioTag, Frame, MP3Metadata, OptionalAudioTags};
use utils::{
     compute_duration, create_utf8_str, get_line, get_samp_line, get_text_field, get_text_fields,
 };
use utils::{get_url_field, get_url_fields};
 
 fn get_id3(i: &mut u32, buf: &[u8], meta: &mut MP3Metadata) -> Result<(), Error> {
     let mut x = *i as usize;