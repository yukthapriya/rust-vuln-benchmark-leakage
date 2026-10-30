pub fn wait_for(dur: f64) -> Result<bool, FltkError> {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        match Fl_wait_for(dur) {
            0.0 => Ok(false),
            1.0 => Ok(true),
            _ => Err(FltkError::Unknown(String::from(
                "An unknown error occured!"))),
        }
    }
}