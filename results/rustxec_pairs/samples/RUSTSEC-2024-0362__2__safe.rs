pub(crate) fn parser(input: &mut Input<'a>) -> PResult<Self> {
        trace("RootType", |input: &mut Input<'a>| {
            identifier_parser(input).map(|ident| {
                // Workaround for enums in library function params or returns.
                // See: https://github.com/alloy-rs/core/pull/386
                // See ethabi workaround: https://github.com/rust-ethereum/ethabi/blob/b1710adc18f5b771d2d2519c87248b1ba9430778/ethabi/src/param_type/reader.rs#L162-L167
                if input.starts_with('.') {
                    let _ = input.next_token();
                    let _ = identifier_parser(input);
                    return Self("uint8");
                }

                // Normalize the `u?int` aliases to the canonical `u?int256`
                match ident {
                    "uint" => Self("uint256"),
                    "int" => Self("int256"),
                    _ => Self(ident),
                }
            })
        })
        .parse_next(input)
    }