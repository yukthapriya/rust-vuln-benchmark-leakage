fn parse_meta(
        &mut self,
        meta: &Meta,
        variant: Option<&VariantInfo<'_>>,
        binding: Option<&BindingInfo<'_>>,
    ) {
        if meta.path().is_ident("drop") {
            assert!(!self.drop, "duplicate #[zeroize] drop flags");

            match (variant, binding) {
                (_variant, Some(_binding)) => {
                    // structs don't have a variant prefix, and only structs have bindings outside of a variant
                    let item_kind = match variant.and_then(|variant| variant.prefix) {
                        Some(_) => "enum",
                        None => "struct",
                    };
                    panic!(
                        concat!(
                            "The #[zeroize(drop)] attribute is not allowed on {} fields. ",
                            "Use it on the containing {} instead.",
                        ),
                        item_kind, item_kind,
                    )
                }
                (Some(_variant), None) => panic!(concat!(
                    "The #[zeroize(drop)] attribute is not allowed on enum variants. ",
                    "Use it on the containing enum instead.",
                )),
                (None, None) => (),
            };

            self.drop = true;
        } else {
            panic!("unknown #[zeroize] attribute type: {:?}", meta.path());
        }
    }