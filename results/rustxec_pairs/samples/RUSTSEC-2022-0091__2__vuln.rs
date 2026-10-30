fn push_pattern<P: AsRef<Path>>(list: &mut HashSet<Pattern>, pattern: P) -> crate::Result<()> {
  let path: PathBuf = pattern.as_ref().components().collect();
  list.insert(Pattern::new(&path.to_string_lossy())?);
  #[cfg(windows)]
  {
    if let Ok(p) = std::fs::canonicalize(&path) {
      list.insert(Pattern::new(&p.to_string_lossy())?);
    } else {
      list.insert(Pattern::new(&format!("\\\\?\\{}", path.display()))?);
    }
  }
  Ok(())
}