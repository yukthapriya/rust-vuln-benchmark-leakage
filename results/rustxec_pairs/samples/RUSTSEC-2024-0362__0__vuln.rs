pub fn identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
    // See note in `is_valid_identifier` above.
    // Use the faster `slice::Iter` instead of `str::Chars`.
    let mut chars = input.as_bytes().iter().map(|b| *b as char);

    let Some(true) = chars.next().map(is_id_start) else {
        return Err(ErrMode::from_error_kind(input, ErrorKind::Fail));
    };

    // 1 for the first character, we know it's ASCII
    let len = 1 + chars.take_while(|c| is_id_continue(*c)).count();
    // SAFETY: Only ASCII characters are valid in identifiers
    unsafe {
        let ident = input.get_unchecked(..len);
        *input = input.get_unchecked(len..);
        Ok(ident)
    }
}