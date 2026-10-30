// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0136
// CVE: N/A
// CWE: N/A
// Crate: sass-rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/sass-rs
// Extraction Method: repo_search
// GitHub Repo: compass-rs/sass-rs
// Commit: 6ce9a50
// File: sass-sys/build.rs


for entry in t!(fs::read_dir(dir)) {
         let entry = t!(entry);
         let path = entry.path();
        let dst = dest.join(path.file_name().unwrap());
         if t!(fs::metadata(&path)).is_file() {
             t!(fs::copy(path, dst));
         } else {
         .arg("init")
         .stderr(Stdio::null())
         .status()
        .expect("")
         .success();
     Command::new("git")
         .arg("submodule")
         .arg("update")
         .stderr(Stdio::null())
         .status()
        .expect("")
         .success();
 }
 
 fn get_libsass_folder() -> PathBuf {
    env::current_dir().unwrap().join("libsass")
 }
 
 #[cfg(target_os = "linux")]
 fn compile() {
     let target = env::var("TARGET").expect("TARGET not found");
     let src = get_libsass_folder();
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());
     let build = dest.join("build");
     t!(fs::create_dir_all(&build));
     cp_r(&src, &build);
     let is_bsd = target.contains("dragonfly")
         || target.contains("freebsd")
         || target.contains("netbsd")
         || target.contains("openbsd");
    let jobs = env::var("MAKE_LIBSASS_JOBS").unwrap_or(num_cpus::get().to_string());
     let r = Command::new(if is_bsd { "gmake" } else { "make" })
         .current_dir(&build)
         .args(&["--jobs", &jobs])
         .output()
        .expect("error running make");
 
     if !r.status.success() {
         let err = String::from_utf8_lossy(&r.stderr);
         .arg("-shared")
         .stderr(Stdio::null())
         .status()
        .expect("")
         .success()
     };
     _compile(libprobe);
     } else {
         "Win32"
     };
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());
     let build = dest.join("build");
     t!(fs::create_dir_all(&build));
     cp_r(&src, &build);
             .args(&["/upgrade", "win\\libsass.sln"])
             .current_dir(&build)
             .output()
            .expect("error running devenv");
         if !d.status.success() {
             let err = String::from_utf8_lossy(&d.stderr);
             let out = String::from_utf8_lossy(&d.stdout);
     let search = Command::new("where")
         .args(&["msbuild.exe"])
         .output()
        .expect("Could not search for msbuild.exe on path");
     let mut msbuild = if search.status.success() {
         Command::new("msbuild.exe")
     } else {
         cc::windows_registry::find(target.as_str(), "msbuild.exe")
             .expect("Could not find msbuild.exe on the registry")
     };
 
    let jobs = env::var("MAKE_LIBSASS_JOBS").unwrap_or(num_cpus::get().to_string());
 
     let r = msbuild
         .args(&[
         ])
         .current_dir(&build)
         .output()
        .expect("error running msbuild");
 
     if !r.status.success() {
         let err = String::from_utf8_lossy(&r.stderr);