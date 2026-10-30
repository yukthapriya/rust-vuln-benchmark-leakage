pub fn forbid_directory<P: AsRef<Path>>(&self, path: P, recursive: bool) -> crate::Result<()> {
    let path = path.as_ref().to_path_buf();
    {
      let mut list = self.forbidden_patterns.lock().unwrap();

      // allow the directory to be read
      push_pattern(&mut list, &path)?;
      // allow its files and subdirectories to be read
      push_pattern(&mut list, path.join(if recursive { "**" } else { "*" }))?;
    }
    self.trigger(Event::PathForbidden(path));
    Ok(())
  }