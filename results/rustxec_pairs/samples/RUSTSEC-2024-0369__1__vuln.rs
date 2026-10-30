pub fn parse_with<S: AsRef<str>>(
    database: &Database,
    country: Option<country::Id>,
    string: S,
) -> Result<PhoneNumber, error::Parse> {
    fn phone_number(i: &str) -> IResult<&str, helper::Number<'_>> {
        parse! { i => alt((rfc3966::phone_number, natural::phone_number)) }
    }

    // Try to parse the number as RFC3966 or natural language.
    let (_, mut number) = phone_number(string.as_ref()).or(Err(error::Parse::NoNumber))?;

    // Normalize the number and extract country code.
    number = helper::country_code(database, country, number)?;

    // Extract carrier and strip national prefix if present.
    if let Some(meta) = country.and_then(|c| database.by_id(c.as_ref())) {
        let mut potential = helper::national_number(meta, number.clone());

        // Strip national prefix if present.
        if let Some(prefix) = meta.national_prefix.as_ref() {
            if potential.national.starts_with(prefix) {
                potential.national = helper::trim(potential.national, prefix.len());
            }
        }

        if validator::length(meta, &potential, Type::Unknown) != Validation::TooShort {
            number = potential;
        }
    }

    if number.national.len() < consts::MIN_LENGTH_FOR_NSN {
        return Err(error::Parse::TooShortNsn);
    }

    if number.national.len() > consts::MAX_LENGTH_FOR_NSN {
        return Err(error::Parse::TooLong);
    }

    Ok(PhoneNumber {
        code: country::Code {
            value: number.prefix.map(|p| p.parse()).unwrap_or(Ok(0))?,
            source: number.country,
        },

        national: NationalNumber::new(
            number.national.parse()?,
            number.national.chars().take_while(|&c| c == '0').count() as u8,
        ),

        extension: number.extension.map(|s| Extension(s.into_owned())),
        carrier: number.carrier.map(|s| Carrier(s.into_owned())),
    })
}