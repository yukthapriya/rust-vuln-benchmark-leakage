fn encode_headers(
        &mut self,
        dst: &mut BytesMut,
        version: Version,
        mut length: BodySize,
        ctype: ConnectionType,
        config: &ServiceConfig,
    ) -> io::Result<()> {
        let chunked = self.chunked();
        let mut skip_len = length != BodySize::Stream;
        let camel_case = self.camel_case();

        // Content length
        if let Some(status) = self.status() {
            match status {
                StatusCode::CONTINUE
                | StatusCode::SWITCHING_PROTOCOLS
                | StatusCode::PROCESSING
                | StatusCode::NO_CONTENT => {
                    // skip content-length and transfer-encoding headers
                    // See https://tools.ietf.org/html/rfc7230#section-3.3.1
                    // and https://tools.ietf.org/html/rfc7230#section-3.3.2
                    skip_len = true;
                    length = BodySize::None
                }
                _ => {}
            }
        }
        match length {
            BodySize::Stream => {
                if chunked {
                    skip_len = true;
                    if camel_case {
                        dst.put_slice(b"\r\nTransfer-Encoding: chunked\r\n")
                    } else {
                        dst.put_slice(b"\r\ntransfer-encoding: chunked\r\n")
                    }
                } else {
                    skip_len = false;
                    dst.put_slice(b"\r\n");
                }
            }
            BodySize::Empty => {
                if camel_case {
                    dst.put_slice(b"\r\nContent-Length: 0\r\n");
                } else {
                    dst.put_slice(b"\r\ncontent-length: 0\r\n");
                }
            }
            BodySize::Sized(len) => helpers::write_content_length(len, dst),
            BodySize::None => dst.put_slice(b"\r\n"),
        }

        // Connection
        match ctype {
            ConnectionType::Upgrade => dst.put_slice(b"connection: upgrade\r\n"),
            ConnectionType::KeepAlive if version < Version::HTTP_11 => {
                if camel_case {
                    dst.put_slice(b"Connection: keep-alive\r\n")
                } else {
                    dst.put_slice(b"connection: keep-alive\r\n")
                }
            }
            ConnectionType::Close if version >= Version::HTTP_11 => {
                if camel_case {
                    dst.put_slice(b"Connection: close\r\n")
                } else {
                    dst.put_slice(b"connection: close\r\n")
                }
            }
            _ => {}
        }

        // write headers

        let mut has_date = false;

        let mut buf = dst.chunk_mut().as_mut_ptr();
        let mut remaining = dst.capacity() - dst.len();

        // tracks bytes written since last buffer resize
        // since buf is a raw pointer to a bytes container storage but is written to without the
        // container's knowledge, this is used to sync the containers cursor after data is written
        let mut pos = 0;

        self.write_headers(|key, value| {
            match *key {
                CONNECTION => return,
                TRANSFER_ENCODING | CONTENT_LENGTH if skip_len => return,
                DATE => has_date = true,
                _ => {}
            }

            let k = key.as_str().as_bytes();
            let k_len = k.len();

            // TODO: drain?
            for val in value.iter() {
                let v = val.as_ref();
                let v_len = v.len();

                // key length + value length + colon + space + \r\n
                let len = k_len + v_len + 4;

                if len > remaining {
                    // SAFETY: all the bytes written up to position "pos" are initialized
                    // the written byte count and pointer advancement are kept in sync
                    unsafe {
                        dst.advance_mut(pos);
                    }

                    pos = 0;
                    dst.reserve(len * 2);
                    remaining = dst.capacity() - dst.len();

                    // re-assign buf raw pointer since it's possible that the buffer was
                    // reallocated and/or resized
                    buf = dst.chunk_mut().as_mut_ptr();
                }

                // SAFETY: on each write, it is enough to ensure that the advancement of
                // the cursor matches the number of bytes written
                unsafe {
                    if camel_case {
                        // use Camel-Case headers
                        write_camel_case(k, from_raw_parts_mut(buf, k_len));
                    } else {
                        write_data(k, buf, k_len);
                    }

                    buf = buf.add(k_len);

                    write_data(b": ", buf, 2);
                    buf = buf.add(2);

                    write_data(v, buf, v_len);
                    buf = buf.add(v_len);

                    write_data(b"\r\n", buf, 2);
                    buf = buf.add(2);
                };

                pos += len;
                remaining -= len;
            }
        });

        // final cursor synchronization with the bytes container
        //
        // SAFETY: all the bytes written up to position "pos" are initialized
        // the written byte count and pointer advancement are kept in sync
        unsafe {
            dst.advance_mut(pos);
        }

        // optimized date header, set_date writes \r\n
        if !has_date {
            config.set_date(dst);
        } else {
            // msg eof
            dst.extend_from_slice(b"\r\n");
        }

        Ok(())
    }