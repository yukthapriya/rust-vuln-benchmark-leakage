// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0109
// CVE: N/A
// CWE: N/A
// Crate: stderr
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/stderr
// Extraction Method: repo_search
// GitHub Repo: biluohc/stderr
// Commit: 2fd8eb8
// File: src/lib.rs


```toml
   [dependencies]
  stderr = "0.8.0"
  ```
 
 # About stderr
 extern crate time;
 /** `log module`
 
`dbxx!()`/`infoxx!()`/`warnxx!()`/`errorxx!()/fatalxx` print message while  environment variable 'LOG' or command line arguments conntains `-log` or `--log`
 
 **Synntax: `LogLvl?/module_path,*`**
 
  set -x LOG "info/app,map"   # -> `info/app,map`
  set -e LOG                  # remove environment variable
 ```
### Cli_Options_Argument
 
 ```sh
   ./app -log   /               # -> `all/app`
   ./app -log   info/app        # -> `info/app`
   ./app -log   info/app,map    # -> `info/app,map`
 ```
 You must use `logger_init!()` before use them on the current process.
 */