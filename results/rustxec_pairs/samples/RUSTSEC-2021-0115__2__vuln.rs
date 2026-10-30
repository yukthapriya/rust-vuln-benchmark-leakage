fn parse_meta(&mut self, meta: &Meta) {
        if meta.path().is_ident("drop") {
            assert!(!self.drop, "duplicate #[zeroize] drop flags");
            self.drop = true;
        } else {
            panic!("unknown #[zeroize] attribute type: {:?}", meta.path());
        }
    }