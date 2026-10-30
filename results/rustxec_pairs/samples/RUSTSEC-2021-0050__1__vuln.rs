pub fn reorder_index_aux_vec<A>(mut arr:Vec<A>,index:&mut [u32])->Vec<A>{
    
    
    let mut v=Vec::with_capacity(arr.len());
    unsafe{v.set_len(arr.len())};
    let vv=unsafe{std::slice::from_raw_parts_mut(v.as_mut_ptr(),v.len())};


    for (i,(a,index)) in arr.drain(..).zip(index.iter_mut()).enumerate(){
        vv[*index as usize]=a;
        *index=i as u32
    }

    v
    
}