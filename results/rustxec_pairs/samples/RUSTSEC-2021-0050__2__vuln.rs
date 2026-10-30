pub fn reorder_index_aux<A>(arr:&mut [A],index:&mut [u32]){
    
    
    let mut v=Vec::with_capacity(arr.len());
    unsafe{
        std::ptr::copy_nonoverlapping(arr.as_ptr(), v.as_mut_ptr(), arr.len());
        v.set_len(arr.len());
    }

    for (i,(a,index)) in v.drain(..).zip(index.iter_mut()).enumerate(){
        let res=core::mem::replace(&mut arr[*index as usize],a);

        //arr[*index as usize]=a;
        *index=i as u32;
        core::mem::forget(res);
    }
    
}