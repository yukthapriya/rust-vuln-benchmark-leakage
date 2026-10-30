// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0075 (FIXED)
// Crate: unsafe-libyaml
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: dtolnay/unsafe-libyaml
// Commit: 7c42792
// File: src/scanner.rs


if yaml_parser_stale_simple_keys(parser).fail {
                 return FAIL;
             }
            simple_key = (*parser)
                .simple_keys
                .start
                .add((*parser).not_simple_keys as usize);
             while simple_key != (*parser).simple_keys.top {
                 if (*simple_key).possible && (*simple_key).token_number == (*parser).tokens_parsed {
                     need_more_tokens = true;
 
 unsafe fn yaml_parser_stale_simple_keys(parser: *mut yaml_parser_t) -> Success {
     let mut simple_key: *mut yaml_simple_key_t;
    simple_key = (*parser)
        .simple_keys
        .start
        .add((*parser).not_simple_keys as usize);
     while simple_key != (*parser).simple_keys.top {
         if (*simple_key).possible
             && ((*simple_key).mark.line < (*parser).mark.line
                 return FAIL;
             }
             (*simple_key).possible = false;
            if (*parser)
                .simple_keys
                .start
                .add((*parser).not_simple_keys as usize)
                == simple_key
            {
                (*parser).not_simple_keys += 1;
            }
         }
         simple_key = simple_key.wrapping_offset(1);
     }
             return FAIL;
         }
         *(*parser).simple_keys.top.wrapping_offset(-1_isize) = simple_key;
        if (*parser)
            .simple_keys
            .start
            .add((*parser).not_simple_keys as usize)
            == (*parser).simple_keys.top
        {
            (*parser).not_simple_keys -= 1;
        }
     }
     OK
 }
     if (*parser).flow_level != 0 {
         let fresh8 = addr_of_mut!((*parser).flow_level);
         *fresh8 -= 1;
        if (*parser)
            .simple_keys
            .start
            .add((*parser).not_simple_keys as usize)
            == (*parser).simple_keys.top
        {
            (*parser).not_simple_keys -= 1;
        }
         let _ = POP!((*parser).simple_keys);
     }
 }
     let token = token.as_mut_ptr();
     (*parser).indent = -1;
     PUSH!((*parser).simple_keys, simple_key);
    (*parser).not_simple_keys = 1;
     (*parser).simple_key_allowed = true;
     (*parser).stream_start_produced = true;
     memset(