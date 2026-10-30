fn set_resource_inner(
        &mut self,
        id: gix_hash::ObjectId,
        mode: gix_object::tree::EntryKind,
        rela_path: &BStr,
        kind: ResourceKind,
        objects: &impl gix_object::FindObjectOrHeader,
    ) -> Result<(), set_resource::Error> {
        if matches!(
            mode,
            gix_object::tree::EntryKind::Commit | gix_object::tree::EntryKind::Tree
        ) {
            return Err(set_resource::Error::InvalidMode { mode });
        }
        let storage = match kind {
            ResourceKind::OldOrSource => &mut self.old,
            ResourceKind::NewOrDestination => &mut self.new,
        }
        .get_or_insert_with(Default::default);

        storage.id = id;
        storage.set_location(rela_path);
        storage.is_link = matches!(mode, gix_object::tree::EntryKind::Link);
        storage.use_id = self.filter.roots.by_kind(kind).is_none();

        if self.diff_cache.contains_key(storage) {
            return Ok(());
        }
        let entry =
            self.attr_stack
                .at_entry(rela_path, None, objects)
                .map_err(|err| set_resource::Error::Attributes {
                    source: err,
                    kind,
                    rela_path: rela_path.to_owned(),
                })?;
        let mut buf = Vec::new();
        let out = self.filter.convert_to_diffable(
            &id,
            mode,
            rela_path,
            kind,
            &mut |_, out| {
                let _ = entry.matching_attributes(out);
            },
            objects,
            self.filter_mode,
            &mut buf,
        )?;
        let key = storage.clone();
        assert!(
            self.diff_cache
                .insert(
                    key,
                    CacheValue {
                        conversion: out,
                        mode,
                        buffer: buf,
                    },
                )
                .is_none(),
            "The key impl makes clashes impossible with our usage"
        );
        Ok(())
    }