// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0003 (FIXED)
// Crate: protobuf
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: stepancheg/rust-protobuf
// Commit: f06992f
// File: protobuf/src/coded_input_stream/mod.rs


}
 
     fn skip_group(&mut self) -> crate::Result<()> {
        self.incr_recursion()?;
        let ret = self.skip_group_no_depth_check();
        self.decr_recursion();
        ret
    }

    fn skip_group_no_depth_check(&mut self) -> crate::Result<()> {
         while !self.eof()? {
             let wire_type = self.read_tag_unpack()?.1;
             if wire_type == WireType::EndGroup {
     /// Read message, do not check if message is initialized
     pub fn merge_message<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
         self.incr_recursion()?;
        let ret = self.merge_message_no_depth_check(message);
        self.decr_recursion();
        ret
    }
 
    fn merge_message_no_depth_check<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        message.merge_from(self)?;
        self.pop_limit(old_limit);
         Ok(())
     }
 
         );
         assert_eq!("field 3", input.read_string().unwrap());
     }

    #[test]
    fn test_shallow_nested_unknown_groups() {
        // Test skip_group() succeeds on a start group tag 50 times
        // followed by end group tag 50 times. We should be able to
        // successfully skip the outermost group.
        let mut vec = Vec::new();
        let mut os = CodedOutputStream::new(&mut vec);
        for _ in 0..50 {
            os.write_tag(1, WireType::StartGroup).unwrap();
        }
        for _ in 0..50 {
            os.write_tag(1, WireType::EndGroup).unwrap();
        }
        drop(os);

        let mut input = CodedInputStream::from_bytes(&vec);
        assert!(input.skip_group().is_ok());
    }

    #[test]
    fn test_deeply_nested_unknown_groups() {
        // Create an output stream that has groups nested recursively 1000
        // deep, and try to skip the group.
        // This should fail the default depth limit of 100 which ensures we
        // don't blow the stack on adversial input.
        let mut vec = Vec::new();
        let mut os = CodedOutputStream::new(&mut vec);
        for _ in 0..1000 {
            os.write_tag(1, WireType::StartGroup).unwrap();
        }
        for _ in 0..1000 {
            os.write_tag(1, WireType::EndGroup).unwrap();
        }
        drop(os);

        let mut input = CodedInputStream::from_bytes(&vec);
        assert!(input
            .skip_group()
            .unwrap_err()
            .to_string()
            .contains("Over recursion limit"));
    }
 }