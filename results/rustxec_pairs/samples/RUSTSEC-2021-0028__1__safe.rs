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
            
            // Prevent duplicate (or any) drops on the array we are modifying.
            // This is to safe-guard against a panic potentially caused by `iter.rev()`.
            // Alternative (less performant) approaches would be:
            // - append the new column to the array and use swapping to shuffle everything into place.
            // - store the new column data in a temporary location before shifting the memory and inserting values.
            self.data.set_len(0);
            
            let p = self.data.as_mut_ptr();
            let mut read_p = p.add(old_len);
            let mut write_p = p.add(new_len);

            let mut elem_count = 0usize;

            // Use `take()` to cap the number of elements processed because we cannot rely on
            // then `len()` value of `ExactSizeIterator` in unsafe code.
            for e in iter.take(self.num_rows).rev() {
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
                elem_count += 1;
            }
            
            // abort if the iterator length was less than expected
            assert_eq!(self.num_rows, elem_count, "unexpected iterator length");
            
            self.data.set_len(new_len);
        }

        // update the number of columns
        self.num_cols += 1;

    }