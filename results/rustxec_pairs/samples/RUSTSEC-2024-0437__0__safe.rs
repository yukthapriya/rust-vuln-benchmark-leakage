pub fn merge_message<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
        self.incr_recursion()?;
        let ret = self.merge_message_no_depth_check(message);
        self.decr_recursion();
        ret
    }