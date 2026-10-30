pub fn insert_col<I>(&mut self, index: usize, data: impl IntoIterator<Item=T, IntoIter=I>)
    where I : Iterator<Item=T> + ExactSizeIterator + DoubleEndedIterator
    {
        assert!(index <= self.num_cols);
        let iter = data.into_iter();
        if self.num_cols == 0 {
            self.num_rows = iter.len();
        } else {
            assert_eq!(self.num_rows, iter.len());
        }
        
        self.reserve(self.num_rows);
        
        let old_len = self.data.len();
        let new_len = old_len + self.num_rows;
        let suffix_len = self.num_cols - index;
        unsafe {
            let p = self.data.as_mut_ptr();
            let mut read_p = p.add(old_len);
            let mut write_p = p.add(new_len);
            for e in iter.rev() {
                // shift suffix
                read_p = read_p.sub(suffix_len);
                write_p = write_p.sub(suffix_len);
                ptr::copy(read_p, write_p, suffix_len);
                write_p = write_p.sub(1);
                // place new col element
                ptr::write(write_p, e);
                // shift prefix
                read_p = read_p.sub(index);
                write_p = write_p.sub(index);
                ptr::copy(read_p, write_p, index);
            }
            self.data.set_len(new_len);
        }

        // update the number of columns
        self.num_cols += 1;

    }