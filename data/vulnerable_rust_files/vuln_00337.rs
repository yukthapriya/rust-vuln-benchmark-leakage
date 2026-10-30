// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0027
// CVE: N/A
// CWE: N/A
// Crate: async-nats
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/async-nats
// Extraction Method: commit_diff
// GitHub Repo: nats-io/nats.rs
// Commit: 817a7b9
// File: async-nats/src/connector.rs


})
                 .map_err(|err| ConnectError::with_source(crate::ConnectErrorKind::Tls, err))?;
 
            // Use the server-advertised hostname to validate if given as a hostname, not an IP address
            let domain = if let Ok(server_hostname @ rustls::ServerName::DnsName(_)) =
                rustls::ServerName::try_from(info.host.as_str())
            {
                server_hostname
            } else if let Ok(tls_hostname @ rustls::ServerName::DnsName(_)) =
                rustls::ServerName::try_from(tls_host)
            {
                tls_hostname
            } else {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "cannot determine hostname for TLS connection",
                ))
                .map_err(|err| ConnectError::with_source(crate::ConnectErrorKind::Tls, err));
            };
 
             connection = Connection {
                 stream: Box::new(tls_connector.connect(domain, connection.stream).await?),