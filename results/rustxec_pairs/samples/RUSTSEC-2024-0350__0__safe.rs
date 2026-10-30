fn push_directory(&mut self, stack: &gix_fs::Stack) -> std::io::Result<()> {
        self.statistics.delegate.push_directory += 1;
        let rela_dir_bstr = gix_path::into_bstr(stack.current_relative());
        let rela_dir = gix_path::to_unix_separators_on_windows(rela_dir_bstr);
        match &mut self.state {
            #[cfg(feature = "attributes")]
            State::CreateDirectoryAndAttributesStack { attributes, .. } | State::AttributesStack(attributes) => {
                attributes.push_directory(
                    stack.root(),
                    stack.current(),
                    &rela_dir,
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
                    &rela_dir,
                    self.buf,
                    self.id_mappings,
                    self.objects,
                    &mut self.statistics.attributes,
                )?;
                ignore.push_directory(
                    stack.root(),
                    stack.current(),
                    &rela_dir,
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
                &rela_dir,
                self.buf,
                self.id_mappings,
                self.objects,
                self.case,
                &mut self.statistics.ignore,
            )?,
        }
        Ok(())
    }