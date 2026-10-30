pub fn reorder_index_aux<A>(arr: &mut [A], index: &mut [usize]) {
    let mut v = Vec::with_capacity(arr.len());
    unsafe {
        std::ptr::copy_nonoverlapping(arr.as_ptr(), v.as_mut_ptr(), arr.len());
        v.set_len(arr.len());
    }

    for (i, (a, index)) in v.drain(..).zip(index.iter_mut()).enumerate() {
        let res = core::mem::replace(&mut arr[*index], a);

        *index = i;
        core::mem::forget(res);
    }
}