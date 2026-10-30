pub fn reorder_index<A>(arr: &mut [A], indices: &mut [usize]) {
    assert_eq!(arr.len(), indices.len());

    let arr: &mut [MaybeUninit<_>] = unsafe { arr.manually_drop_mut().as_out().as_mut_uninit() };
    for i in 0..arr.len() {
        while indices[i] != i {
            let old_target_i = indices[indices[i]];

            //let old_target_e = arr[index[i]];
            let mut old_target_e = std::mem::MaybeUninit::<A>::uninit();
            unsafe {
                old_target_e
                    .as_mut_ptr()
                    .write(arr[indices[i]].as_ptr().read());
            }

            // arr[index[i]] = arr[i];
            // Since the object arr[index[i]] is aliased to old_target_e, we cannot produce
            // mutable reference to it nor assign to it.
            unsafe {
                arr[indices[i]].as_mut_ptr().write(arr[i].as_ptr().read());
            }
            // Now that old_target_e is not longer aliased, we can call assume_init
            let old_target_e = unsafe { old_target_e.assume_init() };

            indices[indices[i]] = indices[i];

            indices[i] = old_target_i;

            //arr[i] = old_target_e;
            // arr[i] is aliased to arr[index[i]
            arr[i].as_out().write(old_target_e);
        }
    }
}