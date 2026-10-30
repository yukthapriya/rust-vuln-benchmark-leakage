pub fn wait_for(dur: f64) -> Result<bool, FltkError> {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        let temp = Fl_wait_for(dur); 
        if temp == 0.0 {
            Ok(false)
        } else if temp == 1.0 {
            Ok(true) 
        } else {
            Err(FltkError::Unknown(String::from(
                "An unknown error occured!")))
        }
    }
}