pub fn resize(&mut self, len: usize, value: T)
    where
        T: Clone,
    {
        let old_len = self.slice.len;

        if self.capacity < len {
            self.reserve(len - old_len);
        }

        for i in old_len..len.saturating_sub(1) {
            unsafe { ptr::write(self.slice.ptr.as_ptr().add(i), value.clone()) }
        }

        if len > old_len {
            unsafe {
                ptr::write(self.slice.ptr.as_ptr().add(len - 1), value);
            }
        } else if len < old_len {
            unsafe {
                ptr::drop_in_place(&mut self.slice[len..old_len]);
            }
        }

        self.slice.len = len;
    }