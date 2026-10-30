fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let mut buf = <&[u8] as Decode<Postgres>>::decode(value)?;
        let len = read_length(&mut buf)?;

        if len < 0 {
            Err(format!("hstore, invalid entry count: {len}"))?;
        }

        let mut result = Self::default();

        while !buf.is_empty() {
            let key_len = read_length(&mut buf)?;
            let key = read_value(&mut buf, key_len)?.ok_or("hstore, key not found")?;

            let value_len = read_length(&mut buf)?;
            let value = read_value(&mut buf, value_len)?;

            result.insert(key, value);
        }

        Ok(result)
    }