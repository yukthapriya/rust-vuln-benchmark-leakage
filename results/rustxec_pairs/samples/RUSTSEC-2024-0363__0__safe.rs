fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let mut buf = <&[u8] as Decode<Postgres>>::decode(value)?;
        let len = read_length(&mut buf)?;

        let len =
            usize::try_from(len).map_err(|_| format!("PgHstore: length out of range: {len}"))?;

        let mut result = Self::default();

        for i in 0..len {
            let key = read_string(&mut buf)
                .map_err(|e| format!("PgHstore: error reading {i}th key: {e}"))?
                .ok_or_else(|| format!("PgHstore: expected {i}th key, got nothing"))?;

            let value = read_string(&mut buf)
                .map_err(|e| format!("PgHstore: error reading value for key {key:?}: {e}"))?;

            result.insert(key, value);
        }

        if !buf.is_empty() {
            tracing::warn!("{} unread bytes at the end of HSTORE value", buf.len());
        }

        Ok(result)
    }