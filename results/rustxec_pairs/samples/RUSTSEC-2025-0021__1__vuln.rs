pub fn verify_checksum(&self) -> Result<gix_hash::ObjectId, (gix_hash::ObjectId, gix_hash::ObjectId)> {
        // Even though we could use gix_features::hash::bytes_of_file(…), this would require using our own
        // Error type to support io::Error and Mismatch. As we only gain progress, there probably isn't much value
        // as these files are usually small enough to process them in less than a second, even for the large ones.
        // But it's possible, once a progress instance is passed.
        let data_len_without_trailer = self.data.len() - self.hash_len;
        let mut hasher = gix_features::hash::hasher(self.object_hash());
        hasher.update(&self.data[..data_len_without_trailer]);
        let actual = gix_hash::ObjectId::from_bytes_or_panic(hasher.digest().as_ref());

        let expected = self.checksum();
        if actual == expected {
            Ok(actual)
        } else {
            Err((actual, expected.into()))
        }
    }