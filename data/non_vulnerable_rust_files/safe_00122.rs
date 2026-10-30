// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0109 (FIXED)
// Crate: stderr
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: biluohc/stderr
// Commit: 2fd8eb8
// File: src/lib.rs


```toml
   [dependencies]
  stderr = "0.8.1"
  ```
 
 # About stderr
 extern crate time;
 /** `log module`
 
`dbxx!()`/`infoxx!()`/`warnxx!()`/`errorxx!()/fatalxx` print message while  environment variable `LOG` or command line arguments conntains `--log`
 
 **Synntax: `LogLvl?/module_path,*`**
 
  set -x LOG "info/app,map"   # -> `info/app,map`
  set -e LOG                  # remove environment variable
 ```
### Cli_Option_Argument
 
 ```sh
   ./app --log   /               # -> `all/app`
   ./app --log   info/app        # -> `info/app`
   ./app --log   info/app,map    # -> `info/app,map`
 ```
 You must use `logger_init!()` before use them on the current process.
 */