fn read_length(buf: &mut &[u8]) -> Result<i32, String> {
    if buf.len() < size_of::<i32>() {
        return Err(format!(
            "expected {} bytes, got {}",
            size_of::<i32>(),
            buf.len()
        ));
    }

    Ok(buf.get_i32())
}