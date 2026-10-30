fn encode(&mut self, frame: &[u8], buf: &mut Vec<u8>) -> Result<(), Error> {
        match frame.len() {
            0 => {
                match self.escape_count {
                    0 => buf.write_all(b"\r\n.\r\n")?,
                    1 => buf.write_all(b"\n.\r\n")?,
                    2 => buf.write_all(b".\r\n")?,
                    _ => unreachable!(),
                }
                self.escape_count = 0;
                Ok(())
            }
            _ => {
                let mut start = 0;
                for (idx, byte) in frame.iter().enumerate() {
                    match self.escape_count {
                        0 => self.escape_count = if *byte == b'\r' { 1 } else { 0 },
                        1 => self.escape_count = if *byte == b'\n' { 2 } else { 0 },
                        2 => self.escape_count = if *byte == b'.' { 3 } else { 0 },
                        _ => unreachable!(),
                    }
                    if self.escape_count == 3 {
                        self.escape_count = 0;
                        buf.write_all(&frame[start..idx])?;
                        buf.write_all(b".")?;
                        start = idx;
                    }
                }
                buf.write_all(&frame[start..])?;
                Ok(())
            }
        }
    }