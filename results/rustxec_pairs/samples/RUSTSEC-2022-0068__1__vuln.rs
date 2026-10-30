pub fn get_pointer_element(self, index: ElementCount32) -> PointerReader<'a> {
        let offset = (u64::from(index) * u64::from(self.step) / BITS_PER_BYTE as u64) as u32;
        PointerReader {
            arena: self.arena,
            segment_id: self.segment_id,
            cap_table: self.cap_table,
            pointer: unsafe { self.ptr.offset(offset as isize) } as *const _,
            nesting_limit: self.nesting_limit,
        }
    }