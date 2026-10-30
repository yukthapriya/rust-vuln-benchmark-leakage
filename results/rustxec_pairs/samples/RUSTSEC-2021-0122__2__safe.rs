fn write_vtable(
        &mut self,
        table_tail_revloc: WIPOffset<TableUnfinishedWIPOffset>,
    ) -> WIPOffset<VTableWIPOffset> {
        self.assert_nested("write_vtable");

        // Write the vtable offset, which is the start of any Table.
        // We fill its value later.
        let object_revloc_to_vtable: WIPOffset<VTableWIPOffset> =
            WIPOffset::new(self.push::<UOffsetT>(0xF0F0_F0F0).value());

        // Layout of the data this function will create when a new vtable is
        // needed.
        // --------------------------------------------------------------------
        // vtable starts here
        // | x, x -- vtable len (bytes) [u16]
        // | x, x -- object inline len (bytes) [u16]
        // | x, x -- zero, or num bytes from start of object to field #0   [u16]
        // | ...
        // | x, x -- zero, or num bytes from start of object to field #n-1 [u16]
        // vtable ends here
        // table starts here
        // | x, x, x, x -- offset (negative direction) to the vtable [i32]
        // |               aka "vtableoffset"
        // | -- table inline data begins here, we don't touch it --
        // table ends here -- aka "table_start"
        // --------------------------------------------------------------------
        //
        // Layout of the data this function will create when we re-use an
        // existing vtable.
        //
        // We always serialize this particular vtable, then compare it to the
        // other vtables we know about to see if there is a duplicate. If there
        // is, then we erase the serialized vtable we just made.
        // We serialize it first so that we are able to do byte-by-byte
        // comparisons with already-serialized vtables. This 1) saves
        // bookkeeping space (we only keep revlocs to existing vtables), 2)
        // allows us to convert to little-endian once, then do
        // fast memcmp comparisons, and 3) by ensuring we are comparing real
        // serialized vtables, we can be more assured that we are doing the
        // comparisons correctly.
        //
        // --------------------------------------------------------------------
        // table starts here
        // | x, x, x, x -- offset (negative direction) to an existing vtable [i32]
        // |               aka "vtableoffset"
        // | -- table inline data begins here, we don't touch it --
        // table starts here: aka "table_start"
        // --------------------------------------------------------------------

        // fill the WIP vtable with zeros:
        let vtable_byte_len = get_vtable_byte_len(&self.field_locs);
        self.make_space(vtable_byte_len);

        // compute the length of the table (not vtable!) in bytes:
        let table_object_size = object_revloc_to_vtable.value() - table_tail_revloc.value();
        debug_assert!(table_object_size < 0x10000); // vTable use 16bit offsets.

        // Write the VTable (we may delete it afterwards, if it is a duplicate):
        let vt_start_pos = self.head;
        let vt_end_pos = self.head + vtable_byte_len;
        {
            // write the vtable header:
            let vtfw = &mut VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]);
            vtfw.write_vtable_byte_length(vtable_byte_len as VOffsetT);
            vtfw.write_object_inline_size(table_object_size as VOffsetT);

            // serialize every FieldLoc to the vtable:
            for &fl in self.field_locs.iter() {
                let pos: VOffsetT = (object_revloc_to_vtable.value() - fl.off) as VOffsetT;
                vtfw.write_field_offset(fl.id, pos);
            }
        }
        let new_vt_bytes = &self.owned_buf[vt_start_pos..vt_end_pos];
        let found = self
            .written_vtable_revpos
            .binary_search_by(|old_vtable_revpos: &UOffsetT| {
                let old_vtable_pos = self.owned_buf.len() - *old_vtable_revpos as usize;
                // Safety:
                // Already written vtables are valid by construction
                let old_vtable = unsafe { VTable::init(&self.owned_buf, old_vtable_pos) };
                new_vt_bytes.cmp(old_vtable.as_bytes())
            });
        let final_vtable_revpos = match found {
            Ok(i) => {
                // The new vtable is a duplicate so clear it.
                VTableWriter::init(&mut self.owned_buf[vt_start_pos..vt_end_pos]).clear();
                self.head += vtable_byte_len;
                self.written_vtable_revpos[i]
            }
            Err(i) => {
                // This is a new vtable. Add it to the cache.
                let new_vt_revpos = self.used_space() as UOffsetT;
                self.written_vtable_revpos.insert(i, new_vt_revpos);
                new_vt_revpos
            }
        };
        // Write signed offset from table to its vtable.
        let table_pos = self.owned_buf.len() - object_revloc_to_vtable.value() as usize;
        if cfg!(debug_assertions) {
            // Safety:
            // Verified slice length
            let tmp_soffset_to_vt = unsafe {
                read_scalar::<UOffsetT>(&self.owned_buf[table_pos..table_pos + SIZE_UOFFSET])
            };
            assert_eq!(tmp_soffset_to_vt, 0xF0F0_F0F0);
        }

        let buf = &mut self.owned_buf[table_pos..table_pos + SIZE_SOFFSET];
        // Safety:
        // Verified length of buf above
        unsafe {
            emplace_scalar::<SOffsetT>(
                buf,
                final_vtable_revpos as SOffsetT - object_revloc_to_vtable.value() as SOffsetT,
            );
        }

        self.field_locs.clear();

        object_revloc_to_vtable
    }