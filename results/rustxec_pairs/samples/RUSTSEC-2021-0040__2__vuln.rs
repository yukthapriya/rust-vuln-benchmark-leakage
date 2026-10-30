pub fn new(handle: H, len: usize) -> Self
    where
        T: Default,
    {
        let mut res = unsafe { Self::new_empty(handle, len) };
        res.len = len;

        for i in 0..len {
            unsafe {
                ptr::write(res.ptr.as_ptr().add(i), T::default());
            }
        }

        res
    }