pub fn merge_message<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
        self.incr_recursion()?;
        struct DecrRecursion<'a, 'b>(&'a mut CodedInputStream<'b>);
        impl<'a, 'b> Drop for DecrRecursion<'a, 'b> {
            fn drop(&mut self) {
                self.0.decr_recursion();
            }
        }

        let mut decr = DecrRecursion(self);

        let len = decr.0.read_raw_varint64()?;
        let old_limit = decr.0.push_limit(len)?;
        message.merge_from(&mut decr.0)?;
        decr.0.pop_limit(old_limit);
        Ok(())
    }