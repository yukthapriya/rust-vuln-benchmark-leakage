pub fn set_label_type(&mut self, typ: LabelType) {
        assert!(!self.was_deleted());
        unsafe {
            if typ != LabelType::Multi {
                Fl_Menu_Item_set_label_type(self._inner, typ as i32);
            }
        }
    }