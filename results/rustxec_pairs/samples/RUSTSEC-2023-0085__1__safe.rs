pub fn decode_with_cb<F>(&mut self, buf: &[u8], mut cb: F) -> Result<(), DecoderError>
            where F: FnMut(Cow<[u8]>, Cow<[u8]>) {
        let mut current_octet_index = 0;

        while current_octet_index < buf.len() {
            // At this point we are always at the beginning of the next block
            // within the HPACK data.
            // The type of the block can always be determined from the first
            // byte.
            let initial_octet = buf[current_octet_index];
            let buffer_leftover = &buf[current_octet_index..];
            let consumed = match FieldRepresentation::new(initial_octet) {
                FieldRepresentation::Indexed => {
                    let ((name, value), consumed) =
                        try!(self.decode_indexed(buffer_leftover));
                    cb(Cow::Borrowed(name), Cow::Borrowed(value));

                    consumed
                },
                FieldRepresentation::LiteralWithIncrementalIndexing => {
                    let ((name, value), consumed) = {
                        let ((name, value), consumed) = try!(
                            self.decode_literal(buffer_leftover, true));
                        cb(Cow::Borrowed(&name), Cow::Borrowed(&value));

                        // Since we are to add the decoded header to the header table, we need to
                        // convert them into owned buffers that the decoder can keep internally.
                        let name = name.into_owned();
                        let value = value.into_owned();

                        ((name, value), consumed)
                    };
                    // This cannot be done in the same scope as the `decode_literal` call, since
                    // Rust cannot figure out that the `into_owned` calls effectively drop the
                    // borrow on `self` that the `decode_literal` return value had. Since adding
                    // a header to the table requires a `&mut self`, it fails to compile.
                    // Manually separating it out here works around it...
                    self.header_table.add_header(name, value);

                    consumed
                },
                FieldRepresentation::LiteralWithoutIndexing => {
                    let ((name, value), consumed) =
                        try!(self.decode_literal(buffer_leftover, false));
                    cb(name, value);

                    consumed
                },
                FieldRepresentation::LiteralNeverIndexed => {
                    // Same as the previous one, except if we were also a proxy
                    // we would need to make sure not to change the
                    // representation received here. We don't care about this
                    // for now.
                    let ((name, value), consumed) =
                        try!(self.decode_literal(buffer_leftover, false));
                    cb(name, value);

                    consumed
                },
                FieldRepresentation::SizeUpdate => {
                    // Handle the dynamic table size update...
                    self.update_max_dynamic_size(buffer_leftover)?
                }
            };

            current_octet_index += consumed;
        }

        Ok(())
    }