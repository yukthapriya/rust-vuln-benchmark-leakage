// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0100
// CVE: CVE-2020-36434
// CWE: N/A
// Crate: sys-info
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/sys-info
// Extraction Method: repo_search
// GitHub Repo: FillZpp/sys-info-rs
// Commit: ce3d516
// File: lib.rs


.collect::<Vec<f64>>();
         bt.tv_sec = secs[0] as libc::time_t;
         bt.tv_usec = secs[1] as libc::suseconds_t;
	Ok(bt)
     }
     #[cfg(any(target_vendor = "apple", target_os="freebsd", target_os = "openbsd", target_os = "netbsd"))]
     {
             if sysctl(&mut mib[0], 2,
                    &mut bt as *mut timeval as *mut libc::c_void,
                    &mut size, null_mut(), 0) == -1 {
		Err(Error::IO(io::Error::last_os_error()))
	    } else {
		Ok(bt)
	    }
         }
     }
     #[cfg(any(target_os = "solaris", target_os = "illumos"))]
             return Err(Error::General("time went backwards".into()));
         }
         bt.tv_sec = (now - start) as i64;
	Ok(bt)
     }
 }
 
 #[cfg(test)]