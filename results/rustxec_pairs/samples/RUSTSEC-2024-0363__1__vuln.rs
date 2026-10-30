fn read_length(buf: &mut &[u8]) -> Result<i32, BoxDynError> {
    let (bytes, rest) = buf.split_at(size_of::<i32>());

    *buf = rest;

    Ok(i32::from_be_bytes(
        bytes
            .try_into()
            .map_err(|err| format!("hstore, reading length: {err}"))?,
    ))
}