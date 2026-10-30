fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // XXX This can’t deal correctly with overly large components.
        //     Since this is a really rare (if not non-existant) case,
        //     I can’t be bothered to figure out how to convert a seven
        //     bit integer into decimal.
        let mut components = self.iter();
        // There’s at least one and it is always a valid u32.
        write!(f, "{}", components.next().unwrap().to_u32().unwrap())?;
        for component in components {
            if let Some(val) = component.to_u32() {
                write!(f, ".{}", val)?;
            }
            else {
                write!(f, ".(not implemented)")?;
            }
        }
        Ok(())
    }