pub fn get_pointer_element(self, index: ElementCount32) -> PointerReader<'a> {
        let offset = (self.struct_data_size as u64 / BITS_PER_BYTE as u64
            + u64::from(index) * u64::from(self.step) / BITS_PER_BYTE as u64)
            as isize;
        PointerReader {
            arena: self.arena,
            segment_id: self.segment_id,
            cap_table: self.cap_table,
            pointer: unsafe { self.ptr.offset(offset) } as *const _,
            nesting_limit: self.nesting_limit,
        }
    }