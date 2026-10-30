pub fn decode(data: &[u8], object_hash: gix_hash::Kind) -> Result<Option<usize>, gix_hash::hasher::Error> {
    let hash_len = object_hash.len_in_bytes();
    if data.len() < MIN_SIZE_WITH_HEADER + hash_len {
        return Ok(None);
    }

    let start_of_eoie = data.len() - MIN_SIZE_WITH_HEADER - hash_len;
    let ext_data = &data[start_of_eoie..data.len() - hash_len];

    let (signature, ext_size, ext_data) = extension::decode::header(ext_data);
    if signature != SIGNATURE || ext_size as usize != MIN_SIZE {
        return Ok(None);
    }

    let (offset, checksum) = ext_data.split_at(4);
    let Ok(checksum) = gix_hash::oid::try_from_bytes(checksum) else {
        return Ok(None);
    };
    let offset = from_be_u32(offset) as usize;
    if offset < header::SIZE || offset > start_of_eoie || checksum.kind() != object_hash {
        return Ok(None);
    }

    let mut hasher = gix_hash::hasher(object_hash);
    let mut last_chunk = None;
    for (signature, chunk) in extension::Iter::new(&data[offset..data.len() - MIN_SIZE_WITH_HEADER - hash_len]) {
        hasher.update(&signature);
        hasher.update(&(chunk.len() as u32).to_be_bytes());
        last_chunk = Some(chunk);
    }

    if hasher.try_finalize()?.verify(checksum).is_err() {
        return Ok(None);
    }
    // The last-to-this chunk ends where ours starts
    if last_chunk.map_or(true, |s| s.as_ptr_range().end != (&data[start_of_eoie]) as *const _) {
        return Ok(None);
    }

    Ok(Some(offset))
}