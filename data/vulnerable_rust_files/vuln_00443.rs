// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0383
// CVE: N/A
// CWE: N/A
// Crate: bcc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/bcc
// Extraction Method: repo_search
// GitHub Repo: rust-bpf/rust-bcc
// Commit: 639cead
// File: examples/tcpretrans.rs


get_datetime(),
             event.pid,
             event.ip,
            format!("{}:{}", Ipv4Addr::from(event.saddr), event.lport),
             event.type_,
            format!("{}:{}", Ipv4Addr::from(event.daddr), event.dport),
             event.state,
         );
     })
             get_datetime(),
             event.pid,
             event.ip,
            format!("{}:{}", Ipv6Addr::from(event.saddr), event.lport),
             event.type_,
            format!("{}:{}", Ipv6Addr::from(event.daddr), event.dport),
             event.state,
         );
     })