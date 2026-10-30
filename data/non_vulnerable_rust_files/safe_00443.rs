// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0383 (FIXED)
// Crate: bcc
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-bpf/rust-bcc
// Commit: 639cead
// File: examples/tcpretrans.rs


get_datetime(),
             event.pid,
             event.ip,
            format!("{}:{}", Ipv4Addr::from(u32::from_be(event.saddr)), event.lport),
             event.type_,
            format!("{}:{}", Ipv4Addr::from(u32::from_be(event.daddr)), event.dport),
             event.state,
         );
     })
             get_datetime(),
             event.pid,
             event.ip,
            format!("{}:{}", Ipv6Addr::from(u128::from_be(event.saddr)), event.lport),
             event.type_,
            format!("{}:{}", Ipv6Addr::from(u128::from_be(event.daddr)), event.dport),
             event.state,
         );
     })