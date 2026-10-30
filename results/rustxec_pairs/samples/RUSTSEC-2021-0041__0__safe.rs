pub fn parse(input: &str) -> Result<Duration, Error> {
    if let Some(int) = NUMBER_RE.captures(input) {
        // This means it's just a value
        // Since the regex matched, the first group exists, so we can unwrap.
        let seconds = BigInt::parse_bytes(int.get(1).unwrap().as_str().as_bytes(), 10)
            .ok_or_else(|| Error::ParseInt(int.get(1).unwrap().as_str().to_owned()))?;
        Ok(Duration::new(
            seconds
                .to_u64()
                .ok_or_else(|| Error::OutOfBounds(seconds))?,
            0,
        ))
    } else if DURATION_RE.is_match(input) {
        // This means we have at least one "unit" (or plain word) and one value.
        let mut duration = ProtoDuration::default();
        for capture in DURATION_RE.captures_iter(input) {
            match (
                capture.name("int"),
                capture.name("dec"),
                capture.name("exp"),
                capture.name("unit"),
            ) {
                // capture.get(0) is *always* the actual match, so unwrapping causes no problems
                (.., None) => {
                    return Err(Error::NoUnitFound(
                        capture.get(0).unwrap().as_str().to_owned(),
                    ))
                }
                (None, ..) => {
                    return Err(Error::NoValueFound(
                        capture.get(0).unwrap().as_str().to_owned(),
                    ))
                }
                (Some(int), None, None, Some(unit)) => {
                    let int = BigInt::parse_bytes(int.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(int.as_str().to_owned()))?;

                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => duration.nanoseconds += int,
                        "microseconds" => duration.microseconds += int,
                        "milliseconds" => duration.milliseconds += int,
                        "seconds" => duration.seconds += int,
                        "minutes" => duration.minutes += int,
                        "hours" => duration.hours += int,
                        "days" => duration.days += int,
                        "weeks" => duration.weeks += int,
                        "months" => duration.months += int,
                        "years" => duration.years += int,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }
                }
                (Some(int), Some(dec), None, Some(unit)) => {
                    let int = BigInt::parse_bytes(int.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(int.as_str().to_owned()))?;

                    let exp = dec.as_str().len();

                    let dec = BigInt::parse_bytes(dec.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(dec.as_str().to_owned()))?;

                    // boosted_int is value * 10^exp * unit
                    let mut boosted_int = int * pow(BigInt::from(10), exp) + dec;

                    // boosted_int is now value * 10^exp * nanoseconds
                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => boosted_int = boosted_int,
                        "microseconds" => boosted_int = 1_000_u64 * boosted_int,
                        "milliseconds" => boosted_int = 1_000_000_u64 * boosted_int,
                        "seconds" => boosted_int = 1_000_000_000_u64 * boosted_int,
                        "minutes" => boosted_int = 60_000_000_000_u64 * boosted_int,
                        "hours" => boosted_int = 3_600_000_000_000_u64 * boosted_int,
                        "days" => boosted_int = 86_400_000_000_000_u64 * boosted_int,
                        "weeks" => boosted_int = 604_800_000_000_000_u64 * boosted_int,
                        "months" => boosted_int = 2_629_746_000_000_000_u64 * boosted_int,
                        "years" => boosted_int = 31_556_952_000_000_000_u64 * boosted_int,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }

                    // boosted_int is now value * nanoseconds (rounding down)
                    boosted_int /= pow(BigInt::from(10), exp);
                    duration.nanoseconds += boosted_int;
                }
                (Some(int), None, Some(exp), Some(unit)) => {
                    let int = BigInt::parse_bytes(int.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(int.as_str().to_owned()))?;

                    let exp = exp
                        .as_str()
                        .parse::<isize>()
                        .or_else(|_| Err(Error::ParseInt(exp.as_str().to_owned())))?;

                    // boosted_int is value * 10^-exp * unit
                    let mut boosted_int = int;

                    // boosted_int is now value * 10^-exp * nanoseconds
                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => boosted_int = boosted_int,
                        "microseconds" => boosted_int = 1_000_u64 * boosted_int,
                        "milliseconds" => boosted_int = 1_000_000_u64 * boosted_int,
                        "seconds" => boosted_int = 1_000_000_000_u64 * boosted_int,
                        "minutes" => boosted_int = 60_000_000_000_u64 * boosted_int,
                        "hours" => boosted_int = 3_600_000_000_000_u64 * boosted_int,
                        "days" => boosted_int = 86_400_000_000_000_u64 * boosted_int,
                        "weeks" => boosted_int = 604_800_000_000_000_u64 * boosted_int,
                        "months" => boosted_int = 2_629_746_000_000_000_u64 * boosted_int,
                        "years" => boosted_int = 31_556_952_000_000_000_u64 * boosted_int,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }

                    // boosted_int is now value * nanoseconds
                    // x.wrapping_abs() as usize will always give the intended result
                    // This is because isize::MIN as usize == abs(isize::MIN) (as a usize)
                    if exp < -1000000 { // Prevents pow() from taking too much time
                        boosted_int = BigInt::from(0);
                    } else if exp < 0 {
                        boosted_int /= pow(BigInt::from(10), exp.wrapping_abs() as usize);
                    } else if exp > 1000000 {
                        return Err(Error::OutOfBounds(exp.into()))
                    } else {
                        boosted_int *= pow(BigInt::from(10), exp.wrapping_abs() as usize);
                    }
                    duration.nanoseconds += boosted_int;
                }
                (Some(int), Some(dec), Some(exp), Some(unit)) => {
                    let int = BigInt::parse_bytes(int.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(int.as_str().to_owned()))?;

                    let dec_exp = dec.as_str().len();

                    let exp = exp
                        .as_str()
                        .parse::<BigInt>()
                        .or_else(|_| Err(Error::ParseInt(exp.as_str().to_owned())))?
                        - (BigInt::from(dec_exp));
                    let exp = exp.to_isize().ok_or_else(|| Error::OutOfBounds(exp))?;

                    let dec = BigInt::parse_bytes(dec.as_str().as_bytes(), 10)
                        .ok_or_else(|| Error::ParseInt(dec.as_str().to_owned()))?;

                    // boosted_int is value * 10^-exp * unit
                    let mut boosted_int = int * pow(BigInt::from(10), dec_exp) + dec;

                    // boosted_int is now value * 10^-exp * nanoseconds
                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => boosted_int = boosted_int,
                        "microseconds" => boosted_int *= 1_000_u64,
                        "milliseconds" => boosted_int *= 1_000_000_u64,
                        "seconds" => boosted_int *= 1_000_000_000_u64,
                        "minutes" => boosted_int *= 60_000_000_000_u64,
                        "hours" => boosted_int *= 3_600_000_000_000_u64,
                        "days" => boosted_int *= 86_400_000_000_000_u64,
                        "weeks" => boosted_int *= 604_800_000_000_000_u64,
                        "months" => boosted_int *= 2_629_746_000_000_000_u64,
                        "years" => boosted_int *= 31_556_952_000_000_000_u64,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }

                    // boosted_int is now value * nanoseconds (potentially rounded down)
                    // x.wrapping_abs() as usize will always give the intended result
                    // This is because isize::MIN as usize == abs(isize::MIN) (as a usize)
                    if exp < -1000000 {
                        boosted_int = BigInt::from(0);
                    } else if exp < 0 {
                        boosted_int /= pow(BigInt::from(10), exp.wrapping_abs() as usize);
                    } else if exp > 1000000 {
                        return Err(Error::OutOfBounds(exp.into()))
                    } else {
                        boosted_int *= pow(BigInt::from(10), exp.wrapping_abs() as usize);
                    }
                    duration.nanoseconds += boosted_int;
                }
            }
        }
        duration.into_duration()
    } else {
        // Just a unit or nothing at all
        Err(Error::NoValueFound(input.to_owned()))
    }
}