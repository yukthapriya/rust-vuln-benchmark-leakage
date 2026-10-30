fn push_directory(&mut self, stack: &gix_fs::Stack) -> std::io::Result<()> {
        self.statistics.delegate.push_directory += 1;
        let dir_bstr = gix_path::into_bstr(stack.current());
        let rela_dir_cow = gix_path::to_unix_separators_on_windows(
            gix_glob::search::pattern::strip_base_handle_recompute_basename_pos(
                gix_path::into_bstr(stack.root()).as_ref(),
                dir_bstr.as_ref(),
                None,
                self.case,
            )
            .expect("dir in root")
            .0,
        );
        let rela_dir: &BStr = if rela_dir_cow.starts_with(b"/") {
            rela_dir_cow[1..].as_bstr()
        } else {
            rela_dir_cow.as_ref()
        };
        match &mut self.state {
            #[cfg(feature = "attributes")]
            State::CreateDirectoryAndAttributesStack { attributes, .. } | State::AttributesStack(attributes) => {
                attributes.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    self.objects,
                    &mut self.statistics.attributes,
                )?;
            }
            #[cfg(feature = "attributes")]
            State::AttributesAndIgnoreStack { ignore, attributes } => {
                attributes.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    self.objects,
                    &mut self.statistics.attributes,
                )?;
                ignore.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    self.objects,
                    self.case,
                    &mut self.statistics.ignore,
                )?
            }
            State::IgnoreStack(ignore) => ignore.push_directory(
                stack.root(),
                stack.current(),
                rela_dir,
                self.buf,
                self.id_mappings,
                self.objects,
                self.case,
                &mut self.statistics.ignore,
            )?,
        }
        Ok(())
    }