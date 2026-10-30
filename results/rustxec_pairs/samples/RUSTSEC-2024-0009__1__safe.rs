fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(known) = s.parse::<KnownHeaderName>() {
            return Ok(known.into());
        }
        let uhn = UnknownHeaderName::from(s.to_string());
        if uhn.is_valid() {
            Ok(uhn.into())
        } else {
            Err(Error::MalformedHeader(s.to_string().into()))
        }
    }