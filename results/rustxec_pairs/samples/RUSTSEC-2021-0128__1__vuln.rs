fn create_scalar_function<'c, F, T>(
        &'c mut self,
        fn_name: &str,
        n_arg: c_int,
        flags: FunctionFlags,
        x_func: F,
    ) -> Result<()>
    where
        F: FnMut(&Context<'_>) -> Result<T> + Send + UnwindSafe + 'c,
        T: ToSql,
    {
        unsafe extern "C" fn call_boxed_closure<F, T>(
            ctx: *mut sqlite3_context,
            argc: c_int,
            argv: *mut *mut sqlite3_value,
        ) where
            F: FnMut(&Context<'_>) -> Result<T>,
            T: ToSql,
        {
            let r = catch_unwind(|| {
                let boxed_f: *mut F = ffi::sqlite3_user_data(ctx) as *mut F;
                assert!(!boxed_f.is_null(), "Internal error - null function pointer");
                let ctx = Context {
                    ctx,
                    args: slice::from_raw_parts(argv, argc as usize),
                };
                (*boxed_f)(&ctx)
            });
            let t = match r {
                Err(_) => {
                    report_error(ctx, &Error::UnwindingPanic);
                    return;
                }
                Ok(r) => r,
            };
            let t = t.as_ref().map(|t| ToSql::to_sql(t));

            match t {
                Ok(Ok(ref value)) => set_result(ctx, value),
                Ok(Err(err)) => report_error(ctx, &err),
                Err(err) => report_error(ctx, err),
            }
        }

        let boxed_f: *mut F = Box::into_raw(Box::new(x_func));
        let c_name = str_to_cstring(fn_name)?;
        let r = unsafe {
            ffi::sqlite3_create_function_v2(
                self.db(),
                c_name.as_ptr(),
                n_arg,
                flags.bits(),
                boxed_f as *mut c_void,
                Some(call_boxed_closure::<F, T>),
                None,
                None,
                Some(free_boxed_value::<F>),
            )
        };
        self.decode_result(r)
    }