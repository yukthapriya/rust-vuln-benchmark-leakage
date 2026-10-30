fn parse_attr(
        &mut self,
        attr: &Attribute,
        variant: Option<&VariantInfo<'_>>,
        binding: Option<&BindingInfo<'_>>,
    ) {
        let meta_list = match attr
            .parse_meta()
            .unwrap_or_else(|e| panic!("error parsing attribute: {:?} ({})", attr, e))
        {
            Meta::List(list) => list,
            _ => return,
        };

        // Ignore any non-zeroize attributes
        if !meta_list.path.is_ident(ZEROIZE_ATTR) {
            return;
        }

        for nested_meta in &meta_list.nested {
            if let NestedMeta::Meta(meta) = nested_meta {
                self.parse_meta(meta, variant, binding);
            } else {
                panic!("malformed #[zeroize] attribute: {:?}", nested_meta);
            }
        }
    }