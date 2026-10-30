pub fn merge<M: Memory>(
        &mut self,
        mut source: Node<K>,
        median: Entry<K>,
        allocator: &mut Allocator<M>,
    ) {
        // Load all the values from the source node first, as they will be moved out.
        for i in 0..source.entries_len() {
            source.value(i, allocator.memory());
        }

        if source.key(0) > self.key(0) {
            // The source node has keys that are greater than self.
            // Append the source node into self.
            Self::append(self, &mut source, median);
        } else {
            // self has keys that are greater than the source node.
            // Append self into the source node (which more efficient).
            Self::append(&mut source, self, median);

            // Move the entries and children into self.
            self.keys = core::mem::take(&mut source.keys);
            self.encoded_values = core::mem::take(&mut source.encoded_values);
            self.children = core::mem::take(&mut source.children);
        }

        self.save(allocator);
        source.deallocate(allocator);
    }