fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut components = self.iter();
        match components.next() {
            Some(component) => component.fmt(f)?,
            None => { return Ok(()) }
        }
        components.try_for_each(|item| write!(f, ".{}", item))
    }