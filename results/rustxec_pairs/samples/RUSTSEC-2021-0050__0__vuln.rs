pub fn reorder_index<A>(arr:&mut [A],index:&mut [u32]){

    
    assert_eq!(arr.len(),index.len());


    

    for i in 0..arr.len(){
        while index[i] as usize!=i{
            let old_target_i = index[index[i] as usize];
            

            let mut old_target_e:A=unsafe{std::mem::MaybeUninit::<A>::zeroed().assume_init()};
            core::mem::swap(&mut old_target_e,&mut arr[index[i] as usize]);
            //let old_target_e = arr[index[i] as usize];

            //arr[index[i] as usize] = arr[i];
            let kk:&mut _=unsafe{&mut *(&mut arr[i] as *mut _)};
            core::mem::swap(&mut arr[index[i] as usize],kk);

            index[index[i] as usize]=index[i];

            index[i]=old_target_i;

            //arr[i] = old_target_e;
            core::mem::swap(&mut arr[i],&mut old_target_e);

            core::mem::forget(old_target_e);
        }
    }
}