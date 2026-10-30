pub fn identifier<'a>(input: &mut &'a str) -> PResult<&'a str> {
    identifier_parser(input)
}