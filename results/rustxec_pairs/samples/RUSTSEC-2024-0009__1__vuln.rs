fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() {
            Ok(Self(match s.parse::<KnownHeaderName>() {
                Ok(known) => KnownHeader(known),
                Err(()) => UnknownHeader(UnknownHeaderName(SmartCow::Owned(SmartString::from(s)))),
            }))
        } else {
            Err(Error::MalformedHeader(s.to_string().into()))
        }
    }